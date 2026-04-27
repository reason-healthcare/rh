## Why

`rh-validator` has 3 test cases in `invariant_scoping_test.rs` that are intentionally ignored because the FHIRPath invariant evaluator runs with resource-level context only — it does not track which element is currently being validated. Some FHIR invariants are element-scoped (they reference `%context` or navigate from a specific element node, not the resource root), and evaluating them with root context produces incorrect results. These 3 tests represent a known correctness gap, not a missing feature: the invariants themselves are structurally valid but produce wrong answers because context is wrong.

## What Changes

- Extend the FHIRPath evaluator to accept and track per-element context during invariant evaluation.
- Update `validate_invariant()` (and related call sites in the validator rule engine) to pass the current element being validated as the FHIRPath context node, rather than always using the resource root.
- Un-ignore the 3 tests in `invariant_scoping_test.rs` and confirm they pass.
- Add targeted integration tests for nested element constraint resolution to prevent regression.

## Capabilities

### Modified Capabilities
- `fhir-invariant-validation`: Invariants that reference `%context` or rely on element-scoped navigation now evaluate correctly against the element being validated, not the resource root.

## Impact

- Affected crates: `crates/rh-validator`, potentially `crates/rh-fhirpath`
- Primary modules: `rh-validator/src/rules/invariant.rs` (or equivalent), `rh-fhirpath` evaluator context API
- Affected tests: `crates/rh-validator/tests/invariant_scoping_test.rs` (3 currently-ignored tests un-ignored)
- Risk: low-medium — narrow scope; no structural changes to validation pipeline
- Expected outcome: 3 previously-ignored correctness failures resolved; stronger invariant coverage across nested element scenarios
