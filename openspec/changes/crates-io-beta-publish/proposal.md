## Why

The rh monorepo contains 7 library crates and 1 CLI binary that are functionally complete enough for early adopters, but none can be installed via `cargo install` or used as dependencies from crates.io. Workspace metadata has placeholder values, crate naming is inconsistent, path dependencies lack version specifiers, and no beta versioning strategy exists. Publishing a beta release unblocks external consumption and validates the packaging pipeline before a stable 1.0.

## What Changes

- **BREAKING**: Rename crate `hl7_fhir_r4_core` → `rh-hl7-fhir-r4-core` to establish consistent `rh-` prefix naming and avoid implying official HL7 provenance
- **BREAKING**: Rename CLI crate `rh` → `rh-cli` (binary stays `rh` via `[[bin]]` table)
- Fix workspace `[workspace.package]` metadata: replace placeholder `authors`, `repository` with real values; add `homepage`
- Normalize all crate Cargo.toml files to use workspace metadata inheritance where possible
- Add `keywords` and `categories` to crates missing them
- Add `readme` field pointing to each crate's README.md
- Set all versions to `0.1.0-beta.1` (rh-validator to `0.2.0-beta.1`)
- Add `version` specifiers alongside `path` for all workspace-internal dependencies (required by crates.io)
- Feature-gate `cdylib` crate-type in `rh-fhirpath` and `rh-vcl` behind a `wasm` feature so crates.io publishes only `rlib`
- **Standardize WASM build approach** across `rh-fhirpath` and `rh-vcl`: uniform `Cargo.toml` structure, consistent justfile recipes, shared `wasm-pack` release profile, and a workspace-level `just wasm` recipe

## Capabilities

### New Capabilities
- `crates-io-packaging`: Workspace metadata, version strategy, dependency version specifiers, publish ordering, and crate naming conventions required for crates.io distribution

### Modified Capabilities
- `wasm-build`: Uniform WASM build tooling across `rh-fhirpath` and `rh-vcl` — consistent Cargo.toml layout, identical justfile structure, shared `wasm-pack` profile, and workspace-level coordination

## Impact

- **All 8 Cargo.toml files** (workspace root + 7 crates + 1 CLI app) will be modified
- **All `use hl7_fhir_r4_core`** imports in source code must be updated to `rh_hl7_fhir_r4_core` (Rust normalizes hyphens to underscores)
- **Dependency declarations** in every crate that depends on `hl7_fhir_r4_core` must update the name
- **CI/build scripts** referencing the old crate name or old package name `rh` need updating
- **`rh-vcl/WASM_BUILD.md`** references "Makefile" but the build file is a justfile — docs need updating
- **Publish order** is constrained by the dependency graph: rh-foundation → (rh-hl7-fhir-r4-core, rh-codegen, rh-cql) → (rh-fhirpath, rh-vcl) → rh-validator → rh-cli
