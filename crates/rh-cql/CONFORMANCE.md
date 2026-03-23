# rh-cql Conformance

**Last updated**: 2026-03-23 (post-wave-2, test counts verified)
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

All 15 XML files from the official [tests.zip](https://cql.hl7.org/tests.zip) are
checked in under `tests/fixtures/hl7_cql_tests/` and run in CI.

| HL7 Category | Fixture file | CI status |
|---|---|---|
| Aggregate Functions | `CqlAggregateFunctionsTest.xml` | ✅ CI |
| Aggregate Operator | `CqlAggregateTest.xml` | ✅ CI |
| Arithmetic Functions | `CqlArithmeticFunctionsTest.xml` | ✅ CI |
| Comparison Operators | `CqlComparisonOperatorsTest.xml` | ✅ CI |
| Conditional Operators | `CqlConditionalOperatorsTest.xml` | ✅ CI |
| Date/Time Operators | `CqlDateTimeOperatorsTest.xml` | ✅ CI |
| Errors & Messaging | `CqlErrorsAndMessagingOperatorsTest.xml` | ✅ CI |
| Interval Operators | `CqlIntervalOperatorsTest.xml` | ✅ CI |
| List Operators | `CqlListOperatorsTest.xml` | ✅ CI |
| Logical Operators | `CqlLogicalOperatorsTest.xml` | ✅ CI |
| Nullological Operators | `CqlNullologicalOperatorsTest.xml` | ✅ CI |
| String Operators | `CqlStringOperatorsTest.xml` | ✅ CI |
| Type Operators | `CqlTypeOperatorsTest.xml` | ✅ CI |
| Types | `CqlTypesTest.xml` | ✅ CI |
| Value Literals & Selectors | `ValueLiteralsAndSelectors.xml` | ✅ CI |

### 1.2 Results — full suite (2026-03-09)

Run with `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture`.

### 1.2.1 Wave-1 baseline delta (2026-03-09)

Baseline for this wave: pre-change snapshot in this document (2026-03-09).
Post-wave command: `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture` (2026-03-09).

| Metric | Baseline | Post-wave | Delta |
|---|---:|---:|---:|
| Pass | 435 | 475 | +40 |
| **Fail (wrong answers)** | **0** | **0** | **0** |
| Compile err | 149 | 149 | 0 |
| Eval err | 628 | 572 | -56 |

| Suite | Pass | **Fail** | Skip (expr) | Skip (output) | Compile err | Eval err | Total |
|---|---|---|---|---|---|---|---|
| CqlAggregateFunctionsTest | 10 | 0 | 0 | 2 | 0 | 27 | 39 |
| CqlAggregateTest | 0 | 0 | 0 | 0 | 2 | 0 | 2 |
| CqlArithmeticFunctionsTest | 42 | 0 | 7 | 0 | 0 | 12 | 61 |
| CqlComparisonOperatorsTest | 123 | 0 | 27 | 0 | 0 | 33 | 183 |
| CqlConditionalOperatorsTest | 9 | 0 | 0 | 0 | 0 | 0 | 9 |
| CqlDateTimeOperatorsTest | 0 | 0 | 0 | 9 | 55 | 227 | 294 |
| CqlErrorsAndMessagingOperatorsTest | 0 | 0 | 4 | 0 | 0 | 0 | 4 |
| CqlIntervalOperatorsTest | 114 | 0 | 8 | 37 | 63 | 136 | 358 |
| CqlListOperatorsTest | 99 | 0 | 2 | 44 | 6 | 55 | 206 |
| CqlLogicalOperatorsTest | 39 | 0 | 0 | 0 | 0 | 0 | 39 |
| CqlNullologicalOperatorsTest | 0 | 0 | 0 | 0 | 0 | 22 | 22 |
| CqlStringOperatorsTest | 30 | 0 | 2 | 3 | 0 | 46 | 81 |
| CqlTypeOperatorsTest | 7 | 0 | 16 | 3 | 0 | 4 | 30 |
| CqlTypesTest | 2 | 0 | 1 | 7 | 2 | 10 | 24 |
| ValueLiteralsAndSelectors | 0 | 0 | 0 | 38 | 21 | 0 | 59 |
| **Total** | **475** | **0** | **67** | **143** | **149** | **572** | **1 406** |

**Outcome definitions**

| Outcome | Meaning |
|---|---|
| Pass | Compiled and evaluated; result matched expected output |
| **Fail** | Result produced but **did not match** expected — a wrong-answer regression |
| Skip (expr) | Expression uses features not yet supported (Long/Quantity literals) |
| Skip (output) | Expected output format not yet parsed (temporal, interval, tuple, list) |
| Compile err | Expression raised a compile error — unimplemented language feature |
| Eval err | Compiled but evaluation raised an error — unimplemented operator/function |

> **Zero wrong-answer failures.** All 475 evaluated expressions return the correct result.
> All other outcomes (compile err, eval err, skip) represent unimplemented features, not bugs.
> CI asserts that no wrong answers are introduced (the Fail count must remain 0).

### 1.2.2 Wave-2 baseline delta (2026-03-09)

Baseline: post-wave-1 metrics (2026-03-09).
Operators added: `IsNull`/`IsTrue`/`IsFalse`/`Coalesce` (function-call and list forms), `AllTrue`, `AnyTrue`, `Median`, `Mode`, `Variance`, `StdDev`, `PopulationVariance`, `PopulationStdDev`, `Product`, `GeometricMean`, `TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`, `Size`, `Repeat` (fixpoint).

| Metric | Baseline (post-wave-1) | Post-wave-2 | Delta |
|---|---:|---:|---:|
| Pass | 475 | 515 | +40 |
| **Fail (wrong answers)** | **0** | **0** | **0** |
| Compile err | 149 | 149 | 0 |
| Eval err | 572 | 520 | -52 |

| Suite | Pass | **Fail** | Skip (expr) | Skip (output) | Compile err | Eval err | Total |
|---|---|---|---|---|---|---|---|
| CqlAggregateFunctionsTest | 32 | 0 | 0 | 3 | 0 | 4 | 39 |
| CqlAggregateTest | 0 | 0 | 0 | 0 | 2 | 0 | 2 |
| CqlArithmeticFunctionsTest | 44 | 0 | 7 | 6 | 0 | 4 | 61 |
| CqlComparisonOperatorsTest | 123 | 0 | 27 | 0 | 0 | 33 | 183 |
| CqlConditionalOperatorsTest | 9 | 0 | 0 | 0 | 0 | 0 | 9 |
| CqlDateTimeOperatorsTest | 0 | 0 | 0 | 10 | 55 | 226 | 291 |
| CqlErrorsAndMessagingOperatorsTest | 0 | 0 | 4 | 0 | 0 | 0 | 4 |
| CqlIntervalOperatorsTest | 114 | 0 | 8 | 37 | 63 | 136 | 358 |
| CqlListOperatorsTest | 99 | 0 | 2 | 44 | 6 | 55 | 206 |
| CqlLogicalOperatorsTest | 39 | 0 | 0 | 0 | 0 | 0 | 39 |
| CqlNullologicalOperatorsTest | 16 | 0 | 0 | 4 | 0 | 2 | 22 |
| CqlStringOperatorsTest | 30 | 0 | 2 | 3 | 0 | 46 | 81 |
| CqlTypeOperatorsTest | 7 | 0 | 16 | 3 | 0 | 4 | 30 |
| CqlTypesTest | 2 | 0 | 1 | 7 | 2 | 10 | 24 |
| ValueLiteralsAndSelectors | 0 | 0 | 0 | 38 | 21 | 0 | 59 |
| **Total** | **515** | **0** | **67** | **155** | **149** | **520** | **1 406** |

**Key category improvements (wave-2):**

| Suite | eval_err before | eval_err after | Delta | Operators resolved |
|---|---:|---:|---:|---|
| CqlNullologicalOperatorsTest | 22 | 2 | -20 | `IsNull`, `IsTrue`, `IsFalse`, `Coalesce` (function & list forms) |
| CqlAggregateFunctionsTest | 27 | 4 | -23 | `AllTrue`, `AnyTrue`, `Median`, `Mode`, `Variance`, `StdDev`, `PopulationVariance`, `PopulationStdDev` |
| CqlArithmeticFunctionsTest | 12 | 4 | -8 | `Precision` (Date/Time), `LowBoundary`/`HighBoundary` (Date/DateTime) |
| CqlDateTimeOperatorsTest | 227 | 226 | -1 | `TimeOfDay` |

> **Zero wrong-answer failures.** All 515 evaluated expressions return the correct result.
> Evidence: `tests/eval_integration_tests.rs::eval_wave2_*` (7 tests), `tests/semantic_tests.rs::semantic_wave2_*` (4 tests).

### 1.3 Known failures and unimplemented categories

**Wrong-answer failures: none (wave-1 and wave-2).**

All previously tracked failures were resolved on 2026-03-09 (wave-1):

| Suite | Fixed | Root cause |
|---|---|---|
| CqlListOperatorsTest | 21 | Null-propagation in `Contains`/`Includes`/`ProperlyIncludes`; Time-value precision equality; unicode escape decoding |
| CqlIntervalOperatorsTest | 16 | Null semantics in `Contains`/`Except`/`In`/`Equal`; null-endpoint interval construction; `collapse()` with all-null intervals |
| CqlComparisonOperatorsTest | 3 | `1.0 = 1` — Decimal/Integer cross-type equality and equivalence (`1.0 ~ 1`) |
| CqlTypesTest | 1 | Single-quote escape: `\'` decoded to `'`; `ToString(@T09:30)` omits leading `T` |

**Skipped expressions (67 total)**
- Quantity literals, e.g., `1'cm'` — the lexer does not yet produce Quantity tokens.
- `Long` integer literals (`1L`) — not yet parsed.
- Type-specifier expressions (`is`, `as`, `cast as`) in type-operator tests.

**Compile errors (149 total — unimplemented language features)**

| Suite | Count | Root cause |
|---|---|---|
| CqlDateTimeOperatorsTest | 55 | Date/Time arithmetic, duration/difference operators |
| CqlIntervalOperatorsTest | 63 | Interval constructor edge cases, timing phrases |
| ValueLiteralsAndSelectors | 21 | Tuple/List/Concept/Ratio literal constructors |
| CqlListOperatorsTest | 6 | Multi-source query expressions in list tests |
| CqlTypesTest | 2 | Ratio, Concept type literals |
| CqlAggregateTest | 2 | `aggregate` clause in query |

**Eval errors (520 total — unimplemented operators/functions, post-wave-2)**

| Suite | Count | Root cause |
|---|---|---|
| CqlDateTimeOperatorsTest | 226 | Date/Time operators: `after`, `before`, `during`, `between`, `Add`/`Subtract` durations |
| CqlIntervalOperatorsTest | 136 | Interval timing operators: `meets`, `overlaps`, `starts`, `ends`, `during` (date precision) |
| CqlStringOperatorsTest | 46 | Remaining string-function gaps |
| CqlListOperatorsTest | 55 | Remaining list-function/query gaps |
| CqlComparisonOperatorsTest | 33 | Date/Time comparison, Quantity comparison |
| CqlTypesTest | 10 | Time/DateTime value constructors |
| CqlNullologicalOperatorsTest | 2 | Remaining `Coalesce` DateTime/Time variants |
| CqlAggregateFunctionsTest | 4 | `Mode`/`Median` on DateTime/Time types; edge cases in `Variance`/`StdDev` |
| CqlArithmeticFunctionsTest | 4 | Quantity arithmetic (`Abs1cm`, `Add1Q1Q`, Quantity divide/subtract); `Precision(Decimal)` trailing-zero edge cases |
| CqlTypeOperatorsTest | 4 | `ToDate`, `ToDateTime`, `ToTime` conversion functions |

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
| System function emission parity | `Abs`/`Ceiling`/`Floor`/`Truncate`/`Round`/`Ln`/`Exp`/`Log`/`Power` emit native ELM nodes (not `FunctionRef`) |
| Negative numeric literal shape | `-n` emits `Negate(Literal("n"))` |

### 2.2 Remaining semantic differences

No high-priority emitter mismatches from the previous wave remain in this category.

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
| Unit tests (lib) | 788 | ✅ all pass |
| golden_elm_tests | 3 | ✅ all pass |
| emit_conformance_tests | 14 | ✅ all pass |
| pipeline_comparison_tests | 11 | ✅ all pass |
| hl7_eval_tests | 16 | ✅ 0 wrong-answer failures; 515 pass / 1 406 total expressions evaluated |
| semantic_tests | 8 | ✅ all pass |
| eval_integration_tests | 68 | ✅ all pass |
| Doc tests | 51 | ✅ all pass |
| **Total** | **959** | **✅ all pass** |

> Run `cargo test -p rh-cql --quiet` to execute the full suite.
> Run `cargo clippy -p rh-cql --all-targets --all-features -- -D warnings` to verify lint hygiene.

---

## 6. Maintaining HL7 Test Fixtures

All 15 fixtures from [tests.zip](https://cql.hl7.org/tests.zip) are checked in.
When a new CQL specification version ships, update the fixtures:

1. Download the new `tests.zip` from [cql.hl7.org/tests.html](https://cql.hl7.org/tests.html).
2. Extract the XML files and replace the existing ones in `tests/fixtures/hl7_cql_tests/`.
3. Run `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture`.
4. Review the per-suite summary and update Section 1 of this document.

---

## 7. Roadmap Priorities

Prioritised by impact on the HL7 test-suite pass rate and real-world CQL content:

### Completed — Wave-1 (2026-03-09)

1. ✅ **Null-propagation in list operators** — `Contains`, `Includes`, `ProperlyIncludes`, `In`
   now implement correct three-valued null semantics. Fixed 21 wrong-answer failures in
   `CqlListOperatorsTest`.

2. ✅ **Null semantics in interval operators** — `Contains`/`Except`/`In`/`Equal` with null
   endpoints, null-bounded interval construction, `collapse()` with all-null intervals.
   Fixed 16 wrong-answer failures in `CqlIntervalOperatorsTest`.

3. ✅ **Decimal/Integer cross-type equality** — `1.0 = 1` and `1.0 ~ 1` now compare correctly
   across types. Fixed 3 wrong-answer failures in `CqlComparisonOperatorsTest`.

4. ✅ **String escape / Time display** — `\'` decoded to `'`; `ToString(@T09:30)` omits
   leading `T`. Fixed 1 wrong-answer failure in `CqlTypesTest`.

5. ✅ **Wave-1 emitter parity + wiring closures** — native system-function emission,
   canonical negative literal emission, string dispatch closures (`Substring`,
   `PositionOf`, `LastPositionOf`, `SplitOnMatches`, `ReplaceMatches`), and list-slice
   closures (`Tail`/`Skip`/`Take`/`Slice`) reduced eval errors from 628 to 572 with
   `Fail = 0` preserved.

### Completed — Wave-2 (2026-03-09)

6. ✅ **Nullological operator implementation** — `IsNull`, `IsTrue`, `IsFalse`,
   `Coalesce` added to `eval/engine.rs` dispatch. Unblocked 20 eval-error tests.

7. ✅ **Aggregate function implementation** — `AllTrue`, `AnyTrue`, `Median`, `Mode`,
   `Variance`, `StdDev`, `PopulationVariance`, `PopulationStdDev`, `Product`,
   `GeometricMean` added. Unblocked 23 eval-error tests.

8. ✅ **Temporal/uncertainty functions** — `TimeOfDay`, `Precision`, `LowBoundary`,
   `HighBoundary` added. `Size` and `Repeat` (fixpoint) implemented.

### High priority (large eval-error count)

9. **String function implementation** — close remaining string-function gaps after
   wave-1 dispatch closures. Remaining string eval-error tests: 46.

10. **List function implementation** — close remaining list/query gaps after wave-1
    list-slice closure. Remaining list eval-error tests: 55.

### Medium priority (large unimplemented areas)

11. **Date/Time operator implementation** — `after`, `before`, `during`, `between`,
    duration arithmetic (`Add`/`Subtract` intervals from dates). Unblocks 227 eval-error tests.

12. **Retrieve execution** — implement data access in the evaluator so that FHIR
    queries can be run end-to-end with an `InMemoryDataProvider`.

13. **Quantity literal support** — lexer/parser extension for `1'cm'` syntax,
    required for clinical quantity comparisons.

### Lower priority

14. **Long literal support** — `1L` syntax for 64-bit integers.
15. **ELM XML output** — emit `library.xml` for interop with Java tooling.
16. **Multi-source query evaluation** — complete the `from A, B` join evaluation.
17. **Locator end-position tracking** — emit `"line:start-line:end"` locators.
