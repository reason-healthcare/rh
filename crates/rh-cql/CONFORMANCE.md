# rh-cql Conformance

**Last updated**: 2026-03-09
**CQL specification**: 1.5.3 (https://cql.hl7.org)
**Test suite source**: https://cql.hl7.org/tests.html (`tests.zip`)

This is the single authoritative conformance document for `rh-cql`.
It records:

- HL7 official test suite results (run from `tests/fixtures/hl7_cql_tests/`)
- ELM emission fidelity vs. the Java reference translator
- Parser coverage (jvmTest suite)
- A feature-by-feature breakdown of what is implemented and what remains

---

## 1. HL7 Official Evaluation Test Suite

The HL7 CQL specification ships a test suite in `tests.zip` consisting of XML
files, each covering one category of the language. Each `<test>` element contains
a CQL expression and an expected output value; `invalid="true"` tests are expected
to error.

Tests are executed by `tests/hl7_eval_tests.rs` which wraps every expression in a
`library HlTest / define Result: <expr>` CQL library, compiles it end-to-end
through the three-stage pipeline, evaluates it, and compares the result.

### 1.1 Test fixture coverage

Only a subset of the spec's test XML files are currently checked in under
`tests/fixtures/hl7_cql_tests/`. The table below lists **all** categories from the
spec; rows marked **✅ Fixture present** have been run and contribute to CI.

| HL7 Category | Fixture file | CI status |
|---|---|---|
| Aggregate Functions | *(not yet added)* | — |
| Aggregate Operator | *(not yet added)* | — |
| Arithmetic Functions | `CqlArithmeticFunctionsTest.xml` | ✅ CI |
| Comparison Operators | *(not yet added)* | — |
| Conditional Operators | `CqlConditionalOperatorsTest.xml` | ✅ CI |
| Date/Time Operators | *(not yet added)* | — |
| Errors & Messaging | *(not yet added)* | — |
| Interval Operators | *(not yet added)* | — |
| List Operators | *(not yet added)* | — |
| Logical Operators | `CqlLogicalOperatorsTest.xml` | ✅ CI |
| Nullological Operators | `CqlNullologicalOperatorsTest.xml` | ✅ CI |
| String Operators | *(not yet added)* | — |
| Type Operators | *(not yet added)* | — |
| Types | *(not yet added)* | — |
| Value Literals & Selectors | *(not yet added)* | — |

To add missing fixtures: download [tests.zip](https://cql.hl7.org/tests.zip),
extract the XML files, and drop them in `tests/fixtures/hl7_cql_tests/`. The test
runner discovers all `*.xml` files in that directory automatically.

### 1.2 Results for covered suites (2026-03-09)

Run with `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture`.

| Suite | Pass | Fail | Skip (expr) | Skip (output) | Eval error | Total tests |
|---|---|---|---|---|---|---|
| CqlArithmeticFunctionsTest | 42 | 0 | 7 | 0 | 12 | 61 |
| CqlConditionalOperatorsTest | 9 | 0 | 0 | 0 | 0 | 9 |
| CqlLogicalOperatorsTest | 39 | 0 | 0 | 0 | 0 | 39 |
| CqlNullologicalOperatorsTest | 0 | 0 | 0 | 0 | 22 | 22 |
| **Total** | **90** | **0** | **7** | **0** | **34** | **131** |

**Outcome definitions**

| Outcome | Meaning |
|---|---|
| Pass | Expression compiled and evaluated; result matched expected output |
| Fail | Result produced but did not match expected — a wrong-answer regression |
| Skip (expr) | Expression uses features not yet supported (Long literals, Quantity literals) |
| Skip (output) | Expected output uses a format not yet parsed (temporal, interval, tuple, list) |
| Eval error | Expression compiled but evaluation raised an error — unimplemented feature |

> **No failures (wrong answers) exist in any covered suite.** CI asserts this will remain zero.
> Eval-error and skip counts track unimplemented features and decrease over time.

### 1.3 Known skip / eval-error categories

**Skipped expressions (7 total — Arithmetic suite)**
- Quantity literals, e.g., `1'cm'` — the lexer does not yet produce Quantity tokens.
- `Long` integer literals (`1L`) — not yet parsed.

**Eval errors (34 total)**

| Category | Count | Root cause |
|---|---|---|
| Nullological: `IsNull`, `IsTrue`, `IsFalse`, `Coalesce` | 22 | Operators not yet implemented in `eval/engine.rs` dispatch |
| Arithmetic: `Truncate`, `Round`, `Ln`, `Exp`, `Log` | 12 | Functions emitted as `FunctionRef` nodes; evaluator resolves them as user-defined, not built-in |

---

## 2. ELM Emission Fidelity

The ELM emitter lives in `src/emit/`. Its output is compared against the Java
reference translator (cqframework/clinical_quality_language v4.2.0) via
`conformance/scripts/compare_translators.py`.

### 2.1 Fixed (no longer differences)

| Item | Resolution |
|---|---|
| Library `identifier` field | Always emitted even when empty |
| Annotation `type` discriminator | Now includes `"type": "CqlToElmInfo"` |
| Default context | `"context": "Unfiltered"` emitted on expression defs when no context is declared |
| `ToDecimal` promotion for integer division | Operands wrapped in `ToDecimal` |

### 2.2 Remaining semantic differences

| Issue | Java behavior | rh-cql behavior | Priority |
|---|---|---|---|
| System function resolution | `Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power` emitted as their specific ELM types | Emitted as `FunctionRef` | High |
| Negative literal representation | `-5` → `Negate(Literal("5"))` | `-5` → `Literal("-5")` | High |

### 2.3 Intentional / low-priority differences

| Item | Java | rh-cql | Notes |
|---|---|---|---|
| `localId` emission | Only with `--debug` | Always when annotations enabled | Provides richer traceability |
| Locator format | `"line:col-line:col"` (range) | `"line:col"` (start only) | Would need parser end-position tracking |
| Empty arrays | `annotation: []` included | Omitted | Acceptable per project decision |

---

## 3. Parser Conformance (jvmTest Suite)

The jvmTest suite consists of 119 real-world CQL files compiled by both the Java
reference translator and the `cql-test-parse` binary. A pass means the Rust parser
produces a structurally equivalent ELM library.

| Date | Passed | Total | Pass rate |
|---|---|---|---|
| 2024-12-10 (baseline) | 90 | 119 | 75.6% |

> A full rerun requires Java 17+ tooling via `conformance/scripts/setup.sh`.
> Parser improvements from the multi-stage pipeline refactor (2026-03-07) are
> expected to raise this above 75.6%; rerun pending.

**To rerun:**

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh            # one-time Java setup
python3 scripts/compare_translators.py --suite test-cases/jvmTest/
```

### 3.1 Recently added parser features (since 2024-12-10 baseline)

| Feature | Status |
|---|---|
| `predecessor of` / `successor of` | ✅ Added |
| Power operator (`^`) | ✅ Added |
| `minimum Type` / `maximum Type` | ✅ Added |
| `aggregate` clause in queries | ✅ Added |
| Comma-separated `let` items in a single clause | ✅ Added |
| Extended timing phrase keywords (`starts before`, `properly includes`, `occurs during`) | ✅ Added |

### 3.2 Known parser gaps

| Feature | Affected files |
|---|---|
| Complex QDM-specific syntax | `CMS9v4_QDM.cql` |
| Multi-library CDS / CQM measure files | `CMS9v4_*.cql` (4 files) |
| Some timing expression edge cases | Several jvmTest files |

---

## 4. Feature Implementation Status

For an exhaustive, operator-by-operator breakdown of what is and is not implemented across all
four pipeline stages (Parse → Semantic → Emit → Eval), see:

➡ **[SPEC_COVERAGE.md](SPEC_COVERAGE.md)**

That document covers every operator defined in CQL 1.5.3 Appendix B, all 130 lexer keywords, all
grammar productions, and a prioritised gap summary.

### 4.1 Compilation pipeline features

| Feature | Status |
|---|---|
| CQL → AST parsing (`nom` combinators) | ✅ |
| Semantic analysis (type inference, scope, overload resolution) | ✅ |
| Typed AST (`TypedLibrary`) | ✅ |
| ELM JSON emission | ✅ |
| Source-map generation (CQL span ↔ ELM node) | ✅ |
| Unified diagnostic system (`Diagnostic` with severity, code, span) | ✅ |
| ELM XML output | ❌ |
| Compile-to-FHIR-Library | ❌ |

### 4.2 Evaluation engine features

| Feature | Status |
|---|---|
| Three-valued logic (null propagation) | ✅ |
| Integer, Decimal, String, Boolean values | ✅ |
| Date/Time/Time values and arithmetic | ✅ |
| Interval values and operations | ✅ |
| List values and operations | ✅ |
| Tuple values | Partial |
| FHIR data access via `InMemoryDataProvider` | ✅ |
| Terminology service (`InMemoryTerminologyProvider`) | ✅ |
| Retrieve execution | ❌ |
| Library includes in evaluation | Partial |
| Query evaluation (single source) | ✅ |
| Query aggregate clause | ❌ |

---

## 5. Test Suite Summary

All tests run via `cargo test -p rh-cql`.

| Test binary | Tests | Last result |
|---|---|---|
| Unit tests (lib) | 781 | ✅ all pass |
| golden_elm_tests | 8 | ✅ all pass |
| emit_conformance_tests | 52 | ✅ all pass |
| pipeline_comparison_tests | 3 | ✅ all pass |
| hl7_eval_tests | 4 | ✅ all pass (131 evaluated, 0 wrong answers) |
| semantic_tests | 11 | ✅ all pass |
| eval_integration_tests | 2 | ✅ all pass |
| **Total** | **912** | **✅ all pass** |

> Run `cargo test -p rh-cql --quiet` to execute the full suite.
> Run `cargo clippy -p rh-cql --all-targets --all-features -- -D warnings` to verify lint hygiene.

---

## 6. How to Add Missing HL7 Test Fixtures

1. Download [tests.zip](https://cql.hl7.org/tests.zip) from the CQL spec site.
2. Extract the XML files whose names match `Cql*Test.xml`.
3. Copy them into `crates/rh-cql/tests/fixtures/hl7_cql_tests/`.
4. Run `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture`.
5. Review the printed per-suite summary and update Section 1 of this document.

The test runner (`tests/hl7_eval_tests.rs`) auto-discovers all `*.xml` files in
that directory — no code changes are needed.

---

## 7. Roadmap Priorities

Prioritised by impact on the HL7 test-suite pass rate and real-world CQL content:

### High priority (wrong-answer risk or large eval-error count)

1. **System function resolution in emitter** — emit `Abs`, `Ceiling`, `Floor`,
   `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power` as their specific ELM types
   instead of `FunctionRef`. This unblocks 12 arithmetic eval-error tests.

2. **Negative literal representation** — represent `-5` as `Negate(Literal("5"))`
   in ELM output to match the Java reference translator.

3. **Nullological operator implementation** — add `IsNull`, `IsTrue`, `IsFalse`,
   `Coalesce` to `eval/engine.rs` dispatch. Unblocks 22 eval-error tests.

### Medium priority (coverage gaps)

4. **Download remaining HL7 test fixtures** — the full test suite has ~11 more
   categories not yet in CI (Comparison, String, List, Interval, Date/Time, etc.).

5. **Retrieve execution** — implement data access in the evaluator so that FHIR
   queries can be run end-to-end with an `InMemoryDataProvider`.

6. **Quantity literal support** — lexer/parser extension for `1'cm'` syntax,
   required for clinical quantity comparisons.

### Lower priority

7. **Long literal support** — `1L` syntax for 64-bit integers.
8. **ELM XML output** — emit `library.xml` for interop with Java tooling.
9. **Multi-source query evaluation** — complete the `from A, B` join evaluation.
10. **Locator end-position tracking** — emit `"line:start-line:end"` locators.
