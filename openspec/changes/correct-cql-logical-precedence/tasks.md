## 1. Confirm Grammar and Add Regressions

- [x] 1.1 Confirm repeated `implies` associativity against the CQL 1.5.3 grammar and reference
  translator and record the expected grouping in tests
- [x] 1.2 Add AST and value regressions for `false implies true and false` and every mixed pairing
  of `and`, `or`, `xor`, and `implies`
- [x] 1.3 Add left-associative chains for `and`, `or`, and `xor`, plus the confirmed repeated
  `implies` chain
- [x] 1.4 Add parenthesized controls and equivalent mixed expressions in a function body and query
  `where` clause

## 2. Correct Logical Parser Structure

- [x] 2.1 Reorder precedence entry points so `implies` delegates to `or`/`xor`, which delegates to
  `and`, which delegates to comparison expressions
- [x] 2.2 Preserve left folds for `and`, `or`, and `xor` and encode the confirmed `implies`
  associativity explicitly
- [x] 2.3 Verify `false implies true and false` produces `false implies (true and false)` and
  evaluates to `true`
- [x] 2.4 Run focused parser and evaluator tests and resolve function/query regressions

## 3. Validate and Document

- [x] 3.1 Run `cargo test -p rh-cql parser` and `cargo test -p rh-cql --test
  eval_integration_tests`
- [x] 3.2 Run `cargo test -p rh-cql` and the full HL7 evaluation suite, confirming `fail=0`
- [x] 3.3 Update logical grammar/precedence coverage notes with AST and evaluation evidence
- [x] 3.4 Run `cargo fmt --all`, `just docs-sync`, and `just check`
