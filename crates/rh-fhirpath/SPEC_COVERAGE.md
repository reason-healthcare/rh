# rh-fhirpath Specification Coverage

**Last updated**: 2026-06-12 (derived from the HL7 suite baseline run, see
[CONFORMANCE.md](CONFORMANCE.md))

Status legend: ✅ implemented & passing suite cases · 🟡 implemented with known
wrong answers · ❌ not implemented.

## Syntax

| Feature | Status | Notes |
|---|---|---|
| Path navigation, indexers | ✅ | |
| Literals (boolean, string, integer, decimal, date, dateTime, time, quantity) | 🟡 | Partial-precision dates (`@2015`, `@2015-02`) and some quantity literals misbehave |
| Comments `//`, `/* */` | ✅ | Stripped by preprocessor (2026-06-12) |
| Backtick identifiers `` `given` `` | ✅ | (2026-06-12) |
| String escapes | ✅ | Full escape set incl. `\uXXXX` (2026-06-12) |
| Environment variables (`%resource`, `%context`) | 🟡 | `%vs-*`/`%ext-*` not supported |

## Operators

| Feature | Status | Notes |
|---|---|---|
| Arithmetic (`+ - * / div mod`) | 🟡 | A few division/mod edge cases (empty propagation) |
| Equality `=`, `!=` | 🟡 | Partial-precision date comparison must return empty, returns boolean |
| Equivalence `~`, `!~` | 🟡 | ~9 wrong answers (decimal precision rounding, list order) |
| Comparison `< <= > >=` | 🟡 | Quantity/calendar-duration comparisons partially wrong |
| Boolean logic (`and or xor implies`) | 🟡 | 3-valued logic gaps with empty operands |
| Union `\|` | 🟡 | Duplicate-elimination edge cases |
| `is` / `as` (operators and functions) | 🟡 | FHIR type hierarchy + choice-element (`value[x]`) polymorphism gaps |
| String concatenation `&` | ✅ | |

## Functions

| Group | Status | Notes |
|---|---|---|
| Existence (`empty exists all allTrue anyTrue allFalse anyFalse count distinct isDistinct subsetOf supersetOf`) | ✅ | minor edge cases |
| Filtering/projection (`where select repeat ofType`) | 🟡 | `repeat()` ordering issues |
| Subsetting (`single first last tail skip take intersect exclude`) | ✅ | |
| Combining (`union combine`) | 🟡 | 1 wrong answer |
| Conversion (`toBoolean toInteger toDecimal toString toQuantity toDate toDateTime toTime` + `convertsTo*`) | 🟡 | `convertsToDecimal` missing; partial-precision and quantity conversions wrong |
| String (`indexOf substring startsWith endsWith contains upper lower replace matches replaceMatches length toChars trim split join`) | 🟡 | regex single-line mode, some empty-propagation cases |
| String escapes (`encode decode escape unescape`) | ❌ | Not implemented (12 cases) |
| Math (`abs ceiling exp floor ln log power round sqrt truncate`) | 🟡 | `exp`/`power` edge cases |
| Tree navigation (`children descendants`) | ✅ | |
| Utility (`trace now today timeOfDay`) | 🟡 | `now()` precision cases |
| `iif` | 🟡 | short-circuit / criterion-type edge cases |
| `aggregate` | ❌ | Not implemented |
| `defineVariable` | ❌ | Not implemented |
| `sort` | ❌ | Not implemented (10 cases; R5/2.0 addition) |
| `lowBoundary` / `highBoundary` / `precision` | 🟡 | Implemented (2026-06-12); trailing-zero decimal literals limited by `f64` representation |
| Type reflection (`type()`) | ❌ | Not implemented (13+ cases) |
| `comparable` | ❌ | Not implemented |
| `conformsTo` | ❌ | Not implemented (needs validator hook) |
| `resolve()` | ❌ | Needs pluggable resolver trait (no network in core) |
| `memberOf()` | ❌ | Needs pluggable terminology trait |

## Implementation priority (refactor plan task 2.3)

1. ~~Parser: comments, backtick identifiers, unicode escapes~~ ✅ done 2026-06-12.
2. ~~`lowBoundary()`/`highBoundary()`/`precision()`~~ ✅ done 2026-06-12 (f64 caveat).
3. `aggregate()`, `defineVariable()`, `sort()`.
4. String `encode/decode/escape/unescape`, `convertsToDecimal`.
5. Choice-type polymorphism for `is`/`as`/`ofType` on FHIR resources.
6. Partial-precision date/time semantics (empty-returning comparisons).
7. `type()` reflection; `comparable()`.
8. `resolve()` + `memberOf()` behind pluggable traits.
