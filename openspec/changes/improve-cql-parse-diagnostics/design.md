## Context

Several parser layers use recoverable `nom` errors and repetition combinators. Once an infix
operator or declaration starter is recognized, a later failure can therefore backtrack and make
the preceding parse appear complete. The top level then reports the remaining source as trailing
content, losing the location and nature of the actual error. The CLI exposes that degraded error.

## Goals / Non-Goals

**Goals:**

- Commit after grammar points that unambiguously require a continuation.
- Preserve the deepest useful source span through `CqlError` conversion.
- Require structural EOF for complete libraries.
- Stabilize CLI syntax-error exit and JSON-envelope behavior with integration tests.

**Non-Goals:**

- Apply `cut` indiscriminately across the grammar.
- Change valid whitespace/newline behavior or public compiler APIs.
- Redesign the entire diagnostic type hierarchy or error wording.

## Decisions

### Commit only after unambiguous grammar recognition

Infix parsers will remain recoverable until an operator is recognized; the required right operand
will then be committed. Statement parsing will use the same principle for recognized declaration
starters. This preserves alternate parses before commitment while preventing false success after
commitment.

A broad `cut` around whole precedence levels was rejected because it could break constructs where
the same tokens delimit `between`, queries, or collections.

### Audit delimiter-sensitive constructs before each commitment

`between ... and ...`, query clauses, list/tuple separators, and alternate statement productions
will receive focused valid regressions. Commitment points will be introduced narrowly and reviewed
against those cases.

### Make EOF part of the complete-library grammar

The library parser will require end of input structurally, while fragment parsers may continue to
return remainders where that is their contract. The top-level declaration loop will propagate a
failure after a recognized starter instead of allowing `many0` to stop successfully.

### Treat the CLI envelope as a public diagnostic contract

Parser error details will continue through the existing CLI mapping. Integration tests will assert
exit code `4`, JSON validity, stable envelope fields, and a location at the failing construct; they
will avoid overfitting incidental prose unless already documented as stable.

## Risks / Trade-offs

- [Overcommitment rejects valid alternatives] → Add valid neighbor tests before each parser cut.
- [Deepest error is noisy rather than useful] → Prefer committed grammar context and verify source
  spans with focused malformed examples.
- [CLI tests become brittle] → Assert documented envelope structure and semantic location fields,
  not arbitrary full-message snapshots.
- [EOF changes fragment parser behavior] → Apply structural EOF only to the complete-library entry
  point.

## Migration Plan

1. Add negative location tests and valid delimiter/alternative controls.
2. Introduce expression and statement commitment incrementally.
3. Add structural EOF and preserve the deepest span in error conversion.
4. Add CLI JSON/exit-code coverage and run focused plus repository documentation checks.

Rollback is a source revert; no user data or API migration is required.

## Open Questions

- Which diagnostic message fields are already documented as stable beyond exit code and the JSON
  envelope shape?

## Implementation Inventory

- Logical, equality, comparison, union, additive, multiplicative, and power precedence levels use
  recoverable repetition around an operator and its required right operand.
- `between` uses a recoverable optional tuple containing both bounds and the `and` delimiter.
- Interval and timing relationships use recoverable probing before parsing their required operand;
  malformed continuations can therefore fall through to a shorter expression.
- The complete-library parser uses recoverable optional/repeated productions for the library
  header and declaration groups. A recognized but malformed starter can stop its group.
- `parse_statement` recoverably probes function and expression definitions that share `define`.
- `CqlParser::parse`, `statement::parse_library`, and `CqlParser::parse_expression` are complete
  entry points. Internal expression and declaration parsers retain remainder-returning fragment
  contracts for grammar composition.
