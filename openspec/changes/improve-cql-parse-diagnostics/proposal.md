## Why

Recognized infix operators and declaration starters can currently backtrack past their real
failure, causing malformed expressions to be reported as unrelated trailing library content.
Precise committed parsing and structural end-of-input handling are needed for actionable library
and CLI diagnostics.

## What Changes

- Commit after recognized infix operators and declaration starters so missing or malformed
  continuations propagate their deepest useful source span.
- Require complete-library end of input structurally instead of relying only on a remainder check.
- Apply commitment narrowly enough to preserve valid alternatives, query clauses, list
  separators, and `between ... and ...` parsing.
- Add negative parser tests for malformed temporal phrases and right operands.
- Add `rh cql` CLI integration coverage for syntax-error exit code `4` and the stable JSON error
  envelope.

## Capabilities

### New Capabilities

- `cql-cli-diagnostics`: Defines stable CLI syntax-error exit and JSON-envelope behavior for CQL
  commands.

### Modified Capabilities

- `cql-parser-conformance`: Require committed failures at recognized constructs, structural EOF,
  and preservation of the deepest useful source span.

## Impact

- Affected code: `crates/rh-cql/src/parser/` and `apps/rh-cli` CQL integration tests
- Primary parser areas: expression precedence, statement parsing, top-level library parsing, and
  span-to-error conversion
- CLI behavior is clarified and regression-tested; successful command behavior is unchanged
- No public compiler API signature changes or whitespace/newline semantic changes
