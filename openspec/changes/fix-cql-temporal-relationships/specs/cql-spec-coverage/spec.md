## ADDED Requirements

### Requirement: Temporal relationship coverage claims are evidence-backed

Coverage documents SHALL mark a temporal relationship form implemented only when parser, canonical
ELM, and supported evaluation evidence exists for that form.

#### Scenario: Closed temporal rows cite implementation and tests

- **WHEN** zero-offset temporal rows are updated in `SPEC_COVERAGE.md`
- **THEN** their notes cite the parser/emitter implementation and validating focused or HL7 tests

#### Scenario: Conformance delta records its baseline

- **WHEN** temporal conformance totals are refreshed
- **THEN** `CONFORMANCE.md` records the run date, command, baseline, post-change totals, and deltas

#### Scenario: Partial behavior remains explicit

- **WHEN** any offset, boundary, precision, or operand behavior remains unsupported
- **THEN** the corresponding coverage stage remains partial or unsupported rather than being
  implied complete
