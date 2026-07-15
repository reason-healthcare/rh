## ADDED Requirements

### Requirement: Nullological coverage reflects canonical end-to-end support

Coverage documents SHALL distinguish parser acceptance from canonical AST, ELM, and evaluator
support for `IsNull`, `IsTrue`, and `IsFalse` source forms.

#### Scenario: Implemented rows cite all pipeline evidence

- **WHEN** a nullological source form is marked implemented
- **THEN** its notes cite parser AST, canonical ELM, and evaluator tests

#### Scenario: Partial stages remain visible

- **WHEN** any positive or negated form lacks canonical emission or evaluation evidence
- **THEN** the relevant stage remains marked partial or unsupported

#### Scenario: Conformance totals preserve the zero-wrong-answer invariant

- **WHEN** documentation is refreshed after the change
- **THEN** it records the test command and run date and reports a `fail` count of `0`
