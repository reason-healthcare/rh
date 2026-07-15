## ADDED Requirements

### Requirement: Logical precedence claims cite grouping and value evidence

Coverage documentation SHALL mark mixed logical precedence and associativity supported only when
both AST grouping and value-sensitive evaluation tests exist.

#### Scenario: Coverage notes cite mixed-operator regressions

- **WHEN** logical precedence support is documented
- **THEN** the notes cite tests covering every pairing of `and`, `or`, `xor`, and `implies`

#### Scenario: Associativity evidence is explicit

- **WHEN** repeated logical operators are marked supported
- **THEN** the evidence includes left-associative `and`/`or`/`xor` chains and the
  reference-confirmed `implies` behavior

#### Scenario: Conformance results report no wrong-answer regression

- **WHEN** conformance documentation is refreshed after the precedence fix
- **THEN** it records the command and run date and reports a wrong-answer `fail` count of `0`
