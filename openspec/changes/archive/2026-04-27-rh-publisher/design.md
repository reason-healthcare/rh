## Context

`rh` can download, validate, snapshot, and evaluate FHIR content but has no way to _produce_ a
publishable FHIR Package. The building blocks already exist across the workspace:

- `rh-foundation` — tarball extraction, package loading, snapshot generation
- `rh-validator` — profile-based FHIR validation
- `rh-cql` — CQL compilation, ELM emission, library source providers

This change wires those components together into a new `rh-publisher` library crate and exposes
it via a `rh publish` CLI subcommand. The output is a conformant FHIR Package (`.tgz` + expanded
directory) from a flat source directory.

## Goals / Non-Goals

**Goals:**
- Produce FHIR Package Spec-conformant `.tgz` and expanded directory output
- Support markdown narrative embedding into resource `.text`
- Support canonical version pinning via `fhir-lock.json`
- Provide composable lifecycle hooks (`snapshot`, `validate`, `cql`) via `publisher.toml`
- Reuse existing `rh-foundation`, `rh-validator`, and `rh-cql` crates without modification

**Non-Goals:**
- HTML site generation or IG rendering (IGPublisher scope)
- Ballot/publication pipeline (QA reports, cross-version comparison)
- FHIR R5 or multi-version support in this change (R4 only)
- Auto-generating `ImplementationGuide.json` from `package.json` (validate sync only)
- Writing auto-created Library resources back to the source directory

## Decisions

### Decision 1: New `rh-publisher` library crate

**Choice**: New crate at `crates/rh-publisher/`, with `rh-cli` depending on it for the
`publish` subcommand.

**Rationale**: Keeps the CLI thin (argument parsing only). Library code is independently
testable and reusable without the CLI. Consistent with the pattern used by `rh-validator`,
`rh-cql`, etc.

**Alternative considered**: Implement directly in `rh-cli`. Rejected — mixing business logic
into the CLI module makes unit testing harder and breaks the established layering.

---

### Decision 2: `publisher.toml` as separate hook config file

**Choice**: Hook configuration lives in `publisher.toml` in the source directory, separate
from `package.json`.

**Rationale**: `package.json` is a FHIR-spec-defined format — extending it with build-tool
fields would break spec compliance and confuse FHIR tooling that reads it. `publisher.toml` is
build-time config, not runtime metadata. TOML is idiomatic in the Rust ecosystem.

**Alternative considered**: `[publish]` section in `package.json`. Rejected — pollutes the
FHIR package manifest with tooling-specific data.

---

### Decision 3: In-memory mutable resource map

**Choice**: All processors operate on a shared in-memory `HashMap<String, serde_json::Value>`
keyed by filename stem. A single write pass at the end materialises resources to disk.

**Rationale**: Processors can observe and modify each other's output (e.g., `snapshot` expands
a StructureDefinition; `validate` then sees the expanded form). No temp file management.
Simpler ownership model.

**Alternative considered**: Staging directory (each processor reads/writes files). Rejected —
adds I/O overhead, complicates cleanup, and makes processor composition harder to reason about.

---

### Decision 4: Sequential hook execution with first-failure abort

**Choice**: Processors in a hook list run in declaration order. The first error aborts the
pipeline with a non-zero exit code.

**Rationale**: Ordering matters — `snapshot` must run before `validate` to give the validator
complete StructureDefinitions. Fail-fast gives authors a clear signal without scrolling through
cascading errors from downstream processors.

**Alternative considered**: Collect all errors before aborting. Rejected — downstream errors
are often caused by the first failure (e.g., a bad snapshot causes dozens of validation errors).

---

### Decision 5: `pulldown-cmark` for markdown → XHTML

**Choice**: Use `pulldown-cmark` (CommonMark) to convert narrative markdown to HTML, then wrap
in `<div xmlns="http://www.w3.org/1999/xhtml">`.

**Rationale**: Mature, widely used, CommonMark spec compliance is sufficient for FHIR narrative.
Lighter than `comrak` (no GFM extension overhead).

**Alternative considered**: `comrak` (GFM extensions). Not needed — FHIR narrative does not
require GitHub-flavored markdown features such as tables-in-narrative or strikethrough.

---

### Decision 6: Validate IG ↔ `package.json` sync; do not auto-generate

**Choice**: At build time, `rh publish build` checks that `ImplementationGuide.json` fields
are consistent with `package.json`. Mismatches are reported as errors. Authors maintain both
files manually.

**Rationale**: The IG contains many author-controlled fields (`title`, `contact`, `publisher`,
`jurisdiction`, `license`, `definition.resource[]`) that cannot be derived from `package.json`
alone. Auto-generation would silently discard those fields on every build.

**Alternative considered**: Auto-generate IG from `package.json`. Deferred — a future
`rh publish init` command could scaffold a minimal IG from `package.json`.

---

### Decision 7: `fhir-lock.json` as custom JSON (not CRMI Parameters resource)

**Choice**: `fhir-lock.json` is a custom JSON format recording `url → resolvedVersion` mappings
with metadata, aligned structurally with IGPublisher's `qa-versions.json` output.

**Rationale**: A lock file is a build-time artifact for reproducibility, not a runtime
resource. A CRMI Parameters manifest is consumed by terminology servers at runtime — a
different use case. Custom JSON is simpler to read, write, and diff in version control.

**Alternative considered**: CRMI Parameters resource (`crmi-manifestlibrary`). Retained as a
future export option (`rh publish manifest`) for ecosystems that need it.

---

### Decision 8: Dependency packages must be pre-downloaded

**Choice**: `rh publish lock` and processors that load dependencies require packages to already
be present in `~/.fhir/packages/`. A clear error is emitted if a required package is missing.

**Rationale**: Keeps `rh publish` composable — authors run `rh download package` first, then
`rh publish`. No implicit network access during build. Consistent with how `rh snapshot` and
`rh validate` work today.

**Alternative considered**: Auto-download missing deps during build. Rejected — implicit
network access in a build step is surprising and makes builds non-reproducible in offline/CI
environments.

## Risks / Trade-offs

- **[Risk] IG sync validation is fragile** → Start with a small set of required-to-match fields
  (`name`, `version`, `fhirVersions`, `url`); make others warnings rather than errors.
- **[Risk] CQL library name → Library resource filename convention breaks for namespaced CQL**
  → Document the convention clearly; emit a clear error when no match is found and auto-creation
  is needed.
- **[Risk] `pulldown-cmark` HTML output may include elements not valid in FHIR XHTML narrative**
  → Strip or escape unsupported elements (scripts, forms); add a post-processing sanitisation
  step. Mark as a known limitation in docs for this release.
- **[Risk] Large packages with many StructureDefinitions make snapshot generation slow** →
  `rh-foundation` `SnapshotGenerator` already caches loaded definitions; acceptable for initial
  release. Parallelism can be added later.
- **[Trade-off] In-memory resource map holds all resources simultaneously** → For typical FHIR
  packages (tens to low hundreds of resources) this is negligible. Not designed for packages
  with thousands of large resources.

## Migration Plan

This is a new feature with no existing behaviour to migrate. No rollback strategy required.

New `rh publish` subcommand is additive; no existing CLI subcommands are affected.

## Open Questions

1. **FHIR narrative XHTML sanitisation scope**: Which HTML elements produced by `pulldown-cmark`
   need to be stripped to conform to the FHIR narrative XHTML profile? Needs a spike against
   the FHIR spec `xhtml-restricted` profile.
2. **`.index.json` schema version**: The FHIR Package Spec has evolved — confirm which
   `.index.json` version/format is expected by current tooling (packages.fhir.org, Firely SDK,
   HAPI FHIR).
3. **`definition.resource[]` in IG auto-population**: Should `rh publish build` populate
   `ImplementationGuide.definition.resource[]` from discovered resources, or require the author
   to maintain it? Leaning toward auto-population as part of the sync check.
