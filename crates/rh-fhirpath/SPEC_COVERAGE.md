# rh-fhirpath Specification Coverage

**Last updated**: 2026-06-14 (derived from the HL7 suite run at 909/935 pass; see
[CONFORMANCE.md](CONFORMANCE.md))

Status legend: `✅` implemented and passing suite cases, `🟡` implemented with
known remaining failures, `❌` not implemented.

## Syntax

| Feature | Status | Notes |
|---|---|---|
| Path navigation, indexers | ✅ | Core navigation is stable; `$index` invocation is still missing |
| Literals (boolean, string, integer, decimal, date, dateTime, time, quantity) | 🟡 | Unary `+` prefix still fails (`+1 < +2`); decimal trailing-zero precision still limited by `f64` |
| Comments `//`, `/* */` | ✅ | |
| Backtick identifiers `` `given` `` | ✅ | |
| String escapes | ✅ | |
| Environment variables (`%resource`, `%context`, `%vs-*`) | 🟡 | `%ext-*` works as variables, but primitive `extension()` handling is incomplete |

## Operators

| Feature | Status | Notes |
|---|---|---|
| Arithmetic (`+ - * / div mod`) | 🟡 | Quantity multiplication/division unit algebra now handles `m2` and `g/m`; decimal-boundary precision gaps remain |
| Equality `=`, `!=` | ✅ | Partial-precision temporal semantics are in place |
| Equivalence `~`, `!~` | 🟡 | Remaining decimal-boundary precision cases need non-`f64` precision retention |
| Comparison `< <= > >=` | 🟡 | One remaining period-boundary eval error depends on decimal boundary precision |
| Boolean logic (`and or xor implies`) | ✅ | Three-valued logic implemented |
| Union `\|` | ✅ | Distinctness semantics implemented |
| `is` / `as` (operators and functions) | 🟡 | Non-string FHIR primitive provenance still missing (`Boolean`, etc.) |
| String concatenation `&` | ✅ | |

## Functions

| Group | Status | Notes |
|---|---|---|
| Existence (`empty exists all allTrue anyTrue allFalse anyFalse count distinct isDistinct subsetOf supersetOf`) | 🟡 | `combine().isDistinct()` still has one wrong answer |
| Filtering/projection (`where select repeat ofType`) | 🟡 | `ofType(HumanName)` still has one complex-type resolution gap |
| Subsetting (`single first last tail skip take intersect exclude`) | 🟡 | `children().skip(1)` ordering mismatch remains |
| Combining (`union combine`) | 🟡 | `combine()` itself works; one downstream distinctness case remains |
| Conversion (`toBoolean toInteger toDecimal toString toQuantity toDate toDateTime toTime` + `convertsTo*`) | ✅ | |
| String (`indexOf substring startsWith endsWith contains upper lower replace matches replaceMatches length toChars trim split join`) | ✅ | |
| String escapes (`encode decode escape unescape`) | ✅ | |
| Math (`abs ceiling exp floor ln log power round sqrt truncate`) | ✅ | |
| Tree navigation (`children descendants`) | ✅ | |
| Utility (`trace now today timeOfDay`) | ✅ | |
| `iif` | ✅ | Lazy evaluation, type checks, and multi-item criterion handling are implemented |
| `aggregate` | ✅ | |
| `defineVariable` | 🟡 | Core support exists; one strict-ordering case remains |
| `sort` | ✅ | |
| `lowBoundary` / `highBoundary` / `precision` | 🟡 | 6 decimal precision failures remain because trailing zeros are lost in `f64` |
| Type reflection (`type()`) | 🟡 | String-like FHIR primitive provenance implemented; boolean provenance still missing |
| `comparable` | ✅ | |
| `conformsTo` | ❌ | Needs validator hook |
| `resolve()` | ❌ | Needs pluggable resolver trait |
| `memberOf()` | ❌ | Needs pluggable terminology trait |

## Remaining implementation priority

1. Primitive extensions and primitive provenance beyond strings:
   `extension()` on `_primitive` siblings, `TypedBoolean`, and the remaining
   `is()/type()` failures.
2. Decimal-precision retention beyond `f64`:
   `lowBoundary()`, `highBoundary()`, `precision()`, and the remaining period
   invariant error all depend on preserving trailing zeros exactly.
3. Core evaluator semantics:
   `$index`, `children()` ordering, `combine().isDistinct()`,
   `ofType(HumanName)`, and the unary `+` parser case.
4. Pluggable external hooks:
   `conformsTo()`, `resolve()`, and `memberOf()`.
