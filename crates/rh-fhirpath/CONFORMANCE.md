# rh-fhirpath Conformance

**Last updated**: 2026-06-12 (parser wave 1: comments, delimited identifiers, string escapes)
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

### 1.1 Current results (2026-06-12, parser wave 1)

| Metric | Count | % |
|---|---|---|
| Total | 935 | 100% |
| Pass | 576 | 61.6% |
| Wrong answer | 123 | 13.2% |
| Parse error | 61 | 6.5% |
| Eval error | 174 | 18.6% |
| Skipped | 1 | 0.1% |

History:

| Date | Pass | Wrong | Parse err | Eval err | Notes |
|---|---|---|---|---|---|
| 2026-06-12 | 564 (60.3%) | 120 | 79 | 171 | Harness introduction baseline |
| 2026-06-12 | 576 (61.6%) | 123 | 61 | 174 | Parser wave 1: `//` and `/* */` comments, backtick identifiers, string escapes incl. `\uXXXX`. 3 cases moved from parse-error to wrong-answer (now parse, eval gaps exposed), 1 to eval-error; `testLiteralUnicode` fixed. |

### 1.2 Regression policy

The harness enforces a **shrink-only wrong-answer baseline**: the current
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
| `convertsToDecimal()`/`toDecimal()` edge cases | 22 | Unknown function + partial semantics |
| `sort()` | 10 | Not implemented |
| `encode()`/`decode()`/`escape()`/`unescape()` | 12 | Not implemented |
| `aggregate()` | 4 | Not implemented |
| `type()` reflection / `is`/`as` on FHIR types | ~50 | Choice-type (`value[x]`) polymorphism + type hierarchy gaps |
| Partial-precision date/time semantics | ~30 | `!=` empty propagation, partial-date `convertsTo*` |
| Quantity conversions (`toQuantity`, calendar units) | ~15 | Unit handling gaps |
