# FHIRPath R5 Testlab Runner

This directory contains a dev-only runner for `tests-fhir-r5.xml`. It is not
part of the `rh` CLI. The runner shells out to `rh fhirpath eval` for each XML
test and writes a fresh report.

```bash
# Run the full suite and write Markdown
python3 crates/rh-fhirpath/testlab/run_r5_testlab.py \
  --report target/fhirpath-r5-testlab-report.md

# Run selected tests by name
python3 crates/rh-fhirpath/testlab/run_r5_testlab.py \
  --name testLiteralTrue --name testPolymorphismB

# JSON report for scripting
python3 crates/rh-fhirpath/testlab/run_r5_testlab.py \
  --report target/fhirpath-r5-testlab-report.json \
  --report-format json

```

Rules:

- Tests with `invalid=...` expect `rh fhirpath eval` to fail. Any `<output>`
  values on those tests are ignored.
- Unsupported syntax/functions are classified as `not_implemented`.
- Missing JSON input fixtures are classified as `skipped`.
- The report includes rationale for skipped and not implemented cases so
  expected gaps are visible without relying on the historical JSON report.
