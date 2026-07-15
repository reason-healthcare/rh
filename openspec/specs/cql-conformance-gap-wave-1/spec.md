# cql-conformance-gap-wave-1

## Purpose

Defines the bounded first wave of CQL conformance improvements and its required evidence.

## Requirements

### Requirement: Wave-1 targets documented high-impact CQL gaps

The wave SHALL implement a bounded set of operator-level gaps selected from `SPEC_COVERAGE.md`
and `CONFORMANCE.md`, emphasizing high-impact compile/eval-error contributors and partially
wired operators.

#### Scenario: Scope is derived from documented gaps
- **WHEN** the implementation scope is defined for this wave
- **THEN** every in-scope operator or function SHALL be traceable to a documented gap

#### Scenario: Scope remains bounded
- **WHEN** implementation tasks are executed
- **THEN** features outside this wave SHALL be deferred and documented rather than implicitly added

### Requirement: Wave-1 preserves correctness while improving conformance

The wave MUST preserve zero wrong-answer conformance failures and SHALL reduce
unimplemented-behavior outcomes from the 2026-03-09 baseline.

#### Scenario: No wrong-answer regressions
- **WHEN** the HL7 evaluation suite is run after wave implementation
- **THEN** the wrong-answer `Fail` count SHALL remain `0`

#### Scenario: Error counts improve from baseline
- **WHEN** the HL7 evaluation suite is run after wave implementation
- **THEN** compile-error or eval-error counts SHALL be lower than the 2026-03-09 baseline

### Requirement: Wave-1 changes are test-backed and auditable

Each gap closed by this wave SHALL have automated test evidence and documentation evidence.

#### Scenario: Operator closure has test coverage
- **WHEN** an operator or function is marked closed by this wave
- **THEN** at least one targeted automated test SHALL verify the changed behavior

#### Scenario: Operator closure is documented
- **WHEN** wave implementation is complete
- **THEN** `SPEC_COVERAGE.md` and `CONFORMANCE.md` SHALL reflect the new status and evidence
