# CQL Engine Test Audit

Last audited: 2026-06-19

This report summarizes how `rh-cql` currently tests ELM production and ELM evaluation, how it uses external reference implementations, and where additional CQL fixtures would improve confidence.

## How To Run And Review

Run the focused audit gate from the crate directory:

```bash
cd crates/rh-cql
just audit
```

The recipe runs:

- `cargo test -p rh-cql --test hl7_eval_tests -- --nocapture`
- `cargo test -p rh-cql --test golden_elm_tests --test emit_conformance_tests --test pipeline_comparison_tests`
- `cargo test -p rh-cql --test eval_integration_tests --test semantic_tests --test clinical_age_operators_test --test ratio_message_tree_test`

It writes reviewable output to `crates/rh-cql/conformance/results/audit/`:

- `hl7_eval_tests.txt`
- `elm_production_tests.txt`
- `eval_engine_tests.txt`
- `hl7_eval_summary.json`
- `hl7_eval_summary.md`
- `implementation_matrix.json`
- `implementation_matrix.csv`
- `implementation_matrix_summary.json`

For regression-sensitive runs, use the current HL7 unimplemented-count ceilings:

```bash
cd crates/rh-cql
just audit-strict
```

To populate the Java and JavaScript columns in the same implementation matrix:

```bash
cd crates/rh-cql
just audit-references
```

`just audit-full` runs both phases.

Expanded corpus checks are separate from the HL7 expression matrix:

```bash
cd crates/rh-cql
just corpus-audit-rh
```

They write `corpus_matrix.csv`, `corpus_matrix.json`, `corpus_summary.json`,
`java_pass_rh_fail.csv`, and `java_non_pass.csv` under
`crates/rh-cql/conformance/results/corpus/`.
The current full RH-only corpus baseline is 1 248 files: 334 compile passes and
914 compile errors across generated, CQFramework, Cooking with CQL, and CMS
2025 eCQM sources.
Use `just corpus-audit` for the heavier Java-inclusive reference pass.

To summarize the latest generated audit and corpus outputs:

```bash
cd crates/rh-cql
just audit-summary
```

This writes date-stamped Markdown and JSON summaries under
`crates/rh-cql/conformance/results/summaries/`.

For the current remediation plan based on these summaries, see
[`CQL_REMEDIATION_PLAN.md`](CQL_REMEDIATION_PLAN.md).

For Java CQL-to-ELM comparison, run the one-time setup first:

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

Then compare a checked-in corpus directory:

```bash
cd crates/rh-cql
just elm-reference simple
```

The Java comparison harness writes artifacts under `crates/rh-cql/conformance/results/`.

## Current Verified Results

On 2026-06-21, the focused audit commands passed.

HL7 evaluation suite:

- 16 Rust tests passed.
- 15 official HL7 XML fixture files were discovered and run.
- Current expression-level result: `pass=765`, `fail=0`, `skip=48`, `compile_err=123`, `eval_err=467`, `invalid_fail=16`.
- The suite is a "no wrong answers" gate. Compile errors, eval errors, and skipped cases are counted as unimplemented coverage, but they do not fail CI.
- `just audit` writes generated JSON and Markdown summaries, so future `CONFORMANCE.md` metric updates can come from structured output instead of copied console text.
- The generated implementation matrix is row-per-HL7-test and includes pass/fail/status plus notes columns for `rh-cql`, Java ELM, and JavaScript evaluation.
- The 2026-06-19 summary work fixed total accounting for `invalid_fail`; those cases were previously present but not included in the printed total.

Implementation matrix status counts after `just audit-references`:

| Implementation | Pass | Compile Err | Eval Err | Fail | Skip | Unimplemented | Timeout |
|---|---:|---:|---:|---:|---:|---:|---:|
| `rh-cql` | 772 | 123 | 467 | 0 | 48 | 16 | 0 |
| Java ELM | 1 410 | 16 | 0 | 0 | 0 | 0 | 0 |
| JavaScript `cql-execution` | 594 | 118 | 467 | 81 | 166 | 0 | 0 |

ELM production tests:

- `golden_elm_tests`: 3 passed.
- `emit_conformance_tests`: 19 passed.
- `pipeline_comparison_tests`: 13 passed.

Additional evaluator and semantic tests:

- `eval_integration_tests`: 70 passed.
- `semantic_tests`: 8 passed.
- `clinical_age_operators_test`: 8 passed.
- `ratio_message_tree_test`: 11 passed.

## ELM Production Coverage

Current strengths:

- Golden JSON tests verify deterministic compiler output for a small committed CQL corpus.
- Emitter conformance tests check specific ELM shapes that are known to matter for Java and JavaScript consumers, including native arithmetic/string nodes, result type QName format, expression context propagation, retrieves, and code references.
- Pipeline comparison tests compare fixed corpus properties against checked-in Java ELM for `SimpleTest.cql` and assert known acceptable metadata differences.
- The conformance harness can run the Java `cqframework/clinical_quality_language` translator for ad hoc CQL-to-ELM comparisons.

Gaps:

- The automated Rust test suite does not invoke the Java translator at test time. `just audit-references` is an explicit external-reference step because it needs Java, npm dependencies, and local setup.
- The checked-in Java reference corpus is very small: `simple` and `arithmetic`, plus in-test CQL strings.
- JavaScript reference evaluation currently uses scalar HL7 expression tests and `rh-cql`-generated ELM. FHIR retrieve evaluation with `cql-exec-fhir` and bundles is still future work.
- The Java comparison script and reference matrix harness are useful but not currently a CI gate and need local tool setup.
- Expanded corpus checks currently assert compile/translation status by CQL source file. They do not yet compare normalized ELM structure or evaluate FHIR bundles.

## Evaluation Coverage

Current strengths:

- The HL7 XML suite exercises all 15 official operator/literal categories from `tests.zip`.
- The runner wraps each expression in a CQL library, compiles through the main pipeline, evaluates ELM, and compares parsed expected values.
- Focused Rust integration tests cover core arithmetic, logic, strings, lists, intervals, queries, retrieves with in-memory data, parameters, cross-library expression references, traces, aggregate functions, clinical age, ratios, message, children, and descendants.

Gaps:

- The HL7 gate tolerates 606 unimplemented/error outcomes. This is appropriate for incremental development, but it is not a full conformance pass.
- Retrieve execution is covered by local in-memory tests, not by a reference-evaluator comparison corpus using realistic FHIR bundles.
- Multi-source queries, aggregate query clauses, tuple-heavy output comparisons, date/time/interval edge behavior, quantity unit conversion, Long literals, and invalid-input enforcement remain weak spots.
- Some expected-output parsing still causes skips, so parts of the official suite are not asserting values yet.

## Java Reference Implementation

Use cases covered today:

- CQL-to-ELM production comparison through `crates/rh-cql/conformance/scripts/compare_translators.py`.
- Full HL7 expression-corpus Java translation status through `crates/rh-cql/conformance/scripts/run_reference_matrix.py`.
- Checked-in Java ELM fixture comparison for a small fixed corpus.

Known caveats:

- Java translator metadata differs from `rh-cql` for `localId`, locator ranges, empty arrays, and annotation defaults. These are documented as acceptable differences in `CONFORMANCE.md` and `pipeline_comparison_tests.rs`.
- Java comparison setup is pinned to `cqframework/clinical_quality_language` tag `v4.2.0`; generated setup/comparison metadata records the checked-out Java commit when available.
- The Java translator is an ELM production reference, not an evaluation oracle for `rh-cql` runtime behavior.

## JavaScript Implementation

Local repo status:

- `crates/rh-cql/conformance/scripts/evaluate_with_cql_execution.mjs` runs `cql-execution` against `rh-cql` ELM for the HL7 expression corpus.
- `packages/cql/test/node.test.ts` verifies the Reason Health WASM wrapper compile/evaluate path only.

Relevant upstream projects:

- `cqframework/cql-execution` executes CQL artifacts expressed as JSON ELM.
- `cqframework/cql-exec-fhir` provides a FHIR data source for that execution engine.

Recommended use:

- Treat JavaScript as a consumer/interoperability oracle for generated ELM, not as a replacement for HL7 expected-result tests.
- Use the npm-based conformance harness to compile CQL with `rh cql compile`, load JSON ELM into `cql-execution`, and compare JavaScript results against HL7 expected outputs for scalar expression cases.
- Include `cql-exec-fhir` cases with small R4 bundles once retrieve semantics are stable enough.

Known JavaScript ambiguity:

- JavaScript numeric behavior is constrained by JavaScript `Number`, so decimal precision and integer boundary tests should be marked explicitly when comparing to the CQL specification.
- Date/timezone behavior and FHIRHelpers availability can differ depending on loaded ELM libraries and data-source configuration.

## Recommended New CQL Sources

See [CQL_TEST_CORPUS.md](CQL_TEST_CORPUS.md) for the crate-local corpus
strategy. In short, use HL7 CQL tests for language conformance, CQFramework
translator content for ELM parity, CMS/CQFramework FHIR eCQM content for
realistic FHIR/QI-Core stress, synthetic FHIR R4 bundles for patient-level
evaluation, and generated CQL for boundary cases.

Add these as committed CQL fixtures first, then decide whether each should be evaluated by Rust only, JavaScript only, or both:

1. Boundary numerics: min/max Integer, Long parse failures or support, decimal scale/rounding, divide by zero, overflow/invalid arithmetic, `Log(x, 1)`.
2. Temporal precision: partial Date/DateTime comparisons, timezone offsets, `same as` at precision, daylight/offset normalization, date-only `ToDateTime`.
3. Interval boundaries: open/closed endpoints, null endpoints, adjacent intervals for `meets`, interval `except` normalization, point-from singleton intervals.
4. Quantity and UCUM: same-unit arithmetic, cross-unit comparisons, incompatible units, quantity intervals, precision preservation in `ToString`.
5. Lists with nulls: `contains`, `includes`, `distinct`, `flatten`, sort, empty vs null list behavior, list of tuples.
6. Query complexity: multi-source joins, `let` scope, `with`/`without`, sort by expression, `aggregate` query clause.
7. FHIR retrieves: code, value set, date filters, nested property paths, choice types, and missing field behavior using small R4 bundles.
8. Library boundaries: included libraries with parameters, versioned includes, overloaded functions, forward references, private/public access.
9. Terminology: code equivalence, concept selectors, value set membership, unknown code systems, empty expansions.
10. Error and message behavior: non-error severity messages, error severity propagation, invalid literals, invalid casts, invalid time/date bounds.

## Recommended Next Work

1. Add a JavaScript reference harness using `cql-execution` for simple ELM evaluation and `cql-exec-fhir` for R4 retrieve cases.
2. Expand the committed ELM comparison corpus using the import order in `CQL_TEST_CORPUS.md`.
3. Add CMS 2025 FHIR eCQM content as a realistic compile/ELM corpus before treating it as an evaluation corpus.
4. Lower `just audit-strict` thresholds as unimplemented HL7 outcomes are burned down.
5. Use generated `hl7_eval_summary.json` / `.md` as the source for future `CONFORMANCE.md` metric updates.
