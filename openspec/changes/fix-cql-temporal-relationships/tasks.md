## 1. Establish Temporal Regression Baseline

- [x] 1.1 Run `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture` and record the dated
  pass, fail, compile-error, eval-error, and skip totals before implementation
- [x] 1.2 Add focused parser controls for repeated conjunctions, parenthesized logical operands,
  nested member access, qualified `ToInterval`, and `end of`
- [x] 1.3 Add the exact multiline medication-request function and a single-line equivalent,
  asserting complete input consumption and equivalent outer AST structure
- [x] 1.4 Add table-driven parser cases for all eight zero-offset relationship spellings, point
  and interval operands, and absent, `month of`, and `hour of` precision

## 2. Correct Relative Timing Parsing

- [x] 2.1 Refactor `parse_relative_timing` so quantity offset remains optional through relationship
  recognition and the relationship is the branch discriminator
- [x] 2.2 Order longer multiword relationships before their shorter prefixes and implement both
  `on or? before|after` and `before|after (or on)?` grammar families
- [x] 2.3 Preserve `TimingPhrase::RelativeTiming` with `offset: None`, complete qualified operands,
  and optional precision in the resulting AST
- [x] 2.4 Run `cargo test -p rh-cql parser` and resolve any neighboring expression regressions

## 3. Close Emission and Evaluation Gaps

- [x] 3.1 Add emitter regressions for simple `before`/`after` and canonical zero-offset
  `SameOrBefore`/`SameOrAfter`, including precision propagation
- [x] 3.2 Update emitter or semantic handling needed for the temporal AST while preserving simple
  exclusive relationship semantics
- [x] 3.3 Run all 17 affected bundled HL7 interval/date-time cases and implement any supported
  temporal evaluator gaps exposed after parsing succeeds
- [x] 3.4 Verify imported HL7 fixture expressions and expected values remain unchanged

## 4. Validate and Document

- [x] 4.1 Run `cargo test -p rh-cql parser`, `cargo test -p rh-cql --test
  emit_conformance_tests`, and `cargo test -p rh-cql --test eval_integration_tests`
- [x] 4.2 Run `cargo test -p rh-cql` and the full `hl7_eval_tests` command; confirm `fail=0` and
  record compile/eval error deltas from baseline
- [x] 4.3 Update `SPEC_COVERAGE.md` and `CONFORMANCE.md` only for forms supported by cited parser,
  emitter, and evaluator evidence
- [x] 4.4 Run `cargo fmt --all`, `just docs-sync`, and `just check`
