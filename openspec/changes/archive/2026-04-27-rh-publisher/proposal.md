## Why

FHIR package authors need a lightweight, reproducible way to assemble conformant FHIR Packages
from a source directory of resources and narrative — without the complexity of a full IGPublisher
pipeline. Today, `rh` can download, validate, and evaluate FHIR content, but has no way to
_produce_ a publishable FHIR Package. `rh publish` fills that gap with a composable, hook-driven
build pipeline scoped strictly to package creation.

## What Changes

- Add `rh publish` subcommand to the `rh` CLI with sub-commands: `build`, `lock`, `check`, `pack`
- Add new `rh-publisher` library crate (`crates/rh-publisher`) containing all build logic
- Introduce `publisher.toml` as the per-project hook configuration file
- Introduce `fhir-lock.json` for canonical version pinning, aligned with IGPublisher pinning guidance
- Source directory layout: flat — `package.json`, `ImplementationGuide.json`, `*.json` FHIR resources, `*.cql` files, `*.md` narrative files
- Output: expanded package directory **and** `.tgz` tarball (FHIR Package Spec layout under `package/`)
- Built-in lifecycle hook processors: `snapshot`, `cql`, `validate` — configured via `publisher.toml`, running sequentially with first-failure abort

## Capabilities

### New Capabilities

- `fhir-package-build`: Core package assembly — reads source directory, processes resources, generates `.index.json`, writes expanded output directory and `.tgz` tarball conforming to the FHIR Package Spec
- `fhir-package-narrative`: Markdown narrative processing — matches `<name>.md` files to `<name>.json` FHIR resources by filename stem, converts markdown to XHTML, embeds as `resource.text`; unmatched `.md` files are included as-is under `package/other/`
- `fhir-package-lock`: Canonical version pinning — scans all source resources for unversioned canonical references, resolves each against loaded dependency packages, writes `fhir-lock.json`; pinning is applied to output resources during build
- `publish-lifecycle-hooks`: Hook runner framework — `publisher.toml` declares named built-in processors at `before_build`, `after_build`, `before_pack`, `after_pack` stages; processors share an in-memory mutable resource map and run sequentially, aborting on first failure
- `fhir-package-validate`: Built-in `validate` hook processor — runs `rh-validator` over all source FHIR resources, loading dependency packages from `~/.fhir/packages/`; configurable via `[validate]` block in `publisher.toml`
- `fhir-snapshot-processor`: Built-in `snapshot` hook processor — runs `rh-foundation` `SnapshotGenerator` over all `StructureDefinition` resources in the source directory, expanding differential to full snapshot in-memory before subsequent steps
- `cql-library-embed`: Built-in `cql` hook processor — finds `*.cql` files, validates and compiles to ELM via `rh-cql`, embeds base64-encoded CQL source and ELM JSON into `Library.content[]` of matching `Library-<name>.json` resources following CQL IG guidance; creates a minimal Library resource if none exists

### Modified Capabilities

## Impact

- **New crate**: `crates/rh-publisher` — depends on `rh-foundation` (http), `rh-validator`, `rh-cql`; adds `pulldown-cmark` as a new workspace dependency for markdown processing
- **`rh-cli`**: new `publish` subcommand wired into `main.rs`; `rh-publisher` added as a dependency
- **Architecture layer**: `rh-publisher` sits at Layer 2 (depends on `rh-foundation`, `rh-validator`, `rh-cql`)
- **No breaking changes** to existing commands or crates
