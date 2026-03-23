## ADDED Requirements

### Requirement: Wave-2 documents baseline and post-change metric deltas
After wave-2 implementation, `SPEC_COVERAGE.md` and `CONFORMANCE.md` SHALL include a dated before/after comparison against the wave-2 baseline (2026-03-09 post-wave-1 metrics).

#### Scenario: Baseline and post-wave-2 values are both present
- **WHEN** a reader reviews updated documents
- **THEN** baseline values, post-wave-2 values, and deltas are all visible for key conformance metrics

#### Scenario: Command evidence and run date are captured
- **WHEN** conformance metrics are updated for wave-2
- **THEN** the documents include the command(s) and run date used to generate the metrics

### Requirement: Wave-2 coverage updates include gap-to-evidence traceability
Each operator/function closed in wave-2 SHALL include traceable references from coverage rows/notes to implementation locations and validating tests.

#### Scenario: Closed wave-2 row references implementation and tests
- **WHEN** a previously open wave-2 gap is marked implemented
- **THEN** coverage notes include implementation and test evidence references

#### Scenario: Deferred items remain explicit after wave-2
- **WHEN** wave-2 completes with remaining out-of-scope gaps
- **THEN** deferred gaps are still explicitly listed and not implied as complete
