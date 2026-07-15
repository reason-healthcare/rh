## ADDED Requirements

### Requirement: Supported zero-offset temporal relationships evaluate end to end

The compiler and evaluator SHALL execute supported zero-offset inclusive temporal relationships
without parse, emission, dispatch, or semantic errors.

#### Scenario: Bundled inclusive temporal cases match expected values

- **WHEN** the 17 `on or after` and `on or before` expressions in the bundled HL7 interval and
  date/time fixtures are compiled and evaluated
- **THEN** every supported case returns its existing HL7 expected value

#### Scenario: Precision reaches temporal evaluation

- **WHEN** a supported zero-offset temporal relationship includes a precision
- **THEN** evaluation compares operands at that precision

### Requirement: Temporal conformance fixes preserve correctness

The temporal relationship change SHALL NOT turn any previously passing HL7 case into a wrong
answer.

#### Scenario: Conformance invariant is preserved

- **WHEN** the HL7 CQL evaluation suite is run after the change
- **THEN** the wrong-answer `fail` count remains `0`
- **THEN** compile or evaluation error totals improve from the recorded pre-change baseline
