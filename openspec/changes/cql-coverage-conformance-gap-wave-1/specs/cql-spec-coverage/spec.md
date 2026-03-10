## ADDED Requirements

### Requirement: Post-wave coverage and conformance delta reporting
After wave-1 implementation, `SPEC_COVERAGE.md` and `CONFORMANCE.md` SHALL be updated with a dated before/after summary against the 2026-03-09 baseline.

#### Scenario: Baseline and post-wave metrics are both visible
- **WHEN** a reader reviews updated conformance and coverage documents
- **THEN** the documents SHALL include baseline values, post-wave values, and the computed delta for relevant metrics

#### Scenario: Update date and command evidence are present
- **WHEN** metrics are updated
- **THEN** the run date and conformance command(s) used to produce the new metrics SHALL be documented

### Requirement: Gap-to-evidence traceability for closed items
Each operator/function moved from not-implemented or partial status to implemented by wave-1 SHALL include traceable evidence references.

#### Scenario: Closed gap references code and tests
- **WHEN** a previously open gap is marked implemented
- **THEN** the coverage notes SHALL reference implementation location(s) and test artifact(s) that validate closure

#### Scenario: Deferred items remain explicit
- **WHEN** wave-1 closes only part of the known gap backlog
- **THEN** remaining deferred gaps SHALL remain explicitly listed and SHALL NOT be implied as complete
