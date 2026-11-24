# rh-fhirpath conformance

RH implements the published FHIRPath 3.0.0 STU 3 language and provides FHIR
R4 and R5 logical models. Evaluation defaults to R4. Use the model selection
on the Rust context, CLI, or WASM evaluation options for R5. FHIR profile and
terminology functions use services supplied by the host. [FHIRPath 3.0.0](https://hl7.org/fhirpath/STU3/en/index.html) ·
[FHIR R4](https://hl7.org/fhir/R4/fhirpath.html) · [FHIR R5](https://hl7.org/fhir/R5/fhirpath.html)

## Typed JSON compatibility

FHIRPath preserves JSON decimal scale and large exponents using the serde_json
`arbitrary_precision` feature. Cargo unifies this feature with other consumers
of the same serde_json dependency in a build. Typed deserialization of a nested
R4 `Extension.valueDecimal` and CQL ELM `Quantity.value` can fail when serde
buffers the number inside a flattened or tagged field. This compatibility issue
affects builds that combine those typed parsers with `rh-fhirpath`.

## Pinned suites

| Suite | Cases | Passed | Documented differences | Outside test profile | Missing fixtures |
| --- | ---: | ---: | ---: | ---: | ---: |
| Vendored R4 | 935 | 925 | 8 | 2 | 0 |
| Upstream R5 at `469d90ab` | 1,381 | 1,214 | 29 | 138 | 0 |

The R5 suite and 20 matching JSON inputs are pinned to FHIR/fhir-test-cases
revision `469d90ab58663266d5d7ff82ba3587257fd8a4be`. The inputs include
five JSON equivalents of XML Parameters resources. The native harness retains
typed values, uses the suite's starting context and model, checks result
ordering when specified, and distinguishes syntax errors from evaluator errors
for R5. The R4 harness retains its historical expected-invalid handling.
[Pinned fixture provenance](tests/fixtures/hl7_fhirpath_r5/README.md) ·
[HL7 test guidance](https://hl7.org/fhirpath/STU3/en/tests.html)

The 29 R5 differences cover expectations such as prototype sort syntax,
timezone defaults on System.DateTime boundaries, older distinct ordering,
and FHIR R5 `hasValue()`, `htmlChecks()`, and `conformsTo()` behavior.
The gate checks case IDs and outcomes, and detects stale entries. [R4 differences](tests/fixtures/hl7_fhirpath_tests/known-differences.tsv) ·
[R5 differences](tests/fixtures/hl7_fhirpath_r5/known-differences.tsv)

The R5 run has 115 custom document-function cases, 15 static type-checking
cases, three CDA cases, three draft terminology client cases, and two cases
requiring a profile-validation host outside the native test profile. The R4
suite has the same two host-dependent profile cases. Validator integration tests
exercise actual registered profiles and terminology-backed invariants.
[Static and run-time evaluation](https://hl7.org/fhirpath/STU3/en/index.html#type-safety-and-strict-evaluation) ·
[Validator host tests](../rh-validator/tests/fhirpath_host.rs)

## Reproduce

```bash
cargo test --locked -p rh-fhirpath --test hl7_conformance -- --nocapture
cargo test --locked -p rh-validator --test fhirpath_host
cargo test --locked -p rh-cli --test fhirpath_integration_test
just test-fhirpath-r5-testlab
wasm-pack test --node crates/rh-fhirpath --features wasm --lib --locked
```

The native suite writes `target/hl7_fhirpath_conformance.json` and
`target/hl7_fhirpath_r5_conformance.json`. Set `HL7_CONFORMANCE_OUT` or
`HL7_R5_CONFORMANCE_OUT` to change their paths. The CLI testlab report
shows the result from the standalone command for each selected case.

## Capability limits

- Decimal arithmetic uses the fixed precision and range of `rust_decimal`.
  Arbitrary-exponent FHIR JSON values remain available through their source
  lexemes for supported comparison and string conversion. Arithmetic that
  exceeds the supported representation yields an empty result.
- UCUM support covers the units and conversions in the implemented converter,
  including common metric, time, affine temperature, and simple compound units.
  [Unit support](units.md)
- RH evaluates types at runtime and implements selected strict navigation
  checks. The validator adapter supplies an R4 profile registry and terminology.
  R5 callers can attach a `FhirHost` for external resolution, validation,
  element definitions, slicing, and terminology. [FHIR host contract](src/extensions/fhir/host.rs)
- The optional draft FHIR terminology, factory, and server APIs require
  capabilities beyond the native runtime profile. [FHIR R5 extensions](https://hl7.org/fhir/R5/fhirpath.html)

See [SPEC_COVERAGE.md](SPEC_COVERAGE.md) for the engine and host coverage.
