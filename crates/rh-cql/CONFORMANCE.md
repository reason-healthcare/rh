# rh-cql Conformance

Last updated: 2026-06-23

This is the current high-level conformance summary for `rh-cql`. It answers
three questions:

1. How close are we to the CQL language tests?
2. How do we compare with the Java CQL-to-ELM translator?
3. How does our generated ELM behave in JavaScript `cql-execution`?

Generated per-run artifacts live under `conformance/results/`. This document is
the canonical human-facing summary. [README.md](README.md) has the very short
crate-level snapshot, [SPEC_COVERAGE.md](SPEC_COVERAGE.md) has the detailed
operator-by-operator map, and [conformance/README.md](conformance/README.md)
has the runbook for regenerating these numbers.

## One-Page Status

The strongest spec-facing signal is the HL7 CQL expression test suite:

| Signal | Result |
|---|---:|
| HL7 parsed expression cases | 1 426 |
| Correct evaluated results | 1 241 |
| Invalid expressions correctly rejected | 28 |
| Wrong answers | 0 |
| Skipped expected-output/expression cases | 45 |
| Compile errors | 89 |
| Runtime eval errors | 23 |
| Invalid expressions incorrectly accepted | 0 |
| Remaining unimplemented outcomes | 112 |

Interpretation:

- `rh-cql` currently has no known wrong-answer failures in the HL7 suite.
- The remaining spec-facing work is mostly compile coverage and eval coverage,
  not incorrect evaluated answers.
- Invalid-input enforcement is clean in this suite: invalid expressions are not
  being accepted as successes.

## Three-Engine Matrix

The same 1 426 HL7 rows are also used as an implementation comparison matrix.

| Implementation | What It Measures | Pass | Compile Error | Eval Error | Fail | Skip |
|---|---|---:|---:|---:|---:|---:|
| `rh-cql` | Compile and evaluate with Reason Health | 1 269 | 89 | 23 | 0 | 45 |
| Java CQL-to-ELM | Translate CQL to ELM with CQFramework Java | 1 410 | 16 | 0 | 0 | 0 |
| JavaScript `cql-execution` | Execute `rh-cql` ELM in JS | 601 | 89 | 502 | 68 | 166 |

Important distinction:

- Java is used as the reference CQL-to-ELM translator.
- JavaScript is used as an ELM consumer/interoperability check.
- A JavaScript failure can mean an `rh-cql` ELM shape issue, a `cql-execution`
  runtime limitation, or an expected-output mismatch. It is not automatically a
  CQL source-validity finding.

### JavaScript Categories

The JavaScript runner classifies every non-simple result so the failures are
reviewable:

| JavaScript category | Count | Meaning |
|---|---:|---|
| `pass` | 584 | Expected value matched in JavaScript |
| `invalid_rejected` | 17 | Invalid input was rejected during JS evaluation |
| `unsupported_expected_output` | 166 | HL7 expected output is not yet parsed by the harness |
| `rh_compile_failure` | 89 | `rh-cql` did not produce ELM for the row |
| `js_runtime_error` | 502 | `cql-execution` raised at runtime |
| `value_mismatch` | 55 | JavaScript ran but returned a different value |
| `invalid_accepted` | 13 | Invalid input evaluated when it should not have |

Current value-mismatch triage found no rows where Java ELM passes JavaScript
while `rh-cql` ELM fails JavaScript. The current 55 value mismatches also fail
with Java ELM in JavaScript, so they are categorized for JavaScript engine or
expected-output review rather than immediate `rh-cql` ELM-shape remediation.

## Expanded Source Corpus

The HL7 matrix is expression-level. The expanded corpus is source-file-level
and is used to stress larger libraries, real-world authoring patterns, includes,
FHIR/QI-Core paths, and CMS eCQM content.

The latest fast RH-only corpus audit covers 1 249 CQL files:

| Corpus | Files | RH Pass | RH Compile Error | Source Validity |
|---|---:|---:|---:|---|
| Generated fixtures | 8 | 7 | 1 | Java not run in RH-only audit |
| Invalid/ambiguous register | 1 | 0 | 1 | Quarantined |
| CQFramework jvmTest | 358 | 147 | 211 | Java not run in RH-only audit |
| CQFramework examples | 34 | 5 | 29 | Java not run in RH-only audit |
| Cooking with CQL | 732 | 126 | 606 | Java not run in RH-only audit |
| CMS 2025 eCQM | 116 | 23 | 93 | Java not run in RH-only audit |
| **Total** | **1 249** | **308** | **941** | **1 quarantined** |

Corpus validity policy:

- Java-passing source rows are remediation candidates.
- Java-non-pass rows are quarantined until manually reviewed.
- Explicit invalid/ambiguous fixtures are kept in
  `conformance/corpus/invalid-or-ambiguous.md` and are not counted as RH
  remediation failures.
- The RH-only corpus audit intentionally marks Java status as `not_run`; run
  `just corpus-audit` when Java-inclusive source validity is needed.

## What Is Considered Current

Use these generated files for exact row-level detail:

| Artifact | Purpose |
|---|---|
| `conformance/results/summaries/latest-summary.md` | Single generated audit/corpus summary |
| `conformance/results/audit/hl7_eval_summary.json` | HL7 suite totals and per-suite details |
| `conformance/results/audit/implementation_matrix.csv` | Row-per-HL7-case `rh-cql`, Java, and JavaScript statuses |
| `conformance/results/audit/implementation_matrix_summary.json` | Status and JavaScript category counts |
| `conformance/results/corpus/corpus_summary.json` | Expanded source-file corpus counts |
| `conformance/results/corpus/java_pass_rh_fail.csv` | Java-passing source files that `rh-cql` does not compile |
| `conformance/results/corpus/java_non_pass.csv` | Quarantined Java-non-pass source files |

## Reproduce The Summary

From `crates/rh-cql`:

```bash
just audit-full
just corpus-audit-rh
just audit-summary
```

Use the heavier Java-inclusive corpus pass selectively:

```bash
just corpus-audit
```

For the local commit gate:

```bash
just check
```

## Remaining Work

At a high level, the remaining conformance work is:

- Reduce the 89 HL7 compile errors.
- Reduce the 23 HL7 runtime eval errors.
- Expand Java-inclusive corpus triage when source validity matters.
- Add more FHIR bundle evaluation cases once retrieve/query semantics are ready.
- Continue using JavaScript results as ELM interoperability evidence rather
  than as standalone CQL source validation.
