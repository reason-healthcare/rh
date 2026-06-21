# rh-cql Conformance

**Last updated**: 2026-06-19
**CQL specification**: 1.5.3 (https://cql.hl7.org)
**Test suite source**: https://cql.hl7.org/tests.html (`tests.zip`)

This document records the current conformance state for `rh-cql`: HL7 CQL
evaluation results, ELM emission fidelity, parser coverage, and the active test
surface. Historical wave deltas are intentionally omitted; generated audit
artifacts under `conformance/results/audit/` are the current source of truth.

---

## 1. HL7 Official Evaluation Test Suite

The HL7 CQL test suite is checked in under
`tests/fixtures/hl7_cql_tests/`. The runner in `tests/hl7_eval_tests.rs` wraps
each expression in a minimal CQL library, compiles through the main pipeline,
evaluates the generated ELM, and compares the result to the expected XML output.

Run the current audit from the crate directory:

```bash
cd crates/rh-cql
just audit
```

For regression-sensitive runs with current unimplemented-count ceilings:

```bash
cd crates/rh-cql
just audit-strict
```

Generated audit artifacts are written to `conformance/results/audit/`:

- `hl7_eval_tests.txt`
- `hl7_eval_summary.json`
- `hl7_eval_summary.md`
- `implementation_matrix.json`
- `implementation_matrix.csv`
- `elm_production_tests.txt`
- `eval_engine_tests.txt`

### 1.1 Current Results

| Metric | Count |
|---|---:|
| Pass | 765 |
| **Fail (wrong answers)** | **0** |
| Skip | 48 |
| Compile err | 123 |
| Eval err | 467 |
| Invalid pass | 7 |
| Invalid fail | 16 |
| Unimplemented (`compile_err + eval_err + invalid_fail`) | 606 |
| Total parsed cases | 1 426 |

Policy:

- Wrong-answer failures fail CI.
- Compile errors, eval errors, skips, and invalid-expression successes are
  counted as unimplemented coverage, not wrong answers.
- `just audit-strict` locks the current ceilings: skip `48`, compile errors
  `123`, eval errors `467`, invalid failures `16`, total unimplemented `606`.
  Lower these ceilings as coverage improves.

### 1.2 Fixture Coverage

All 15 XML fixture files are checked in and run.

| HL7 Category | Fixture file | Status |
|---|---|---|
| Aggregate Functions | `CqlAggregateFunctionsTest.xml` | Current audit |
| Aggregate Operator | `CqlAggregateTest.xml` | Current audit |
| Arithmetic Functions | `CqlArithmeticFunctionsTest.xml` | Current audit |
| Comparison Operators | `CqlComparisonOperatorsTest.xml` | Current audit |
| Conditional Operators | `CqlConditionalOperatorsTest.xml` | Current audit |
| Date/Time Operators | `CqlDateTimeOperatorsTest.xml` | Current audit |
| Errors & Messaging | `CqlErrorsAndMessagingOperatorsTest.xml` | Current audit |
| Interval Operators | `CqlIntervalOperatorsTest.xml` | Current audit |
| List Operators | `CqlListOperatorsTest.xml` | Current audit |
| Logical Operators | `CqlLogicalOperatorsTest.xml` | Current audit |
| Nullological Operators | `CqlNullologicalOperatorsTest.xml` | Current audit |
| String Operators | `CqlStringOperatorsTest.xml` | Current audit |
| Type Operators | `CqlTypeOperatorsTest.xml` | Current audit |
| Types | `CqlTypesTest.xml` | Current audit |
| Value Literals & Selectors | `ValueLiteralsAndSelectors.xml` | Current audit |

### 1.3 Current Gap Summary

Current unimplemented/error outcomes are concentrated in:

| Area | Current signal |
|---|---|
| Date/time operators | Largest eval-error group; duration/difference, precision, and temporal relationship behavior still need burn-down |
| Interval operators | Timing relationships, boundary normalization, and invalid/open-bound cases remain a major gap |
| String functions | Remaining string-function dispatch/evaluation gaps are still visible in the HL7 suite |
| List/query behavior | Multi-source joins, query aggregate behavior, and some list edge cases remain incomplete |
| Value literals/selectors | Tuple, concept, and related selector coverage still has compile errors |
| Quantity/UCUM | Same-unit support exists in places; cross-unit conversion/comparison is intentionally skipped or incomplete |
| Invalid input enforcement | Some `invalid="true"` expressions still evaluate instead of erroring and are counted as `invalid_fail` |

For operator-by-operator implementation state across parse, semantic analysis,
emit, and eval, see [`SPEC_COVERAGE.md`](SPEC_COVERAGE.md).

### 1.4 Implementation Matrix

`implementation_matrix.csv` / `.json` are generated with one row per HL7 test
case and implementation status/notes columns for:

- `rh-cql` evaluator.
- Java ELM translator.
- JavaScript evaluation.

Currently, `rh-cql` statuses are populated from the HL7 runner. Java ELM and
JavaScript columns are explicit `not_run` placeholders until those harnesses are
wired into the same matrix.

---

## 2. ELM Emission Fidelity

The ELM emitter lives in `src/emit/`. Its output is compared against the pinned
Java reference translator (`cqframework/clinical_quality_language` tag `v4.2.0`)
via `conformance/scripts/compare_translators.py`.

Set up the Java reference once:

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

The setup writes `reference-version.json` with the Java translator repository,
ref, and commit. To compare a checked-in corpus directory:

```bash
cd crates/rh-cql
just elm-reference simple
```

### 2.1 Current Coverage

`tests/pipeline_comparison_tests.rs` checks a small fixed corpus:

| Corpus item | What is checked |
|---|---|
| `SimpleTest.cql` | Direct metadata diff against checked-in Java reference ELM (`conformance/test-cases/simple/SimpleTest.json`) |
| `test-0-input` | Retrieve + temporal query structure preserved; translator metadata present |
| `test-2-input` | Retrieve + `First` query structure preserved; translator metadata present |
| `ArithmeticTests.cql` | Native arithmetic node emission (`Add`, `Divide`, `Power`) plus metadata presence |

No high-priority emitter mismatches are currently tracked for this fixed corpus.
The corpus is intentionally small and should be expanded using the strategy in
[`conformance/CQL_TEST_CORPUS.md`](conformance/CQL_TEST_CORPUS.md).

### 2.2 Known Intentional Differences

| Item | Java | rh-cql | Notes |
|---|---|---|---|
| `localId` emission | Only with `--debug` | Always when annotations enabled | Verified by `simple_test_metadata_diff_matches_java_reference` |
| Locator format | `"line:col-line:col"` range | `"line:col"` start only | Needs parser end-position tracking |
| Empty arrays | Includes empty arrays such as `annotation: []` | Omits empty arrays | Verified by `simple_test_metadata_diff_matches_java_reference` |

---

## 3. Parser Conformance

The historical jvmTest parser baseline was 90/119 files passing on 2024-12-10.
A current rerun is pending and should use the pinned Java translator setup.

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
python3 scripts/compare_translators.py --operator-tests --summary-only
```

Known parser stress areas remain:

| Feature area | Notes |
|---|---|
| Complex QDM-specific syntax | Lower priority for FHIR-first work |
| Multi-library CDS/CQM files | Important for realistic corpus expansion |
| Timing expression edge cases | Still visible in realistic and HL7-derived content |

---

## 4. Feature Implementation Status

For the exhaustive operator/grammar inventory, see
[`SPEC_COVERAGE.md`](SPEC_COVERAGE.md). Current high-level state:

### 4.1 Compilation Pipeline

| Feature | Status |
|---|---|
| CQL -> AST parsing (`nom` combinators) | Implemented |
| Semantic analysis (type inference, scope, overload resolution) | Implemented |
| Typed AST (`TypedLibrary`) | Implemented |
| ELM JSON emission | Implemented |
| Source-map generation (CQL span -> ELM node) | Implemented |
| Unified diagnostic system (`Diagnostic` with severity, code, span) | Implemented |
| ELM XML output | Not implemented |
| Compile-to-FHIR-Library | Not implemented |

### 4.2 Evaluation Engine

| Feature | Status |
|---|---|
| Three-valued logic / null propagation | Implemented for covered operators |
| Primitive values (`Integer`, `Decimal`, `String`, `Boolean`) | Implemented |
| Date, DateTime, Time values and arithmetic | Partial |
| Interval values and operations | Partial |
| List values and operations | Partial |
| Tuple values | Partial |
| FHIR data access via `InMemoryDataProvider` | Implemented for local tests |
| Terminology service (`InMemoryTerminologyProvider`) | Implemented for local tests |
| Retrieve execution | Partial; local retrieve tests exist, realistic corpus pending |
| Library includes in evaluation | Partial |
| Query evaluation | Single-source support; multi-source and aggregate gaps remain |

---

## 5. Test Suite Summary

All tests run via `cargo test -p rh-cql`.

| Test binary | Tests | Current result |
|---|---:|---|
| Unit tests (lib) | 788 | Passing |
| `golden_elm_tests` | 3 | Passing |
| `emit_conformance_tests` | 19 | Passing |
| `pipeline_comparison_tests` | 13 | Passing |
| `hl7_eval_tests` | 16 | 0 wrong-answer failures; 765 pass / 1 426 parsed cases; 606 unimplemented outcomes |
| `semantic_tests` | 8 | Passing |
| `eval_integration_tests` | 70 | Passing |
| `clinical_age_operators_test` | 8 | Passing |
| `ratio_message_tree_test` | 11 | Passing |
| Doc tests | 51 | Passing |

Recommended focused commands:

```bash
cd crates/rh-cql
just audit-strict
cargo clippy -p rh-cql --all-targets --all-features -- -D warnings
```

---

## 6. Maintaining Test Fixtures

For source selection and import order, see
[`conformance/CQL_TEST_CORPUS.md`](conformance/CQL_TEST_CORPUS.md).

When a new CQL specification version ships:

1. Download the new test fixtures from the CQL tests page.
2. Replace `tests/fixtures/hl7_cql_tests/*.xml`.
3. Run `cd crates/rh-cql && just audit-strict`.
4. Review generated `hl7_eval_summary.json`, `hl7_eval_summary.md`, and
   `implementation_matrix.*`.
5. Update this document with the current generated totals.
