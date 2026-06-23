# CQL Engine Test Audit

Last audited: 2026-06-23

This page explains the audit surfaces behind [../CONFORMANCE.md](../CONFORMANCE.md).
Use the top-level conformance page for the current user-facing summary.

## Current Audit Surfaces

| Surface | Main Command | Role |
|---|---|---|
| HL7 expression audit | `just audit-strict` | Spec-facing compile/eval gate with current unimplemented ceilings |
| Three-engine matrix | `just audit-full` | Adds Java CQL-to-ELM and JavaScript `cql-execution` statuses to the HL7 rows |
| Expanded source corpus | `just corpus-audit-rh` | Fast RH-only compile baseline for larger CQL files |
| Java-inclusive source corpus | `just corpus-audit` | Source-validity and Java-pass/RH-fail remediation triage |
| Summary rollup | `just audit-summary` | Writes `results/summaries/latest-summary.*` |

## Latest Spec-Facing Result

The latest generated HL7 summary contains 1 426 parsed expression cases:

| Metric | Count |
|---|---:|
| Correct evaluated results | 1 241 |
| Invalid expressions correctly rejected | 28 |
| Wrong answers | 0 |
| Skips | 45 |
| Compile errors | 89 |
| Eval errors | 23 |
| Invalid expressions incorrectly accepted | 0 |
| Remaining unimplemented outcomes | 112 |

The policy remains: wrong answers fail the audit, while compile errors, eval
errors, skips, and unsupported expected-output cases are tracked as remaining
coverage.

## Latest Three-Engine Result

| Implementation | Rows | Pass | Compile Error | Eval Error | Fail | Skip |
|---|---:|---:|---:|---:|---:|---:|
| `rh-cql` | 1 426 | 1 269 | 89 | 23 | 0 | 45 |
| Java CQL-to-ELM | 1 426 | 1 410 | 16 | 0 | 0 | 0 |
| JavaScript `cql-execution` | 1 426 | 601 | 89 | 502 | 68 | 166 |

JavaScript category counts are recorded in
`results/audit/implementation_matrix_summary.json` and surfaced in
`results/summaries/latest-summary.md`.

## Generated Files

| File | Meaning |
|---|---|
| `results/audit/hl7_eval_summary.json` | Suite and total counts for the HL7 expression audit |
| `results/audit/implementation_matrix.csv` | Row-level `rh-cql`, Java, and JavaScript status/notes |
| `results/audit/implementation_matrix_summary.json` | Aggregate status counts plus JavaScript categories |
| `results/audit/javascript_value_mismatches.json` | Value-mismatch triage and RH-vs-Java ELM diff samples |
| `results/corpus/corpus_summary.json` | Expanded source corpus counts and source-validity counters |
| `results/summaries/latest-summary.md` | Human-readable rollup of the latest generated results |

Generated files are intentionally not committed.

## Recommended Review Loop

```bash
cd crates/rh-cql
just audit-full
just corpus-audit-rh
just audit-summary
```

Then update [../CONFORMANCE.md](../CONFORMANCE.md) only from the generated
summary and matrix outputs.
