## ADDED Requirements

### Requirement: Wave-2 closes documented non-clinical conformance gaps
Wave-2 SHALL implement a bounded set of non-clinical gaps selected from `crates/rh-cql/SPEC_COVERAGE.md` and `crates/rh-cql/CONFORMANCE.md`, including nullological dispatch parity, aggregate completion (`Product`, `GeometricMean`), date/time uncertainty functions (`Precision`, `LowBoundary`, `HighBoundary`, `TimeOfDay`), and list/interval control gaps (`Repeat`, `Size`).

#### Scenario: Scope is traceable to documented gaps
- **WHEN** wave-2 scope is reviewed
- **THEN** each in-scope operator/function maps to an explicit open gap in `SPEC_COVERAGE.md` or `CONFORMANCE.md`

#### Scenario: Scope remains bounded
- **WHEN** implementation planning is completed
- **THEN** deferred clinical/FHIR-navigation features (`AgeIn*`, `Children`, `Descendents`) remain explicitly out of scope for wave-2

### Requirement: Wave-2 preserves correctness and improves conformance outcomes
Wave-2 MUST preserve zero wrong-answer failures and SHALL reduce unimplemented-behavior outcomes relative to the post-wave-1 baseline recorded on 2026-03-09 (`Fail=0`, `Compile err=149`, `Eval err=572`).

#### Scenario: No wrong-answer regressions
- **WHEN** the HL7 evaluation suite is run after wave-2 changes
- **THEN** the wrong-answer `Fail` count remains `0`

#### Scenario: Compile/eval outcomes improve from baseline
- **WHEN** the HL7 evaluation suite is run after wave-2 changes
- **THEN** compile-error and/or eval-error totals are lower than the 2026-03-09 post-wave-1 baseline

### Requirement: Wave-2 closure is test-backed and documented
Each gap closed by wave-2 SHALL include targeted automated test evidence and corresponding status updates in `SPEC_COVERAGE.md` and `CONFORMANCE.md`.

#### Scenario: Closed gap has automated evidence
- **WHEN** an operator/function is marked closed in wave-2
- **THEN** at least one targeted automated test verifies the implemented behavior

#### Scenario: Closed gap is reflected in coverage/conformance docs
- **WHEN** wave-2 implementation is complete
- **THEN** documentation includes updated statuses and dated metric deltas for the wave-2 closure set
