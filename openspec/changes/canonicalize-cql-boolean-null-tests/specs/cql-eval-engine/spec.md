## ADDED Requirements

### Requirement: Source null and Boolean tests evaluate with CQL semantics

Positive and negated null/Boolean tests compiled from source SHALL evaluate through dedicated
nullological ELM dispatch and SHALL return the CQL-defined Boolean result for true, false, and null
operands.

#### Scenario: Positive identity tests are true

- **WHEN** `true is true`, `false is false`, and `null is null` are compiled and evaluated
- **THEN** each result is `Value::Boolean(true)`

#### Scenario: Mismatched positive tests are false

- **WHEN** supported nonmatching true, false, and null test combinations are evaluated
- **THEN** each result is the CQL-defined Boolean false rather than an unsupported or generic-type
  result

#### Scenario: Negated forms invert their canonical tests

- **WHEN** `is not null`, `is not true`, and `is not false` expressions are evaluated across true,
  false, and null operands
- **THEN** each result matches explicit `not` applied to the corresponding canonical positive test

### Requirement: Legacy generic null-test ELM remains consumable

The evaluator SHALL continue to accept previously generated or external generic `Is` ELM whose
type name is `null` while the compiler stops emitting that shape.

#### Scenario: Legacy ELM compatibility path remains isolated

- **WHEN** legacy generic `Is(null)` ELM is evaluated
- **THEN** it retains its compatible result
- **THEN** compiling equivalent source produces native `IsNull` ELM instead
