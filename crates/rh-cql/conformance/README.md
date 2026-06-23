# rh-cql Conformance Runbook

Start with [../CONFORMANCE.md](../CONFORMANCE.md) for the current single-page
summary. This directory contains the scripts, fixtures, generated reports, and
external-tool setup used to produce that summary.

## What Each Signal Means

| Signal | Command | What It Answers |
|---|---|---|
| HL7 spec-facing audit | `just audit-strict` | Does `rh-cql` compile/evaluate the official HL7 expression cases without wrong answers? |
| Three-engine matrix | `just audit-full` | How do `rh-cql`, Java CQL-to-ELM, and JavaScript `cql-execution` behave on the same HL7 rows? |
| RH-only source corpus | `just corpus-audit-rh` | Which larger CQL source files compile with `rh-cql` today? |
| Java-inclusive source corpus | `just corpus-audit` | Which source files are Java-passing remediation candidates vs Java-non-pass quarantine rows? |
| Date-stamped rollup | `just audit-summary` | What is the single generated summary for the latest audit/corpus artifacts? |

## Recommended Refresh

From `crates/rh-cql`:

```bash
just audit-full
just corpus-audit-rh
just audit-summary
```

Review:

- `conformance/results/summaries/latest-summary.md`
- `conformance/results/audit/implementation_matrix_summary.json`
- `conformance/results/corpus/corpus_summary.json`

Use the Java-inclusive source corpus only when source validity matters:

```bash
just corpus-audit
```

## External Reference Setup

The Java and JavaScript comparison steps need local tools.

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

The setup pins `cqframework/clinical_quality_language` to `v4.2.0` by default,
builds the Java CQL-to-ELM CLI, installs JavaScript dependencies, and writes
`reference-version.json`.

To test another Java reference version for an experiment:

```bash
CQL_JAVA_REF=v4.7.0 ./scripts/setup.sh
```

## Generated Outputs

| Artifact | Purpose |
|---|---|
| `results/audit/hl7_eval_summary.md` / `.json` | HL7 expression-suite totals |
| `results/audit/implementation_matrix.csv` / `.json` | Row-per-HL7-case `rh-cql`, Java, and JavaScript statuses |
| `results/audit/implementation_matrix_summary.json` | Status counts and JavaScript evaluation categories |
| `results/audit/javascript_value_mismatches.csv` / `.json` | JavaScript mismatch triage with RH-vs-Java ELM diff samples |
| `results/corpus/corpus_matrix.csv` / `.json` | Source-file corpus compile/translation matrix |
| `results/corpus/corpus_summary.json` | Source-file corpus counts by corpus and validity |
| `results/corpus/java_pass_rh_fail.csv` | Java-passing rows that `rh-cql` should treat as remediation candidates |
| `results/corpus/java_non_pass.csv` | Java-non-pass rows quarantined from remediation totals |
| `results/summaries/latest-summary.md` / `.json` | Single generated rollup used by `CONFORMANCE.md` |

Generated reports are ignored by git. Regenerate them instead of hand-editing.

## Source Validity Rules

- Java-passing CQL is a high-value remediation target when `rh-cql` fails.
- Java-non-pass CQL is quarantined until reviewed.
- Explicit invalid/ambiguous examples live in
  `corpus/invalid-or-ambiguous.md` and are not counted as RH remediation
  failures.
- Reduced fixtures are preferred over large copied external libraries.

## Adding Or Reducing Cases

Use [CQL_TEST_CORPUS.md](CQL_TEST_CORPUS.md) for source selection and reduction
rules. In short:

1. Start from generated CSV output.
2. Reduce the source to the smallest useful CQL.
3. Check Java status before treating it as a remediation target.
4. Commit reduced valid fixtures under `corpus/generated/<topic>/`.
5. Commit invalid or ambiguous reduced fixtures under
   `corpus/invalid-or-ambiguous/` and document them in the register.

## Single-File Java ELM Comparison

After setup:

```bash
cd crates/rh-cql/conformance
python3 scripts/compare_translators.py -f test-cases/simple/SimpleTest.cql
```

Or from `crates/rh-cql`:

```bash
just elm-reference simple
```
