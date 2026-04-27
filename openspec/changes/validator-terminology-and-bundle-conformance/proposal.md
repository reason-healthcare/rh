## Why

`rh-validator` completed conformance wave-1 (security-checks config and quick wins), bringing the pass rate from 55% to ~72% against the official FHIR test subset. The next highest-leverage conformance targets, as documented in the post-wave-1 analysis, fall into two categories: **terminology/vocabulary validation** and **bundle entry validation**. Together these account for 11 known failing tests with well-understood root causes and bounded fix scope — no architectural changes required.

Terminology gaps (9 tests): the validator lacks CodeSystem supplement support, does not validate against external CodeSystems (CVX, Swedish national CodeSystems), and does not enforce UCUM unit checks or ValueSet code membership for required bindings. Bundle gaps (2 tests): bundle entry ID uniqueness and `fullUrl` reference resolution checks are not enforced.

## What Changes

- Implement CodeSystem supplement support so supplement-defined codes are recognized during terminology validation.
- Add ValueSet expansion membership validation for `required`-strength bindings (so invalid codes that are not in the bound ValueSet produce errors, not silence).
- Add UCUM unit validation for `Quantity.code` and similar unit-bearing elements when a required binding specifies a UCUM-based ValueSet.
- Enhance mock/local terminology service with common external CodeSystem stubs (CVX vaccine codes, UCUM unit codes) for deterministic test coverage.
- Implement `Bundle.entry` ID uniqueness validation (bundle-id-5 case).
- Implement `Bundle.entry.fullUrl` reference consistency checks.

## Capabilities

### New Capabilities
- `codesystem-supplement-support`: Recognize codes defined by CodeSystem supplements during terminology validation.
- `bundle-entry-constraints`: Enforce bundle entry ID uniqueness and `fullUrl` reference resolution rules.

### Modified Capabilities
- `valueset-binding-validation`: Extend required-strength binding validation to enforce ValueSet membership (not just structure).
- `terminology-mock-service`: Expand local terminology data to include CVX and UCUM stubs for deterministic test coverage.

## Impact

- Affected crates: `crates/rh-validator`
- Primary modules: terminology validation, binding validation, bundle rule evaluation
- Affected tests: conformance test suite (targeting the 11 known failures); `crates/rh-validator/tests/binding_validation_test.rs`, `tests/valueset_test.rs`
- Risk: low-medium — terminology changes must not introduce false positives; existing binding behavior at weaker binding strengths must remain unchanged
- Expected outcome: ~10-11% improvement in official FHIR conformance test pass rate (from ~72% toward ~82%)
