## Why

The current parser makes `implies` bind more tightly than `and` and `or`, contrary to the CQL
grammar and capable of changing query and function results. The logical precedence ladder needs a
focused correction with explicit associativity coverage.

## What Changes

- Reorder logical parsing from lowest to highest precedence as `implies`, `or`/`xor`, `and`, then
  comparison and higher-precedence expressions.
- Keep `and`, `or`, and `xor` left-associative and encode the specification-defined associativity
  of repeated `implies`.
- Add mixed-operator parser and evaluator regressions, including expressions whose values expose
  incorrect grouping.
- Verify parenthesized expressions, query `where` clauses, and function bodies retain their
  explicit or expected grouping.
- Update parser conformance documentation only when backed by the new tests.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `cql-parser-conformance`: Require the CQL logical precedence hierarchy and explicit operator
  associativity across mixed logical expressions.
- `cql-spec-coverage`: Require logical-precedence coverage claims to be backed by grouping and
  evaluation evidence.

## Impact

- Affected crate: `crates/rh-cql`
- Primary code: `src/parser/expression/precedence.rs`
- Tests: parser grouping cases, evaluator integration cases, queries, and function bodies
- Documentation: `crates/rh-cql/SPEC_COVERAGE.md` and related conformance notes
- No AST, ELM, evaluator API, or public compiler signature changes are expected
