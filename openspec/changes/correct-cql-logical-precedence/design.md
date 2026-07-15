## Context

The logical parser entry points currently let `implies` bind more tightly than `and` and `or`.
CQL requires `implies` to be the lowest-precedence logical operator. Because the AST is already a
nested binary tree, the correction is confined to parser delegation and associativity.

## Goals / Non-Goals

**Goals:**

- Encode the CQL logical precedence hierarchy directly in parser entry points.
- Make operator associativity explicit and testable.
- Demonstrate correct grouping through both AST and evaluation results.
- Preserve explicit parentheses and existing valid query/function parsing.

**Non-Goals:**

- Change logical operator runtime semantics or ELM representations.
- Rework comparison, arithmetic, or timing precedence beyond delegation boundaries.
- Change public APIs.

## Decisions

### Make `implies` the logical entry point

The precedence chain will delegate from `implies` to `or`/`xor`, from `or`/`xor` to `and`, and
from `and` to comparison expressions. This structure makes lower precedence correspond to a
higher-level parser function.

Adding an AST regrouping pass after parsing was rejected because it would obscure grammar errors
and complicate source spans.

### Preserve left folds except where the grammar says otherwise

`and`, `or`, and `xor` will continue to use left folds. Repeated `implies` associativity will be
confirmed from CQL 1.5.3 grammar/reference behavior and then encoded directly rather than inherited
accidentally from a helper.

### Use value-sensitive regression expressions

Tests such as `false implies true and false` will assert both the nested AST and the evaluated
value. A matrix of mixed logical pairs, plus parenthesized controls, will prevent a syntactically
plausible but semantically wrong ordering.

## Risks / Trade-offs

- [Parser delegation becomes recursive or skips a level] → Cover every mixed operator pairing.
- [Repeated `implies` gets an assumed associativity] → Confirm against normative/reference
  behavior before implementing the fold.
- [Query or function parsing regresses] → Add representative `where` and function-body cases to
  focused tests and run the full crate suite.

## Migration Plan

1. Add failing grouping and value regressions for mixed logical expressions.
2. Reorder parser entry points and encode associativity.
3. Run parser, evaluator, crate-wide, and conformance suites.
4. Update coverage evidence only after the test results are recorded.

Rollback is a source revert; there is no API or persisted-data migration.

## Open Questions

- What associativity does the CQL 1.5.3 reference translator apply to an unparenthesized repeated
  `implies` chain?
