## ADDED Requirements

### Requirement: Wave-1 targets documented high-impact CQL gaps
The wave SHALL implement a bounded set of operator-level gaps selected from `SPEC_COVERAGE.md` and `CONFORMANCE.md` with emphasis on high-impact compile/eval error contributors and partially wired operators.

#### Scenario: Scope derived from documented gaps
- **WHEN** the implementation scope is defined for this change
- **THEN** every in-scope operator/function SHALL be traceable to an explicit documented gap in `SPEC_COVERAGE.md` or `CONFORMANCE.md`

#### Scenario: Scope stays bounded
- **WHEN** implementation tasks are executed
- **THEN** features not listed in this wave scope SHALL be deferred and documented rather than implicitly expanded into the change

### Requirement: Wave-1 preserves correctness while improving conformance
The wave MUST preserve zero wrong-answer conformance failures and SHALL reduce total unimplemented-behavior outcomes from the 2026-03-09 baseline.

#### Scenario: No wrong-answer regressions
- **WHEN** the HL7 evaluation suite is run after wave implementation
- **THEN** the wrong-answer `Fail` count SHALL remain `0`

#### Scenario: Error counts improve from baseline
- **WHEN** the HL7 evaluation suite is run after wave implementation
- **THEN** compile-error and/or eval-error counts SHALL be lower than the baseline recorded in `CONFORMANCE.md` for 2026-03-09

### Requirement: Wave-1 changes are test-backed and auditable
Each closed gap in this wave SHALL have test evidence and documentation evidence.

#### Scenario: Operator closure has test coverage
- **WHEN** an operator/function is marked closed by this wave
- **THEN** at least one targeted automated test SHALL verify the behavior end-to-end or at the changed stage boundary

#### Scenario: Operator closure is documented
- **WHEN** wave implementation is complete
- **THEN** `SPEC_COVERAGE.md` and `CONFORMANCE.md` SHALL be updated to reflect the new status and reference the evidence-producing tests
