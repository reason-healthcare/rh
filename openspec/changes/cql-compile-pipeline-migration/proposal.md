## Why

`rh-cql` completed the three-stage pipeline refactor (`cql-multi-stage-pipeline-refactor`) introducing `SemanticAnalyzer` and `ElmEmitter` as the canonical compilation path. However, the primary `compile()` entry point in `src/compiler.rs` still delegates to the legacy `LibraryBuilder` path, meaning the new pipeline is only exercised through parallel code paths, not through the main API surface. This creates drift risk and maintenance burden — two competing pipelines must be kept in sync, and the new pipeline's correctness can only be validated indirectly.

## What Changes

- Route the primary `compile()` function through the new `SemanticAnalyzer` + `ElmEmitter` three-stage pipeline.
- Validate conformance equivalence between the legacy `LibraryBuilder` path and the new pipeline against the existing test and conformance baseline before switching.
- Remove `builder.rs` and `translator.rs` (and related legacy modules) once equivalence is confirmed and the new path is stable.
- Update internal documentation and design notes to reflect the single canonical pipeline.

## Capabilities

### Modified Capabilities
- `cql-compile-pipeline`: The `compile()` API now routes through the authoritative three-stage pipeline (`syntax → semantics → emit`), eliminating the legacy `LibraryBuilder` delegation.

### Removed Capabilities
- Legacy `builder.rs` / `translator.rs` modules are deleted after equivalence validation. No external API surface changes — only internal routing.

## Impact

- Affected crate: `crates/rh-cql`
- Primary modules: `src/compiler.rs`, `src/builder.rs`, `src/translator.rs`, `src/semantics/`, `src/emit/`
- Affected tests: all `rh-cql` integration and conformance tests serve as the equivalence gate
- Risk: medium — requires careful equivalence validation before deletion of legacy path
- Expected outcome: single authoritative compile path, reduced maintenance surface, improved traceability of compile behavior through the documented pipeline stages
