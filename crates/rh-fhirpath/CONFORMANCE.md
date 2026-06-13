# rh-fhirpath Conformance

**Last updated**: 2026-06-12 (harness introduction, baseline recorded)
**FHIRPath specification**: 2.0.0 (http://hl7.org/fhirpath)
**Test suite source**: `tests-fhir-r4.xml` from
https://github.com/FHIR/fhir-test-cases/blob/master/r4/fhirpath/ (R4 copy of
the R5 source of truth)

This is the authoritative conformance document for `rh-fhirpath`. It records
the official HL7 FHIRPath test-suite results and the regression policy. For a
feature-by-feature breakdown see [SPEC_COVERAGE.md](SPEC_COVERAGE.md).

---

## 1. HL7 Official Test Suite

The suite (937 cases) is vendored under
`tests/fixtures/hl7_fhirpath_tests/` together with JSON versions of the input
resources (the XML suite references XML inputs; rh-fhirpath evaluates JSON, so
the equivalent JSON examples from `fhir-test-cases` / hl7.org are used).

Run with:

```bash
cargo test -p rh-fhirpath --test hl7_conformance -- --nocapture
```

Every case is categorized as one of:

| Category | Meaning |
|---|---|
| `pass` | Result matches expected output (or expected-invalid case errored) |
| `wrong_answer` | Evaluation succeeded but produced an incorrect result |
| `parse_error` | Expression failed to parse (and was not expected to) |
| `eval_error` | Evaluation failed (and was not expected to) |
| `skipped` | Missing input fixture (1 case: `patient-name-extensions.json`, not published) |

A machine-readable summary is written to
`target/hl7_fhirpath_conformance.json` on every run.

### 1.1 Baseline results (2026-06-12)

| Metric | Count | % |
|---|---|---|
| Total | 935 | 100% |
| Pass | 564 | 60.3% |
| Wrong answer | 120 | 12.8% |
| Parse error | 79 | 8.4% |
| Eval error | 171 | 18.3% |
| Skipped | 1 | 0.1% |

### 1.2 Regression policy

The harness enforces a **shrink-only wrong-answer baseline**: the 120 baseline
wrong answers are listed in `KNOWN_WRONG_ANSWERS` inside
`tests/hl7_conformance.rs`.

- Any **new** wrong answer fails CI.
- Any baseline entry that starts passing also fails CI until it is removed
  from the list (so the baseline can only shrink).
- Parse/eval errors are currently tolerated and tracked here; the long-term
  goal (refactor plan WS2) is zero wrong answers and a shrinking error count.

> Note: the original plan called for asserting zero wrong answers from day
> one. The measured baseline (120) made that gate impossible without weeks of
> evaluator work, so the rh-cql-style "no regressions, shrink-only baseline"
> gate is used instead.

### 1.3 Expected-invalid semantics

Cases marked `invalid="syntax|semantic|execution|true"` expect an error from a
strict engine. rh-fhirpath is lenient in places; the harness counts a
successful **empty** result for such cases as a pass (error-equivalent), and a
successful **non-empty** result as a wrong answer.

---

## 2. Top failure clusters (baseline)

| Cluster | Cases | Notes |
|---|---|---|
| `lowBoundary()`/`highBoundary()`/`precision()` | 57 | Functions not implemented |
| Comments (`//`, `/* */`) | 7 | Parser does not strip comments |
| Backtick-delimited identifiers / string escapes | ~5 | Parser gaps |
| `convertsToDecimal()`/`toDecimal()` edge cases | 22 | Unknown function + partial semantics |
| `sort()` | 10 | Not implemented |
| `encode()`/`decode()`/`escape()`/`unescape()` | 12 | Not implemented |
| `aggregate()` | 4 | Not implemented |
| `type()` reflection / `is`/`as` on FHIR types | ~50 | Choice-type (`value[x]`) polymorphism + type hierarchy gaps |
| Partial-precision date/time semantics | ~30 | `!=` empty propagation, partial-date `convertsTo*` |
| Quantity conversions (`toQuantity`, calendar units) | ~15 | Unit handling gaps |
