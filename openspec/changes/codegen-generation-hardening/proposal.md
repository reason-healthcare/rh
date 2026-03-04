## Why

`rh-codegen` and generated core outputs still contain known implementation TODOs (manual-default optimization, dynamic StructureDefinition loading gap, and generated `todo!()` placeholders). These are manageable but important hardening tasks that improve generated code quality and runtime flexibility.

## What Changes

- Introduce selective `#[derive(Default)]` generation where semantics are equivalent, while preserving explicit impls where needed.
- Implement dynamic StructureDefinition loading path used by generated trait implementations.
- Eliminate generated `todo!()` method bodies by providing safe defaults or explicit unsupported-operation behavior where applicable.

## Capabilities

### New Capabilities
- `selective-default-derivation`: Generator logic that derives `Default` when valid and falls back to explicit impls otherwise.
- `dynamic-structuredefinition-loading`: Runtime-capable StructureDefinition resolution in generated trait implementation paths.
- `todo-free-generated-bodies`: Generated outputs avoid placeholder panics in production code paths.

### Modified Capabilities
- `codegen-cli-generation-pipeline`: Update codegen pipeline behavior and emitted crate scaffolding to align with hardened generation rules.

## Impact

- Affected crates: `crates/rh-codegen`, generated crate `crates/rh-hl7_fhir_r4_core`, and CLI codegen integration in `apps/rh-cli`.
- Expected outcome: cleaner generated output, fewer runtime placeholders, and better maintainability of generated core types.
