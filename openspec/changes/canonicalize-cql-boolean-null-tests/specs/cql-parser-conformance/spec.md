## ADDED Requirements

### Requirement: Parser lowers null and Boolean tests canonically

The parser SHALL lower `is null`, `is true`, and `is false` to `IsNull`, `IsTrue`, and `IsFalse`
unary AST operations instead of generic named-type tests.

#### Scenario: Positive forms produce dedicated operations

- **WHEN** `A is null`, `A is true`, and `A is false` are parsed
- **THEN** their AST nodes use `IsNull`, `IsTrue`, and `IsFalse`, respectively

#### Scenario: Negative forms produce explicit negation

- **WHEN** `A is not null`, `A is not true`, and `A is not false` are parsed
- **THEN** each AST is the canonical negated form of its corresponding dedicated positive test

### Requirement: Literal tests do not replace generic type tests

The dedicated null and Boolean syntax SHALL preserve generic `is` and `is not` type-specifier
parsing for named and qualified types.

#### Scenario: Qualified type checks remain generic

- **WHEN** `value is FHIR.Quantity` and `value is not FHIR.Quantity` are parsed
- **THEN** they remain generic type tests with the complete qualified type specifier

#### Scenario: Medication-request null test is canonical

- **WHEN** the motivating medication-request function is parsed
- **THEN** `medicationRequest.dispenseRequest.validityPeriod is null` produces an `IsNull` AST
  node over the complete nested member expression
