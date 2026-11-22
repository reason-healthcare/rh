# FHIRPath R5 CLI testlab

The runner exercises `rh fhirpath eval` against the pinned R5 XML suite and its
FHIR JSON inputs. It passes the selected model, starting focus, and strict
and ordering settings to the CLI. The Rust conformance harness checks typed
values, expected error phase, and fixture applicability. [Pinned fixtures](../tests/fixtures/hl7_fhirpath_r5/README.md) · [Conformance results](../CONFORMANCE.md)

```bash
just test-fhirpath-r5-testlab

python3 crates/rh-fhirpath/testlab/run_r5_testlab.py \
  --name isEffectOnLogic1Base --name testNoValueExists

python3 crates/rh-fhirpath/testlab/run_r5_testlab.py \
  --report target/fhirpath-r5-testlab.json --report-format json
```

`--suite` and repeatable `--input-dir` select another suite and its fixtures.
`--rh` accepts a prebuilt CLI. The report includes case-level results and
reasons for unavailable inputs and functions.
