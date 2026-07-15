## ADDED Requirements

### Requirement: Source null and Boolean tests emit canonical ELM

The compiler SHALL emit native `IsNull`, `IsTrue`, and `IsFalse` ELM nodes for the corresponding
source syntax and SHALL NOT emit those forms as generic `Is` type tests.

#### Scenario: Positive nullological forms use native nodes

- **WHEN** source expressions using `is null`, `is true`, and `is false` are compiled
- **THEN** structural or golden ELM assertions find `IsNull`, `IsTrue`, and `IsFalse` nodes

#### Scenario: Negative forms wrap canonical positive nodes

- **WHEN** source expressions using each `is not` literal form are compiled
- **THEN** the ELM uses the reference-compatible negated representation around the corresponding
  native positive node

#### Scenario: Named type checks retain generic ELM

- **WHEN** a named or qualified `is` type test is compiled
- **THEN** its ELM remains a generic `Is` expression with the requested type specifier
