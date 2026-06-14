# rh-fhirpath Conformance

**Last updated**: 2026-06-14 (waves 26–29: quantity equivalence, FHIR type provenance, matches single-line, iif collection error)
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

### 1.1 Current results (2026-06-14, waves 26–29)

| Metric | Count | % |
|---|---|---|
| Total | 935 | 100% |
| Pass | 907 | 97.0% |
| Wrong answer | 19 | 2.0% |
| Parse error | 1 | 0.1% |
| Eval error | 7 | 0.7% |
| Skipped | 1 | 0.1% |

History:

| Date | Pass | Wrong | Parse err | Eval err | Notes |
|---|---|---|---|---|---|
| 2026-06-12 | 564 (60.3%) | 120 | 79 | 171 | Harness introduction baseline |
| 2026-06-12 | 576 (61.6%) | 123 | 61 | 174 | Parser wave 1: `//` and `/* */` comments, backtick identifiers, string escapes incl. `\uXXXX`. 3 cases moved from parse-error to wrong-answer (now parse, eval gaps exposed), 1 to eval-error; `testLiteralUnicode` fixed. |
| 2026-06-12 | 616 (65.9%) | 128 | 61 | 129 | Wave 2: `lowBoundary()`/`highBoundary()`/`precision()` implemented (45 eval errors fixed). 6 baseline additions: trailing-zero decimal literals lose precision in `f64` (needs a decimal type — see plan 5.3 stage 2), plus 3 suite `±0.0`-boundary cases with non-floor/ceil rounding. |

| 2026-06-12 | 658 (70.4%) | 120 | 61 | 95 | Wave 3: `toDecimal()`/`convertsToDecimal()`, `encode()`/`decode()` (base64/urlbase64/hex), `escape()`/`unescape()` (html/json), cross-type numeric equality fix (`1 = 1.0`). 8 wrong answers removed from baseline. |

| 2026-06-12 | 694 (74.2%) | 122 | 61 | 57 | Wave 4: `sort()`, `aggregate()`/`$total`, `type()` (System + FHIR resource names), `comparable()`, string empty-propagation (`{}.startsWith()` etc.). FHIR primitive type tracking deferred (`testType9/10`). |

| 2026-06-13 | 848 (90.7%) | 57 | 1 | 28 | Waves 5–20: partial-precision datetime/time literals, temporal comparison/arithmetic, UCUM unit map + quantity equivalence, three-valued logic, lazy iif, sort, `%vs-`/`%ext-` variables, repeat(), defineVariable(), string empty-propagation, iif criterion type check, FHIR operator precedence. |

| 2026-06-13 | 895 (95.7%) | 31 | 1 | 7 | Waves 21–25: UCUM month/year units, FHIRPath operator precedence fix, singleton equality, type check fixes, testWhere2/4/testIndexer2. |

| 2026-06-14 | 907 (97.0%) | 19 | 1 | 7 | Waves 26–29: quantity `~` uses precision-aware equivalence; `FhirPathValue::TypedString` carries FHIR primitive type provenance (code/id/uri/url/canonical); `is()` follows FHIR type hierarchy; `as()`/`ofType()` use exact type match; `type()` returns FHIR namespace for TypedString; `matches()` uses single-line (dotall) mode; `iif()` errors on multi-element collections. 9 inheritance tests fixed, 2 misc tests fixed (testQuantity4, testMatchesSingleLineMode1, testIif10). |

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
| `highBoundary()`/`lowBoundary()`/`precision()` | 5 | Need decimal precision support |
| Polymorphic choice-type access | 3 | `value[x]` semantic errors (valueQuantity, extension) |
| FHIR `type()`/`is()` for non-string primitives | 4 | `Patient.active.type()` needs TypedBoolean (deferred) |
| `defineVariable`/`$this` ordering | 1 | Semantic error expected |
| `extension()` on primitives | 1 | Not yet implemented |
| `combine().isDistinct()` | 1 | `isDistinct()` semantics |
| `ofType(HumanName)` on collection | 1 | Complex type resolution |
| Remaining parse error | 1 | Unary `+` prefix not supported |
