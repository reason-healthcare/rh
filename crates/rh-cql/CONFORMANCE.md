# rh-cql Conformance

**Last updated**: 2026-03-09 (full 15-suite run)
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

| Suite | Pass | **Fail** | Skip (expr) | Skip (output) | Compile err | Eval err | Total |
|---|---|---|---|---|---|---|---|
| CqlAggregateFunctionsTest | 10 | 0 | 0 | 2 | 0 | 27 | 39 |
| CqlAggregateTest | 0 | 0 | 0 | 0 | 2 | 0 | 2 |
| CqlArithmeticFunctionsTest | 42 | 0 | 7 | 0 | 0 | 12 | 61 |
| CqlComparisonOperatorsTest | 120 | **3** | 27 | 0 | 0 | 33 | 183 |
| CqlConditionalOperatorsTest | 9 | 0 | 0 | 0 | 0 | 0 | 9 |
| CqlDateTimeOperatorsTest | 0 | 0 | 0 | 9 | 55 | 227 | 294 |
| CqlErrorsAndMessagingOperatorsTest | 0 | 0 | 4 | 0 | 0 | 0 | 4 |
| CqlIntervalOperatorsTest | 96 | **16** | 8 | 37 | 63 | 138 | 358 |
| CqlListOperatorsTest | 68 | **21** | 2 | 31 | 6 | 78 | 207 |
| CqlLogicalOperatorsTest | 39 | 0 | 0 | 0 | 0 | 0 | 39 |
| CqlNullologicalOperatorsTest | 0 | 0 | 0 | 0 | 0 | 22 | 22 |
| CqlStringOperatorsTest | 2 | 0 | 2 | 0 | 0 | 77 | 81 |
| CqlTypeOperatorsTest | 7 | 0 | 16 | 3 | 0 | 4 | 30 |
| CqlTypesTest | 1 | **1** | 1 | 7 | 2 | 10 | 24 |
| ValueLiteralsAndSelectors | 0 | 0 | 0 | 38 | 21 | 0 | 59 |
| **Total** | **394** | **41** | **67** | **127** | **149** | **628** | **1 412** |

**Outcome definitions**

| Outcome | Meaning |
|---|---|
| Pass | Compiled and evaluated; result matched expected output |
| **Fail** | Result produced but **did not match** expected — a wrong-answer regression |
| Skip (expr) | Expression uses features not yet supported (Long/Quantity literals) |
| Skip (output) | Expected output format not yet parsed (temporal, interval, tuple, list) |
| Compile err | Expression raised a compile error — unimplemented language feature |
| Eval err | Compiled but evaluation raised an error — unimplemented operator/function |

> **41 wrong-answer failures exist** across Comparison (3), Interval (16), List (21), and Types (1).
> All other outcomes (compile err, eval err, skip) represent unimplemented features, not bugs.
> CI asserts that no *new* wrong answers are introduced (the Fail count must not grow).

### 1.3 Known failures and unimplemented categories

**Wrong-answer failures (41 total)**

| Suite | Count | Root cause |
|---|---|---|
| CqlListOperatorsTest | 21 | Null-propagation in `Contains`/`Includes`/`ProperlyIncludes`, 0-based vs 1-based `Indexer`, Time-value equality |
| CqlIntervalOperatorsTest | 16 | Null semantics in `Contains`/`Except`/`In`/`Equal`, integer/decimal `Meets` successor arithmetic, DateTime precision in `IncludedIn` |
| CqlComparisonOperatorsTest | 3 | `1.0 = 1` — Decimal/Integer cross-type equality not yet promoted |
| CqlTypesTest | 1 | Single-quote escape parsing: `\'` not decoded to `'` in string literals |

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

**Eval errors (628 total — unimplemented operators/functions)**

| Suite | Count | Root cause |
|---|---|---|
| CqlDateTimeOperatorsTest | 227 | Date/Time operators: `after`, `before`, `during`, `between`, `Add`/`Subtract` durations |
| CqlIntervalOperatorsTest | 138 | Interval timing operators: `meets`, `overlaps`, `starts`, `ends`, `during` (date precision) |
| CqlStringOperatorsTest | 77 | String functions: `Length`, `Upper`, `Lower`, `Concat`, `Split`, etc. |
| CqlListOperatorsTest | 78 | List functions: `First`, `Last`, `Skip`, `Tail`, `Take`, `Flatten`, `Sort`, `Distinct`, etc. |
| CqlAggregateFunctionsTest | 27 | Aggregate functions: `Count`, `Sum`, `Min`, `Max`, `Avg`, `Median`, etc. |
| CqlNullologicalOperatorsTest | 22 | `IsNull`, `IsTrue`, `IsFalse`, `Coalesce` not yet in `eval/engine.rs` dispatch |
| CqlArithmeticFunctionsTest | 12 | `Truncate`, `Round`, `Ln`, `Exp`, `Log` emitted as `FunctionRef` |
| CqlComparisonOperatorsTest | 33 | Date/Time comparison, Quantity comparison |
| CqlTypeOperatorsTest | 4 | `ToDate`, `ToDateTime`, `ToTime` conversion functions |
| CqlTypesTest | 10 | Time/DateTime value constructors |

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
| hl7_eval_tests | 16 | ⚠️ 41 wrong-answer failures (Comparison 3, Interval 16, List 21, Types 1); 1 412 total expressions evaluated |
| semantic_tests | 11 | ✅ all pass |
| eval_integration_tests | 2 | ✅ all pass |
| **Total** | **924** | **⚠️ 41 hl7 wrong-answer failures; all other tests pass** |

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

### High priority (wrong-answer risk or large eval-error count)

1. **Null-propagation in list operators** — `Contains`, `Includes`, `ProperlyIncludes`, `In`
   need correct three-valued null semantics. Fixes 21 wrong-answer failures in
   `CqlListOperatorsTest`.

2. **Null semantics in interval operators** — `Contains`/`Except`/`In`/`Equal` with null
   endpoints, integer/decimal `Meets` successor arithmetic, DateTime precision in `IncludedIn`.
   Fixes 16 wrong-answer failures in `CqlIntervalOperatorsTest`.

3. **Decimal/Integer cross-type equality** — promote `1.0 = 1` to same type before comparison.
   Fixes 3 wrong-answer failures in `CqlComparisonOperatorsTest`.

4. **String escape decoding** — `\'` should decode to `'` in CQL string literals.
   Fixes 1 wrong-answer failure in `CqlTypesTest`.

5. **System function resolution in emitter** — emit `Abs`, `Ceiling`, `Floor`,
   `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power` as their specific ELM types
   instead of `FunctionRef`. This unblocks 12 arithmetic eval-error tests.

6. **Nullological operator implementation** — add `IsNull`, `IsTrue`, `IsFalse`,
   `Coalesce` to `eval/engine.rs` dispatch. Unblocks 22 eval-error tests.

### Medium priority (large unimplemented areas)

7. **String function implementation** — `Length`, `Upper`, `Lower`, `Concat`,
   `Split`, `StartsWith`, `EndsWith`, `Matches`, `IndexOf`, `Substring`, etc.
   Unblocks 77 string eval-error tests.

8. **List function implementation** — `First`, `Last`, `Skip`, `Tail`, `Take`,
   `Flatten`, `Sort`, `Distinct`, `IndexOf`, etc. Unblocks 78 list eval-error tests.

9. **Aggregate function implementation** — `Count`, `Sum`, `Min`, `Max`, `Avg`,
   `Median`, `Mode`, `StdDev`, `Variance`, etc. Unblocks 27 eval-error tests.

10. **Date/Time operator implementation** — `after`, `before`, `during`, `between`,
    duration arithmetic (`Add`/`Subtract` intervals from dates). Unblocks 227 eval-error tests.

11. **Retrieve execution** — implement data access in the evaluator so that FHIR
    queries can be run end-to-end with an `InMemoryDataProvider`.

12. **Quantity literal support** — lexer/parser extension for `1'cm'` syntax,
    required for clinical quantity comparisons.

### Lower priority

13. **Long literal support** — `1L` syntax for 64-bit integers.
14. **Negative literal representation** — represent `-5` as `Negate(Literal("5"))`
    in ELM output to match the Java reference translator.
15. **ELM XML output** — emit `library.xml` for interop with Java tooling.
16. **Multi-source query evaluation** — complete the `from A, B` join evaluation.
17. **Locator end-position tracking** — emit `"line:start-line:end"` locators.
