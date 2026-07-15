## Context

The comparison parser currently routes `is null`, `is true`, and `is false` through generic type
testing. The evaluator compensates for a type literally named `null`, but this produces
noncanonical ELM and cannot make Boolean identity tests correct. Native unary operators and ELM
forms already exist, so the parser must select them before generic type-specifier parsing.

## Goals / Non-Goals

**Goals:**

- Produce canonical AST and ELM for positive and negated null/Boolean tests.
- Evaluate the forms with CQL null semantics.
- Preserve named-type tests and compatibility with previously generated ELM.
- Protect behavior with AST, golden ELM, and evaluator tests.

**Non-Goals:**

- Remove generic `Is` or `As` type operations.
- Change public compile/evaluate APIs.
- Redesign the full comparison precedence level.

## Decisions

### Parse literal test targets before type specifiers

A dedicated branch for `is not? (null | true | false)` will precede generic `is`/`as` parsing.
Keyword boundaries will ensure identifiers or qualified types are not captured as literals.

The alternative—repairing only evaluator handling of generic `Is`—would leave emitted ELM
noncanonical and conflate values with type names.

### Lower negation through `Not`

Positive tests will lower to `UnaryOperator::IsNull`, `IsTrue`, or `IsFalse`. Unless the ELM model
requires a dedicated negated node, `is not` will lower to `Not` around the corresponding positive
test, making negation explicit and reusable across the pipeline.

### Retain legacy ELM compatibility

The evaluator's generic `Is`-type-`null` path may remain for external or previously emitted ELM,
but the compiler will no longer produce that representation. Tests will distinguish compatibility
from the canonical compiler output.

### Require compile-to-ELM evidence

AST tests alone can pass while semantic analysis or emission reconstructs a generic type test.
Golden or structural ELM assertions will therefore be part of the acceptance gate.

## Risks / Trade-offs

- [Dedicated syntax shadows named types] → Require exact literal-keyword boundaries and retain
  named and qualified type regressions.
- [Double negation or incorrect null propagation] → Assert positive and negative forms for true,
  false, and null operands at evaluator level.
- [Legacy compatibility masks compiler regressions] → Assert the exact compiled ELM node shape.
- [Motivating function also depends on temporal parsing] → Keep focused null-test cases
  independent and run the full-function regression once both changes are present.

## Migration Plan

1. Capture current AST, ELM, and evaluator failures.
2. Add dedicated parsing and canonical lowering.
3. Wire semantic/emitter/evaluator handling while retaining legacy input compatibility.
4. Run focused, crate-wide, and HL7 checks before refreshing coverage claims.

Rollback requires only reverting source and tests; no serialized format migration is required.

## Open Questions

- Confirm the canonical ELM representation of each negated form against the reference translator
  before finalizing golden output.
