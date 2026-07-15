## 1. Map Commitment Boundaries and Add Tests

- [x] 1.1 Inventory recoverable expression and statement parsers that can stop after recognizing an
  infix operator or declaration starter
- [x] 1.2 Add negative tests for malformed temporal phrases, nested right operands, invalid
  declarations, and unexpected trailing constructs with expected source spans
- [x] 1.3 Add valid controls for `between ... and ...`, query clauses, list/tuple separators, and
  alternate statement productions before introducing commitment
- [x] 1.4 Document which complete-library versus fragment parser entry points require EOF

## 2. Implement Precise Parser Failures

- [x] 2.1 Commit narrowly to the required right operand after each recognized infix operator
- [x] 2.2 Make recognized declaration starters propagate invalid declaration errors instead of
  allowing the top-level repetition to stop successfully
- [x] 2.3 Require structural EOF in the complete-library production while preserving fragment parser
  remainder contracts
- [x] 2.4 Preserve the deepest useful committed source span when converting parser failures to
  `CqlError`
- [x] 2.5 Run focused parser tests and remove or relocate any commitment that rejects a valid
  alternative

## 3. Stabilize CLI Diagnostic Behavior

- [x] 3.1 Review the documented `rh cql` exit-code and global JSON-format contract before defining
  integration assertions
- [x] 3.2 Add CLI integration cases showing CQL syntax errors exit with code `4` in human and JSON
  output modes
- [x] 3.3 Assert the JSON error envelope is machine-readable, contains the established stable fields,
  and reports the nested failing construct location
- [x] 3.4 Verify valid CQL commands retain their existing success codes and human/JSON envelopes

## 4. Validate

- [x] 4.1 Run `cargo test -p rh-cql parser` and `cargo test -p rh-cli --test
  cql_integration_test`
- [x] 4.2 Run `cargo test -p rh-cql`, `cargo test -p rh-cli`, and the full HL7 evaluation suite,
  confirming `fail=0`
- [x] 4.3 Run `cargo fmt --all`, `just docs-sync`, and `just check`
