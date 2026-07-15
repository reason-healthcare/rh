# rh-cql Parser Conformance Fix Plan

**Area**: `crates/rh-cql`  
**Primary scope**: temporal relationships, Boolean/null tests, logical precedence, and parse
diagnostics  
**Baseline date**: 2026-07-14  
**Status**: Completed 2026-07-15

This plan addresses the parser defects exposed by the valid CQL function containing chained
conjunctions, `is null`, `end of`, and `on or after`. The work should preserve the current
zero-wrong-answer conformance invariant while converting affected compile errors into passing
parse, emit, and evaluation cases.

---

## 1. Goals and Non-Goals

### Goals

- Parse the original `Is Current Medication Request` function without source rewrites.
- Support the complete CQL 1.5.3 zero-offset temporal relationship forms, including
  `on or after` and `on or before`.
- Emit canonical `IsNull`, `IsTrue`, and `IsFalse` ELM for Boolean/null-test syntax.
- Give `implies` lower precedence than `or`/`xor` and `and`.
- Report malformed expressions at the actual failing construct instead of as trailing library
  content at an earlier Boolean operator.
- Add regression coverage at parser, emitter, evaluator, CLI, and HL7 conformance levels.
- Correct conformance documentation so every claimed capability has test evidence.

### Non-Goals

- Redesigning the full hand-written parser or replacing `nom`.
- Expanding unrelated CQL operator coverage.
- Changing whitespace or newline semantics.
- Completing offset/boundary timing semantics that are already documented as placeholders,
  except where required by the zero-offset relationships in this plan.
- Changing public compiler API signatures.

---

## 2. Current Baseline

- Repeated conjunctions and left folding already work: `A and B and C` parses correctly.
- Parenthesized Boolean operands, nested member access, qualified function calls, and `end of`
  parse correctly in isolation.
- The first syntax failure in the original function is the zero-offset `on or after` phrase.
- `parse_relative_timing` recognizes an optional offset but rejects the expression before parsing
  a timing direction when the offset is absent.
- The AST and emitter already represent and emit `OnOrAfter`/`OnOrBefore` with `offset: None`.
- All 17 single-line `on or after`/`on or before` expressions in the bundled HL7 interval and
  date/time fixtures currently fail parsing.
- `A is null`, `A is true`, and `A is false` currently parse as generic `Is` type tests. The
  evaluator special-cases a type named `null`; `true is true` and `false is false` evaluate
  incorrectly.
- The current precedence chain makes `implies` bind more tightly than `and` and `or`.
- The HL7 test harness reports remaining compile/evaluation errors as unimplemented outcomes, so
  the affected cases do not currently fail the test target.

Record the exact pre-change totals from a fresh run before implementation:

```bash
cargo test -p rh-cql --test hl7_eval_tests -- --nocapture
```

Expected starting summary from the investigation:

```text
pass=1241
fail=0
compile_err=89
eval_err=23
skip=45
```

---

## 3. Implementation Sequence

### Phase 1 — Add Focused Regression Coverage

- [x] 1.1 Add parser tests for repeated `and`, parenthesized `and`/`or`, nested member access,
  `is null`, qualified `ToInterval`, and `end of` so the already-working components remain
  protected.
- [x] 1.2 Add a parser regression containing the exact multiline
  `Is Current Medication Request` function.
- [x] 1.3 Add the same function on one physical line and assert that it produces the same AST
  shape as the multiline version.
- [x] 1.4 Add table-driven temporal relationship cases for:
  - `on before`
  - `on or before`
  - `on after`
  - `on or after`
  - `before`
  - `before or on`
  - `after`
  - `after or on`
- [x] 1.5 Cover temporal relationships with no precision, with `month of`/`hour of`, and with
  point and interval operands.
- [x] 1.6 Add canonical AST tests for `is null`, `is true`, `is false`, and each `is not` form.
- [x] 1.7 Add a precedence regression whose result differs between the correct and current
  groupings, such as `false implies true and false`.

The tests added alongside each implementation patch must fail against the old behavior and pass
against the new behavior. Avoid merging a commit that leaves the default test suite red.

### Phase 2 — Fix Zero-Offset Temporal Relationships

- [x] 2.1 Refactor `parse_relative_timing` so `quantityOffset` remains optional through timing
  direction parsing.
- [x] 2.2 Parse the temporal relationship before deciding that the input is not a timing phrase;
  do not use offset presence as the discriminator.
- [x] 2.3 Preserve the existing `TimingPhrase::RelativeTiming` representation with
  `offset: Option<TimingOffset>`.
- [x] 2.4 Ensure longer multiword forms are attempted before their shorter prefixes.
- [x] 2.5 Match the CQL 1.5.3 `temporalRelationship` production, including the optional `or` in
  `on or? before|after` and the optional `or on` suffix after `before|after`.
- [x] 2.6 Verify that simple `before` and `after` retain their current emitted ELM semantics even
  if their internal AST route changes from `BinaryExpression` to `TimingExpression`.
- [x] 2.7 Verify zero-offset `OnOrBefore` emits `SameOrBefore` and `OnOrAfter` emits
  `SameOrAfter`, including precision propagation.
- [x] 2.8 Run all 17 bundled HL7 `on or after`/`on or before` cases and close any downstream
  semantic, emit, or evaluation gaps exposed after parsing succeeds.

Likely code areas:

- `src/parser/expression/precedence.rs`
- `src/parser/ast.rs` only if an uncovered grammar form cannot use the existing variants
- `src/emit/operators.rs`
- `src/eval/operators/temporal.rs`
- `tests/fixtures/hl7_cql_tests/CqlIntervalOperatorsTest.xml`
- `tests/fixtures/hl7_cql_tests/CqlDateTimeOperatorsTest.xml`

Do not edit the imported HL7 fixture expressions or expected values to accommodate the parser.

### Phase 3 — Implement Canonical Boolean/Null Tests

- [x] 3.1 Add a dedicated parser branch for
  `expression is not? (null | true | false)` before generic `is/as typeSpecifier` parsing.
- [x] 3.2 Lower positive forms to `UnaryOperator::IsNull`, `IsTrue`, or `IsFalse`.
- [x] 3.3 Lower negative forms to `Not(IsNull(...))`, `Not(IsTrue(...))`, or
  `Not(IsFalse(...))` unless a dedicated negated representation is required by canonical ELM.
- [x] 3.4 Preserve generic type checks such as `value is FHIR.Quantity` and
  `value is not FHIR.Quantity`.
- [x] 3.5 Emit canonical ELM `IsNull`, `IsTrue`, and `IsFalse` nodes from source syntax.
- [x] 3.6 Add evaluator assertions for true, false, and null operands using CQL three-valued
  logic.
- [x] 3.7 Retain the evaluator's legacy `Is`-type-`null` compatibility path initially if needed
  for previously generated or external ELM, but ensure the compiler no longer emits that shape.
- [x] 3.8 Add compile-to-ELM golden assertions so AST-only correctness cannot mask a
  noncanonical emitted representation.

Likely code areas:

- `src/parser/expression/precedence.rs`
- `src/parser/ast.rs`
- `src/semantics/analyzer.rs`
- `src/emit/operators.rs`
- `src/eval/engine.rs`
- `tests/golden/`
- `tests/eval_integration_tests.rs`

### Phase 4 — Correct Logical Precedence

- [x] 4.1 Reorder the logical precedence entry points to match CQL, from lowest to highest:
  `implies`, `or`/`xor`, `and`, then comparison and higher-precedence expressions.
- [x] 4.2 Keep `and`, `or`, and `xor` left-associative.
- [x] 4.3 Confirm the intended associativity of repeated `implies` against the CQL 1.5.3 grammar
  and reference translator, then encode it explicitly in tests.
- [x] 4.4 Add mixed logical tests covering all pairings of `and`, `or`, `xor`, and `implies`,
  with and without parentheses.
- [x] 4.5 Verify existing query `where` expressions and function bodies do not regress.

Likely code area:

- `src/parser/expression/precedence.rs`

### Phase 5 — Improve Parse Commitment and EOF Diagnostics

- [x] 5.1 Make a recognized infix operator commit to requiring its right operand so a nested
  failure is not silently treated as the end of the expression.
- [x] 5.2 Apply commitment narrowly; audit `between ... and ...`, query clauses, list separators,
  and alternate statement parsers before adding `nom::combinator::cut` broadly.
- [x] 5.3 Make the complete library production require EOF structurally instead of relying only
  on a post-parse remainder check.
- [x] 5.4 Change the top-level statement loop so a recognized declaration starter followed by an
  invalid declaration propagates its error rather than ending `many0` successfully.
- [x] 5.5 Preserve the deepest useful source span when converting `nom` errors to `CqlError`.
- [x] 5.6 Add negative tests asserting that malformed temporal phrases point at the temporal
  phrase and malformed right operands point after their operator.
- [x] 5.7 Add CLI integration coverage for exit code `4` and stable JSON error envelopes on CQL
  syntax errors.

Likely code areas:

- `src/parser/expression/precedence.rs`
- `src/parser/statement.rs`
- `src/parser/mod.rs`
- `src/parser/span.rs`
- `apps/rh-cli/tests/cql_integration_test.rs`

---

## 4. Acceptance Criteria

The change is complete when all of the following are true:

- [x] The exact original multiline function parses successfully without changing its CQL.
- [x] Its outer AST is `And(And(status-in, intent-in), Or(is-null, on-or-after))`.
- [x] `end of FHIRHelpers.ToInterval(...)` consumes the complete qualified invocation.
- [x] `validityPeriod is null` produces an `IsNull` AST and canonical ELM node.
- [x] `true is true`, `false is false`, and `null is null` evaluate to `true`.
- [x] The corresponding `is not` forms evaluate correctly.
- [x] All 17 bundled zero-offset `on or after`/`on or before` expressions compile, and their
  supported evaluations match the existing HL7 expected outputs.
- [x] `false implies true and false` groups as
  `false implies (true and false)` and evaluates to `true`.
- [x] Multiline and single-line forms produce equivalent AST structure.
- [x] A malformed nested right operand is reported at the nested construct, not as unexpected
  content beginning at the preceding `and`.
- [x] No previously passing HL7 case becomes a wrong answer.
- [x] The conformance `fail` count remains `0`.
- [x] Compile/evaluation error counts improve from the recorded baseline.

---

## 5. Validation Commands

Run focused checks during implementation:

```bash
cargo test -p rh-cql parser
cargo test -p rh-cql --test emit_conformance_tests
cargo test -p rh-cql --test eval_integration_tests
cargo test -p rh-cli --test cql_integration_test
```

Run the full CQL and conformance checks before completion:

```bash
cargo test -p rh-cql
cargo test -p rh-cql --test hl7_eval_tests -- --nocapture
```

Run repository gates after code and documentation are finalized:

```bash
cargo fmt --all
just docs-sync
just check
```

If the local Java reference translator is configured, compare the focused source and affected
HL7 cases against it before updating conformance claims.

---

## 6. Documentation and Conformance Updates

- [x] 6.1 Update `crates/rh-cql/SPEC_COVERAGE.md` only after canonical AST, emit, and eval tests
  support each claimed status.
- [x] 6.2 Correct the nullological rows if any stage remains partial.
- [x] 6.3 Add an explicit temporal-relationship coverage note for zero-offset multiword forms.
- [x] 6.4 Refresh `crates/rh-cql/CONFORMANCE.md` with dated pre/post totals.
- [x] 6.5 Record how many of the 17 affected HL7 cases moved from compile error to pass or to a
  newly exposed downstream category.
- [x] 6.6 Keep generated reports under `conformance/results/` generated by their scripts rather
  than editing them manually.

---

## 7. Risks and Mitigations

- **Parser backtracking regressions**: Removing the offset gate may cause temporal parsing to
  claim tokens intended for another construct. Mitigate with table-driven positive and negative
  parser tests and by requiring a successfully parsed temporal direction before committing.
- **AST-shape regressions for simple `before`/`after`**: A unified timing parser may change the
  internal representation. Mitigate by asserting emitted ELM equivalence and evaluator behavior,
  not only AST variants.
- **Over-broad `cut` usage**: Commitment can prevent legitimate alternatives from backtracking.
  Add commitment only after an unambiguous operator or declaration prefix and test neighboring
  grammar alternatives.
- **Hidden downstream timing gaps**: Fixing parsing may convert compile errors into emit or eval
  errors. Treat the 17 focused HL7 cases as end-to-end scope rather than declaring completion at
  parse success.
- **Compatibility with previously emitted noncanonical ELM**: Keep evaluator compatibility for
  `Is` with type `null` during the initial change while ensuring new compilation uses `IsNull`.
- **Conformance metric drift**: Capture the baseline immediately before implementation and require
  zero new wrong answers as a hard gate.

---

## 8. Suggested Review Boundaries

The work can be delivered as three independently reviewable pull requests:

1. **Temporal parsing and focused HL7 closure** — Phases 1–2 for timing relationships, including
   exact-function regression coverage.
2. **Canonical Boolean/null tests and precedence** — Phases 3–4 with golden ELM and evaluator
   coverage.
3. **Diagnostics and conformance refresh** — Phase 5 plus documentation, final metrics, and full
   repository gates.

PR 2 may proceed in parallel with PR 1 after the shared regression-test conventions are agreed.
PR 3 should follow both because diagnostic expectations and conformance totals depend on their
final parser behavior.
