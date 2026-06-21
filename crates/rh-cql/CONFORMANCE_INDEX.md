# rh-cql Conformance Index

Use this file as the entry point for `rh-cql` conformance, specification
coverage, corpus planning, and generated audit reports.

## Start Here

| Document | Purpose |
|---|---|
| [CONFORMANCE.md](CONFORMANCE.md) | Current conformance state: HL7 eval totals, current gaps, ELM fidelity, parser status, and test suite summary |
| [SPEC_COVERAGE.md](SPEC_COVERAGE.md) | Operator-by-operator and pipeline-stage coverage across parse, semantic analysis, ELM emit, and eval |
| [conformance/README.md](conformance/README.md) | How to set up and run Java reference translator comparisons |
| [conformance/CQL_ENGINE_TEST_AUDIT.md](conformance/CQL_ENGINE_TEST_AUDIT.md) | Audit workflow, generated report locations, Java/JavaScript status, and next work |
| [conformance/CQL_TEST_CORPUS.md](conformance/CQL_TEST_CORPUS.md) | Test corpus strategy: HL7 tests, CQFramework translator content, CMS eCQM content, synthetic bundles, and generated edge cases |

## Generated Reports

Generated reports are not committed. Regenerate them with:

```bash
cd crates/rh-cql
just audit-full
```

Then review:

| Generated artifact | Purpose |
|---|---|
| `conformance/results/audit/hl7_eval_summary.md` | Suite-level HL7 eval summary |
| `conformance/results/audit/hl7_eval_summary.json` | Machine-readable suite-level HL7 eval summary |
| `conformance/results/audit/implementation_matrix.csv` | Row-per-test implementation matrix with `rh-cql`, Java ELM, and JavaScript eval status/notes |
| `conformance/results/audit/implementation_matrix.json` | Machine-readable implementation matrix |
| `conformance/results/audit/implementation_matrix_summary.json` | Status counts for each implementation column |
| `conformance/results/audit/hl7_eval_tests.txt` | Captured HL7 eval test output |
| `conformance/results/audit/elm_production_tests.txt` | Captured ELM production/fidelity test output |
| `conformance/results/audit/eval_engine_tests.txt` | Captured evaluator/semantic test output |
| `conformance/results/corpus/corpus_matrix.csv` | Expanded source-file corpus matrix across generated, CQFramework, Cooking with CQL, and CMS eCQM sources |
| `conformance/results/corpus/corpus_summary.json` | Expanded corpus status counts by corpus |

## Common Workflows

### Current Audit

```bash
cd crates/rh-cql
just audit-strict
```

Use this before changing conformance documentation. It enforces current ceilings
for skip, compile error, eval error, invalid failure, and total unimplemented
HL7 outcomes.

### Three-Engine Matrix

```bash
cd crates/rh-cql
just audit-full
```

This runs the strict Rust audit, then populates Java ELM and JavaScript
`cql-execution` columns in `implementation_matrix.csv` / `.json`.

### Expanded Corpus

```bash
cd crates/rh-cql
just corpus-audit-rh
```

This writes source-file RH compile status reports under
`conformance/results/corpus/`. Use `just corpus-audit` for the heavier
Java-inclusive reference pass.

### Java Reference Translator Setup

```bash
cd crates/rh-cql/conformance
./scripts/setup.sh
```

The setup pins `cqframework/clinical_quality_language` to `v4.2.0` by default
and writes `reference-version.json`.

### Java ELM Comparison

```bash
cd crates/rh-cql
just elm-reference simple
```

Comparison output is written under `conformance/results/`.

## Current Source Of Truth

- Current aggregate status belongs in [CONFORMANCE.md](CONFORMANCE.md).
- Detailed operator coverage belongs in [SPEC_COVERAGE.md](SPEC_COVERAGE.md).
- Corpus/source selection belongs in [conformance/CQL_TEST_CORPUS.md](conformance/CQL_TEST_CORPUS.md).
- Generated per-run facts belong under `conformance/results/` and should be
  regenerated, not hand-edited.

## Notes

- `implementation_matrix.*` is generated in two phases: Rust statuses from
  `just audit-strict`, then Java and JavaScript statuses from
  `just audit-references`.
- Historical wave/baseline notes are intentionally excluded from
  [CONFORMANCE.md](CONFORMANCE.md); keep that document focused on current state.
