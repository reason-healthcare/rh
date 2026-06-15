# rh-fhirpath Conformance

**Last updated**: 2026-06-15 (wave 35: String→temporal boundary dispatch, zero eval-errors achieved)
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

### 1.1 Current results (2026-06-15, wave 35)

| Metric | Count | % |
|---|---|---|
| Total | 935 | 100% |
| Pass | 934 | 99.9% |
| Wrong answer | 0 | 0.0% |
| Parse error | 0 | 0.0% |
| Eval error | 0 | 0.0% |
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
| 2026-06-14 | 909 (97.2%) | 19 | 1 | 5 | Wave 30: quantity unit algebra for `Quantity * Quantity` and cross-dimension `Quantity / Quantity`. Operands are normalized to base UCUM units before composing the result unit (`2.0 'cm' * 2.0 'm' -> 0.040 'm2'`, `4.0 'g' / 2.0 'm' -> 2 'g/m'`). `testQuantity9/10` flipped from eval-error to pass. |
| 2026-06-14 | 913 (97.6%) | 15 | 1 | 5 | Wave 31: `TypedBoolean` and `TypedDateTime` now preserve FHIR primitive provenance for JSON booleans and `dateTime`/`instant` values, choice-type primitive promotion returns typed FHIR values, `type()/is()/as()/ofType()` distinguish `boolean` vs `Boolean` and `FHIR.Patient` correctly, and `getValue()` / `iif()` accept the new typed primitives. Fixed HL7 `testType9/10/11/12/13/14/18/19/21/23/A1/A3/A4` plus the previous `Observation.issued is instant` type gap. |
| 2026-06-14 | 923 (98.7%) | 10 | 0 | 1 | Wave 32: unary `+` parser; `$index` in `select()`/`where()`; `is` returns empty for empty input (spec-correct); `codesystem-example` fixture updated with nested `chol` sub-concepts (fixes `testCombine1`); `TypedObject` variant tracks FHIR complex type names enabling `ofType(HumanName)`, `value is Age`, `value is Quantity` (FHIR complex type hierarchy); `FhirPrimitive` wrapper enables `extension()` on primitive fields (`_birthDate` sibling); `%ext-` variable expansion to StructureDefinition URLs; `conformsTo()` stub function; `as()` errors on multi-element collections (spec-correct). Cleared 5 wrong answers, 1 parse error, 4 eval errors. Baseline shrunk from 15 → 10 wrong answers. |
| 2026-06-14 | 926 (99.0%) | 7 | 0 | 1 | Wave 33: replaced `f64` with `rust_decimal::Decimal` for `FhirPathValue::Number` and `Quantity.value`, preserving trailing-zero precision. Fixed `PrecisionDecimal` (`1.58700.precision() = 5`), `LowBoundaryDecimal11` (`12.500.lowBoundary(4)`), `HighBoundaryDecimal12` (`12.500.highBoundary(4)`). 3 cases removed from baseline. Remaining decimal gaps: `0.0034`/`-0.0034` boundary at precision 1 (expects `0.0`/`-0.0`). |
| 2026-06-15 | 933 (99.8%) | 0 | 0 | 1 | Wave 34: strict polymorphic choice-type access rejects suffixed keys (`valueQuantity` → use `value[x]`); `checkOrderedFunctions` flag prevents order-dependent functions (`skip`/`tail`/`first`/`last`/`orderBy`) on unordered collections (`children()`); `UnorderedCollection` variant tracks order provenance; `boundary_decimal` uses truncation toward zero for values near zero at coarser precision (`0.0034.highBoundary(1)` = `0.0`, `(-0.0034).lowBoundary(1)` = `-0.0`). All 7 remaining wrong answers eliminated. Baseline is now empty (zero wrong answers). |
| 2026-06-15 | 934 (99.9%) | 0 | 0 | 0 | Wave 35: `boundary()` now accepts `String` values and dispatches to temporal boundary functions based on content (date vs dateTime vs time pattern). Date strings produce DateTime boundaries to enable cross-type comparison (`start.lowBoundary() < end.highBoundary()` where start is Date and end is DateTime). Last eval-error (`testPeriodInvariantNew`) eliminated — **zero eval-errors achieved.** Only remaining non-pass is 1 skipped test (missing `patient-name-extensions.json` input fixture). |

### 1.2 Regression policy

The harness enforces a **zero wrong-answer gate**: the `KNOWN_WRONG_ANSWERS`
list in `tests/hl7_conformance.rs` is currently empty.

- Any **new** wrong answer fails CI.
- Any baseline entry that starts passing also fails CI until it is removed
  from the list (so the baseline can only shrink).
- Parse/eval errors are currently tolerated and tracked here; the long-term
- goal (refactor plan WS2) is zero wrong answers and a shrinking error count.

### 1.3 Expected-invalid semantics

Cases marked `invalid="syntax|semantic|execution|true"` expect an error from a
strict engine. rh-fhirpath now rejects choice-type suffixed keys (e.g.
`valueQuantity`) as semantic errors per the FHIRPath spec. The harness counts a
successful **empty** result for such cases as a pass (error-equivalent), and a
successful **non-empty** result as a wrong answer.

The `mode="strict"` and `checkOrderedFunctions="true"` test attributes are
parsed and propagated to `EvaluationContext.strict_mode` and
`EvaluationContext.check_ordered_functions`. Order-dependent functions
(`skip`, `tail`, `first`, `last`, `orderBy`, `reverse`) applied to
`UnorderedCollection` values (e.g. from `children()`) produce a semantic
error when `check_ordered_functions` is true.

---

## 2. Top failure clusters (baseline)

| Cluster | Cases | Notes |
|---|---|---|
| `testPrimitiveExtensions` | 1 | Skipped: missing `patient-name-extensions.json` input fixture (not published by HL7) |
