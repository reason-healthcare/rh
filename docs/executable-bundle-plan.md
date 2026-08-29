# Executable Bundle Support for rh-packager

> **Goal:** Make `rh package` dual-purpose: (1) build conformant FHIR Packages for
> registry distribution, and (2) build self-contained executable bundles for WASM
> evaluation. The executable bundle is a FHIR Bundle where every ValueSet is
> pre-expanded, every StructureDefinition is snapshotted, every CQL library is
> compiled to ELM, and every transitive dependency is resolved and included.

---

## Background

The `rh-packager` crate already builds conformant FHIR Packages (`.tgz` + expanded
`package/` directory) from source directories containing FHIR resources, FSH, CQL,
and markdown narrative. It has a hook-processor pipeline (`HookProcessor` trait),
canonical resolution (`lock.rs`), StructureDefinition snapshot generation
(`processors/snapshot.rs`), CQL-to-ELM compilation (`processors/cql.rs`), and FHIR
validation (`processors/validate.rs`).

The CPG preview plan (in `workbench/docs/cpg-preview-plan.md`) requires a
self-contained content bundle that can be passed to a WASM module for client-side
`$apply`, CQL evaluation, and measure evaluation -- with zero external I/O at
runtime. The missing piece is a "linker" step that resolves all transitive
dependencies, pre-expands ValueSets, bundles FHIRHelpers, validates completeness,
and outputs a single self-contained FHIR Bundle.

This plan adds that capability to the existing `rh-packager` crate -- no new crate,
no new binary, no new dependencies beyond `uuid`.

---

## CLI Design

The tool is now dual-purpose. The two modes share the same source directory and
`packager.toml` config but produce different outputs:

| Command | Output | Purpose |
|---------|--------|---------|
| `rh package build <dir>` | FHIR package `.tgz` + `package/` directory | Registry distribution, IG publishing |
| `rh package link <dir>` | Executable bundle JSON (or resource directory) | WASM evaluation, CPG preview, client-side CQL |

### `rh package link`

```
rh package link [OPTIONS] <DIR>

Arguments:
  <DIR>                    Source directory containing packager.toml and FHIR resources

Options:
  -o, --out <PATH>         Output directory (default: <DIR>/executable)
  --format <FORMAT>        Output format: "bundle" (single FHIR Bundle JSON) or
                           "directory" (one .json file per resource) [default: from
                           packager.toml, or "bundle"]
  --terminology <URL>      Override terminology server URL from packager.toml
  --terminology-dir <PATH> Override terminology directory from packager.toml
  --no-validate            Skip link-validate completeness check
  --verbose                Show resolution trace for each dependency
```

CLI flags override `packager.toml` `[link]` section values when present.

### Relationship between `build` and `link`

```
                  +-----------------------------+
                  |       Source Directory      |
                  |  packager.toml, IG, FSH, CQL|
                  +----------+--------+---------+
                             |        |
                    build ---+        +--- link
                             |                 |
                    +--------v-----+   +-------v-----------+
                    |  before_build|   |  before_build      |
                    |  (fsh, cql,  |   |  (fsh, cql,        |
                    |   snapshot,  |   |   snapshot,        |
                    |   validate)  |   |   validate)        |
                    +--------------+   +--------------------+
                    |  narrative   |   |  narrative         |
                    |  ig_sync     |   |  ig_sync           |
                    |  pinning     |   |  pinning           |
                    +--------------+   +--------------------+
                    |  after_build |   |  after_build       |
                    |  (user-config|   |  + resolve-deps    |
                    |              |   |  + expand-valuesets|
                    +--------------+   +--------------------+
                    |  write pkg/  |   |  link-validate     |
                    |  create .tgz |   |  write executable  |
                    +--------------+   |    bundle          |
                                       +--------------------+
```

Both commands share the same `before_build` stage (same processors, same config).
`link` adds `resolve-dependencies` and `expand-valuesets` to `after_build`, runs
`link-validate`, and writes a different output format. The shared prefix means a
source directory that passes `rh package build` will also pass the early stages
of `rh package link` -- failures in `link` are specifically about executability
(unresolved deps, unexpanded ValueSets), not about basic FHIR conformance.

---

## `packager.toml` `[link]` Configuration

New config section, added to `PublisherConfig`:

```toml
[link]
# FHIR terminology server URL for ValueSet $expand.
# Used by the expand-valuesets processor.
terminology_server = "https://tx.fhir.org/r4"

# Local directory of pre-expanded ValueSets and CodeSystems.
# Alternative to terminology_server -- fully offline.
# terminology_dir = "/path/to/terminology-snapshots"

# Path to FHIRHelpers.cql or pre-compiled FHIRHelpers ELM JSON.
# Defaults to a version bundled with rh-packager.
# fhir_helpers = "/path/to/FHIRHelpers.cql"

# Output format for `rh package link`.
# "bundle" = single FHIR Bundle JSON (default)
# "directory" = one .json file per resource + _manifest.json
format = "bundle"

# Override packages_dir for dependency resolution.
# Falls back to top-level packages_dir or ~/.fhir/packages.
# packages_dir = "/custom/.fhir/packages"
```

| Field | Type | Default | Description |
|---|---|---|---|
| `terminology_server` | string | -- | FHIR terminology server base URL for `$expand`. |
| `terminology_dir` | string | -- | Local directory of pre-expanded ValueSets and CodeSystems. |
| `fhir_helpers` | string | bundled | Path to FHIRHelpers source or pre-compiled ELM. |
| `format` | string | `"bundle"` | Output format: `"bundle"` or `"directory"`. |
| `packages_dir` | string | top-level | Packages cache for dependency resolution. |

---

## New Hook Processors

Three new processors, registered alongside the existing `snapshot`, `validate`,
`cql`, and `fsh` processors.

### `resolve-dependencies` (processors/resolve_deps.rs)

Pulls in transitive dependencies from installed FHIR packages so the resource
set is self-contained.

**Algorithm:**
1. Collect all canonical references from all resources using the existing
   `walk_canonical_fields` + `collect_canonicals` logic from `lock.rs`.
2. For each canonical URL not already in `ctx.resources`, search dependency
   packages (using the existing `search_package_for_canonical` pattern from
   `lock.rs`).
3. Load the found resource into `ctx.resources` using the standard
   `<ResourceType>-<id>` key.
4. Repeat -- the newly added resource may itself reference other canonicals.
   Continue until no new unresolved references are found (fixpoint).
5. If any canonical cannot be resolved, fail with `PublisherError::MissingCanonical`.

**Typical stage:** `after_build` (run before `expand-valuesets` so that
ValueSets from dependency packages are available for expansion).

**Config:** Uses `[link] packages_dir` (falls back to top-level
`packages_dir` or `~/.fhir/packages`).

### `expand-valuesets` (processors/expand.rs)

Pre-expands all ValueSets that have `compose` but no `expansion` (or an empty
expansion).

**Algorithm:**
1. Collect all ValueSet resources in `ctx.resources` that need expansion.
2. For each, resolve the expansion:
   - If `terminology_dir` is configured, look for a pre-expanded ValueSet
     file matching the URL.
   - If `terminology_server` is configured, call `$expand` via
     `rh-foundation` HTTP client.
   - If the ValueSet already has `expansion.contains`, skip it.
3. For ValueSets that compose from other ValueSets (`compose.include.valueSet`),
   expand referenced ValueSets first (transitive).
4. Store the full expansion in `ValueSet.expansion.contains`.
5. If a ValueSet cannot be expanded, fail with `PublisherError::Other`.

**Typical stage:** `after_build` (run after `resolve-dependencies`).

**Config:** Uses `[link] terminology_server` and `[link] terminology_dir`.

### `link-validate` (processors/link_validate.rs)

Verifies the resource set is self-contained and executable. This is the
completeness gate -- if it fails, the bundle is not executable.

**Checks:**
1. **No dangling canonical references** -- every `instantiatesCanonical`,
   `definitionCanonical`, `library`, `relatedArtifact.url`, `answerValueSet`,
   `valueSet` reference resolves to a resource in `ctx.resources`.
2. **All ValueSets are expanded** -- every ValueSet has `expansion.contains`
   with at least one entry (or is explicitly empty by design).
3. **All StructureDefinitions are snapshotted** -- every SD has
   `snapshot.element`.
4. **All Libraries have ELM** -- every Library that will be evaluated has an
   `application/elm+json` attachment in `content[]`.
5. **FHIRHelpers is present** -- if any Library's CQL imports FHIRHelpers,
   a FHIRHelpers Library resource is in `ctx.resources`.
6. **No circular Library dependencies** -- detect and report cycles in
   Library `relatedArtifact` (type: depends-on) references.

On failure, returns `PublisherError::LinkValidation(Vec<String>)` with one
message per failing check.

**Typical stage:** `after_build` (run last, after `resolve-dependencies` and
`expand-valuesets`).

**Config:** None -- reads from `ctx.resources` directly.

---

## Output Format

### Bundle format (default)

A single FHIR `Bundle` JSON file written to
`<output_dir>/executable-bundle.json`:

```json
{
  "resourceType": "Bundle",
  "type": "collection",
  "meta": {
    "tag": [{
      "system": "https://reasonhealth.com/fhir/CodeSystem/bundle-type",
      "code": "executable",
      "display": "Executable Bundle"
    }]
  },
  "extension": [{
    "url": "https://reasonhealth.com/fhir/StructureDefinition/executable-bundle-metadata",
    "extension": [
      { "url": "linkedAt", "valueDateTime": "2026-08-29T12:00:00Z" },
      { "url": "linkerVersion", "valueString": "rh 0.2.8" },
      { "url": "resourceCount", "valueInteger": 47 },
      { "url": "validationPassed", "valueBoolean": true }
    ]
  }],
  "entry": [
    { "fullUrl": "urn:uuid:...", "resource": { } },
    {}
  ]
}
```

### Directory format

One `.json` file per resource, plus a `_manifest.json`:

```
executable/
  _manifest.json
  PlanDefinition-crs-surgical-management.json
  Library-crs-logic.json
  Library-FHIRHelpers.json
  ValueSet-hypertension-codes.json
  CodeSystem-snomed-ct.json
```

---

## Implementation Phases

### Phase 1: Core Processors and Config

**Files created:**
- `crates/rh-packager/src/processors/resolve_deps.rs`
- `crates/rh-packager/src/processors/expand.rs`
- `crates/rh-packager/src/processors/link_validate.rs`

**Files modified:**
- `crates/rh-packager/src/config.rs` -- Add `LinkConfig` struct and `link` field on `PublisherConfig`
- `crates/rh-packager/src/error.rs` -- Add `MissingCanonical(String)` and `LinkValidation(Vec<String>)` variants
- `crates/rh-packager/src/processors/mod.rs` -- Add `pub mod resolve_deps; pub mod expand; pub mod link_validate;`
- `crates/rh-packager/src/hooks.rs` -- Register three new processors in `build_registry()`

**Testing:** Unit tests in each new processor file, following the existing test
patterns (tempdir-based, `make_ctx` helper, assert on resource map mutations).

**Acceptance:**
- `resolve-dependencies` processor pulls a ValueSet from a fake dependency package into the resource map.
- `expand-valuesets` processor populates `expansion.contains` from a local terminology directory.
- `link-validate` processor fails when a canonical reference is unresolved.
- All existing tests continue to pass.

### Phase 2: Pipeline, Output, and CLI

**Files modified:**
- `crates/rh-packager/src/pack.rs` -- Add `write_executable_bundle()` and `write_executable_directory()`
- `crates/rh-packager/src/pipeline.rs` -- Add `link()` function
- `crates/rh-packager/src/lib.rs` -- Re-export `link as link_package`
- `crates/rh-packager/Cargo.toml` -- Add `uuid = { version = "1", features = ["v4"] }`
- `apps/rh-cli/src/package.rs` -- Add `Link(LinkArgs)` variant and handler

**Testing:**
- Integration test in `crates/rh-packager/tests/` that runs `link()` on a fixture
  package and verifies the output bundle contains all expected resources.
- CLI smoke test: `rh package link` on the existing test fixture produces an
  executable bundle file.

**Acceptance:**
- `rh package link <dir>` produces `executable-bundle.json` in the output directory.
- The bundle contains all resources from the source directory plus any resolved
  dependencies.
- `--format directory` produces one file per resource plus `_manifest.json`.
- CLI flags (`--terminology`, `--terminology-dir`, `--format`, `--no-validate`)
  override `packager.toml` values.
- `--no-validate` skips the `link-validate` processor.
- `--verbose` shows resolution trace output.

### Phase 3: Documentation

**Files modified in `rh/`:**

1. **`crates/rh-packager/README.md`**
   - Add "Dual Purpose" section at the top explaining `build` vs `link`.
   - Add `[link]` section to the `packager.toml Reference`.
   - Add new processors to the "Built-in processors" table.
   - Add "Executable Bundle Output" section describing the output format.
   - Add `link` to the pipeline stage diagram.

2. **`crates/rh-packager/PROCESSORS.md`**
   - Add entries for `resolve-dependencies`, `expand-valuesets`, `link-validate`
     in the "Built-in Processors" section (Section 3).
   - Document each processor's behavior, config, and typical stage.
   - Note the ordering constraint: `resolve-dependencies` before
     `expand-valuesets` before `link-validate`.

3. **`apps/rh-cli/docs/PACKAGER.md`**
   - Add "Dual Purpose" framing to the Overview section.
   - Add `rh package link` command section with full usage, arguments, options,
     and examples.
   - Add "Executable Bundle Layout" section showing the output format.
   - Update "Related Documentation" links if needed.
   - Add a "Step-by-Step: Building an Executable Bundle" guide section that
     shows the `link` workflow alongside the existing `build` walkthrough.

4. **`CHANGELOG.md`**
   - Add entry under `## Unreleased`:
     ```
     - `rh package link` -- new command to build self-contained executable
       bundles for WASM evaluation. Pre-expands ValueSets, resolves transitive
       dependencies, validates completeness.
     ```

5. **`README.md`** (top-level)
   - Add `rh package link` to the CLI command summary table if one exists.
   - Add a brief mention of executable bundle support.

**Files modified in `workbench/`:**

6. **`docs/cpg-preview-plan.md`**
   - Replace references to the hypothetical `rh-linker` crate with
     `rh package link`.
   - Update Section 2.7 (WASM bindings) to note that the content bundle is
     produced by `rh package link`.
   - Update Section 3, Phase 3 (Workbench integration) to reference
     `executable-bundle.json` produced by `rh package link`.
   - Update the snapshot manifest extension example to use the
     `executable` field.
   - Remove or replace the "new crate: rh-linker" proposal in Section 2.1
     with a note that this functionality is now part of `rh-packager`.

7. **`docs/build-a-snapshot.md`**
   - Add a step between the existing package build and snapshot packaging:
     ```
     # 2. Link the executable bundle (if L3 content is present)
     rh package link \
       topics/<topic>/process/package-workspace/output/package \
       --out topics/<topic>/process/package-workspace/executable
     ```
   - Update the snapshot directory layout to include `executable/`.
   - Update the manifest example to show the `executable` field.

8. **`docs/architecture-plan.md`**
   - Add a note in the Runtime Architecture section that L3 content is
     pre-processed into an executable bundle via `rh package link` before
     being included in snapshots.
   - Update the "Core Modules" -> "Validation" section to mention that
     `link-validate` evidence can be stored as a `ValidationRun`.

**Acceptance:**
- All documentation accurately describes the implemented CLI, config, and output.
- No references to the hypothetical `rh-linker` crate remain.
- The `packager.toml` reference in README.md includes the `[link]` section.
- The CLI docs include `rh package link` with correct options.
- The workbench docs reference `rh package link` as the L3 pre-processing step.

### Phase 4: Integration Tests and Fixtures

**Files created:**
- `crates/rh-packager/tests/executable_bundle_test.rs` -- Integration test
- `crates/rh-packager/tests/fixtures/executable-package/` -- Test fixture

**Fixture contents:**
- `packager.toml` with `[link]` section
- `input/ImplementationGuide.json`
- `input/PlanDefinition-test.json` referencing a Library and ValueSet
- `input/Library-TestLogic.json` with CQL source
- `input/cql/TestLogic.cql` importing FHIRHelpers
- `input/ValueSet-test-codes.json` with compose (not expanded)
- A fake dependency package directory with a CodeSystem

**Test cases:**
1. `link()` produces a bundle with all source resources.
2. `link()` resolves the ValueSet from the dependency package.
3. `link()` expands the ValueSet (using terminology-dir fixture).
4. `link()` compiles CQL to ELM (existing `cql` processor).
5. `link()` generates SD snapshots (existing `snapshot` processor).
6. `link-validate` passes when all deps are resolved.
7. `link-validate` fails when a dependency is missing.
8. `--format directory` produces separate files.
9. `--format bundle` produces a single JSON file.
10. `--no-validate` skips validation (produces bundle even with missing deps).

**Acceptance:**
- All 10 test cases pass.
- `cargo test -p rh-packager` passes.
- `cargo clippy -p rh-packager` passes.
- `cargo test -p rh-cli` passes (if CLI tests are added).

---

## Dependency Changes

| Crate | Change | Reason |
|-------|--------|--------|
| `rh-packager` | Add `uuid = { version = "1", features = ["v4"] }` | Generate `urn:uuid:` fullUrl entries in bundle output |

No other dependency changes. All existing dependencies (`rh-cql`, `rh-fhirpath`,
`rh-hl7-fhir-r4-core`, `rh-foundation`, `rh-validator`, `rh-fsh`, `serde`,
`serde_json`, `anyhow`, `thiserror`, `tracing`, `chrono`, `tar`, `flate2`)
are already in `Cargo.toml` and sufficient for the new functionality.

---

## File Change Summary

### New files (7)

| File | Purpose |
|------|---------|
| `docs/executable-bundle-plan.md` | This plan document |
| `crates/rh-packager/src/processors/resolve_deps.rs` | Transitive dependency resolution processor |
| `crates/rh-packager/src/processors/expand.rs` | ValueSet pre-expansion processor |
| `crates/rh-packager/src/processors/link_validate.rs` | Completeness validation processor |
| `crates/rh-packager/tests/executable_bundle_test.rs` | Integration tests |
| `crates/rh-packager/tests/fixtures/executable-package/packager.toml` | Test fixture config |
| `crates/rh-packager/tests/fixtures/executable-package/input/*` | Test fixture resources |

### Modified files (14)

| File | Changes |
|------|---------|
| `crates/rh-packager/src/config.rs` | Add `LinkConfig` struct, `link` field on `PublisherConfig` |
| `crates/rh-packager/src/error.rs` | Add `MissingCanonical`, `LinkValidation` error variants |
| `crates/rh-packager/src/processors/mod.rs` | Add `resolve_deps`, `expand`, `link_validate` module declarations |
| `crates/rh-packager/src/hooks.rs` | Register 3 new processors in `build_registry()` |
| `crates/rh-packager/src/pack.rs` | Add `write_executable_bundle()`, `write_executable_directory()` |
| `crates/rh-packager/src/pipeline.rs` | Add `link()` function |
| `crates/rh-packager/src/lib.rs` | Re-export `link as link_package` |
| `crates/rh-packager/Cargo.toml` | Add `uuid` dependency |
| `apps/rh-cli/src/package.rs` | Add `Link(LinkArgs)` variant and handler |
| `crates/rh-packager/README.md` | Add `[link]` config docs, new processors, executable bundle output, dual-purpose framing |
| `crates/rh-packager/PROCESSORS.md` | Document 3 new processors |
| `apps/rh-cli/docs/PACKAGER.md` | Add `rh package link` command docs, executable bundle layout, dual-purpose framing |
| `CHANGELOG.md` | Add changelog entry |
| `README.md` | Add `rh package link` to CLI command summary |

### Workbench files modified (3)

| File | Changes |
|------|---------|
| `workbench/docs/cpg-preview-plan.md` | Replace `rh-linker` references with `rh package link` |
| `workbench/docs/build-a-snapshot.md` | Add `rh package link` step to snapshot build process |
| `workbench/docs/architecture-plan.md` | Note executable bundle as L3 pre-processing step |

---

## What This Unblocks

| Currently blocked | How `rh package link` unblocks it |
|---|---|
| WASM CQL evaluation | All ValueSets pre-expanded -- `InMemoryTerminologyProvider` works with no I/O |
| WASM `$apply` | All dependencies resolved -- `BundleResolver` finds everything in the bundle |
| CQL FHIRHelpers | Bundled as pre-compiled ELM -- eval engine loads it as a dependency library |
| Questionnaire rendering | SDs snapshotted -- renderer reads `snapshot.element` directly |
| Deterministic preview | Bundle is self-contained -- same bundle always produces same evaluation results |
| Offline preview | No terminology server needed at runtime -- works in browser with no network |
| Validation evidence | `link-validate` report -- stored as a `ValidationRun` in the workbench DB |
