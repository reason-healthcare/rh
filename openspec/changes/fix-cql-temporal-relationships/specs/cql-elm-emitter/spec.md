## ADDED Requirements

### Requirement: Zero-offset inclusive temporal relationships emit canonical ELM

The emitter SHALL translate zero-offset inclusive temporal relationships to canonical
`SameOrBefore` or `SameOrAfter` ELM and SHALL preserve any specified precision.

#### Scenario: On-or-before emits SameOrBefore

- **WHEN** an `OnOrBefore` timing expression with `offset: None` is emitted
- **THEN** the ELM expression is `SameOrBefore` with the original operands and precision

#### Scenario: On-or-after emits SameOrAfter

- **WHEN** an `OnOrAfter` timing expression with `offset: None` is emitted
- **THEN** the ELM expression is `SameOrAfter` with the original operands and precision

### Requirement: Exclusive temporal emission remains stable

Routing simple zero-offset `before` and `after` through timing expressions SHALL preserve their
existing ELM meaning.

#### Scenario: Before and after retain semantics

- **WHEN** simple `before` and `after` expressions are emitted after the parser change
- **THEN** their ELM operators, operand order, and precision match the pre-change behavior
