## 1. Add Canonical-Shape Regressions

- [x] 1.1 Add parser AST tests for `is null`, `is true`, `is false`, and each corresponding
  `is not` form
- [x] 1.2 Add named and qualified type controls for `value is FHIR.Quantity` and
  `value is not FHIR.Quantity`
- [x] 1.3 Add the medication-request nested `validityPeriod is null` expression and assert an
  `IsNull` node over the complete member-access operand
- [x] 1.4 Add compile-to-ELM golden or structural assertions for all positive and negated forms

## 2. Implement Parser and Compiler Lowering

- [x] 2.1 Add a dedicated `expression is not? (null | true | false)` parser branch before generic
  `is`/`as` type-specifier handling with exact keyword boundaries
- [x] 2.2 Lower positive forms to `IsNull`, `IsTrue`, and `IsFalse` AST operations and negative
  forms to the reference-compatible explicit negation
- [x] 2.3 Update semantic analysis and ELM emission as needed so compiled source retains the
  canonical native node shape
- [x] 2.4 Preserve generic named-type parsing and emission without public API changes

## 3. Implement and Verify Evaluation

- [x] 3.1 Add evaluator assertions for matching and nonmatching true, false, and null operands
  across positive forms
- [x] 3.2 Add evaluator assertions for all `is not` forms and verify they match explicit negation
  of the canonical positive operations
- [x] 3.3 Wire or correct native nullological dispatch and three-valued behavior exposed by the
  focused tests
- [x] 3.4 Retain and isolate the legacy generic `Is(null)` compatibility path while asserting the
  compiler no longer emits it

## 4. Validate and Document

- [x] 4.1 Run parser, `emit_conformance_tests`, and `eval_integration_tests` focused checks
- [x] 4.2 Run `cargo test -p rh-cql` and the full HL7 evaluation suite, confirming `fail=0`
- [x] 4.3 Update nullological rows in `SPEC_COVERAGE.md` and conformance totals only where all
  claimed stages have cited evidence
- [x] 4.4 Run `cargo fmt --all`, `just docs-sync`, and `just check`
