# rh-fsh Conformance

**Last updated**: 2026-06-12 (initial CONFORMANCE.md, corpus expanded to 61 fixtures)
**SUSHI version**: 3.x (FSHOnline corpus reference)
**Test suite**: SUSHI-generated golden file comparison (`tests/sushi_compat.rs`)

This document tracks rh-fsh's compatibility with the SUSHI reference implementation.
For a full feature breakdown, see the table below.

---

## 1. Test Corpus

The SUSHI compatibility test suite compares rh-fsh output against SUSHI-generated
golden files. All fixtures are in `tests/fixtures/`, organized by FSH feature category.

### 1.1 Fixture inventory (2026-06-12)

**Total: 61 fixtures** (plan target was ≥60)

| Category | Count | Coverage notes |
|---|---|---|
| Aliases | 6 | Plain, `$`-prefixed, FHIR/HL7/V2/V3/US Core aliases |
| Code Systems | 2 | Simple codes, hierarchical concept trees |
| Extensions | 3 | Simple, complex (multi-component), race/sub-extension |
| Instances | 5 | Basic, contained, bundle, inline, quantity values |
| Invariants | 4 | Regex, OR/XOR require, string length |
| Logicals | 1 | Custom element definitions |
| Mappings | 1 | Element mapping rules |
| Paths | 6 | Array, caret, extension, reference, sliced, soft-indexing, nested caret |
| Profiles | 5 | Basic, abstract, shared-via-ruleset, multiple in one file, slicing |
| Rule Sets | 6 | Simple reuse, slicing, parameterized (basic, complex), shared properties |
| Rules (Contains) | (in Rules/Contains/) | Slice-contains rules |
| Rules (Flags) | (in Rules/Flags/) | MustSupport, Summary, etc. |
| Rules (other) | 3 | Choice-type only/binding, obeys, add-element |
| Value Sets | 9 | Include/exclude/filter by system, valueset, LOINC, SNOMED, filter operators |

### 1.2 How to run

```bash
# Generate SUSHI golden files (requires Node.js and npx):
./scripts/generate-fsh-goldens.sh

# Run SUSHI compat tests:
cargo test -p rh-fsh --test sushi_compat -- --include-ignored

# Parser unit tests (no SUSHI required):
cargo test -p rh-fsh --lib
```

Parser unit tests (68 cases) cover every entity type and rule form without requiring
SUSHI.

---

## 2. Feature Coverage

Status: ✅ implemented · 🟡 partial · ❌ not implemented

### Entities

| Feature | Status | Notes |
|---|---|---|
| Profile | ✅ | Metadata + all rule types |
| Extension | ✅ | Simple and complex (sub-extension slicing) |
| Logical | ✅ | Custom element definitions via AddElement |
| Resource | ✅ | |
| Instance | ✅ | Including inline, contained, Bundle entries |
| ValueSet | ✅ | Include/exclude, filter operators, concept lists |
| CodeSystem | ✅ | Concepts, displays, definitions, hierarchy |
| Invariant | ✅ | expression, xpath, severity |
| Mapping | ✅ | Mapping rules with arrow notation |
| RuleSet | ✅ | Non-parameterized |
| ParamRuleSet | 🟡 | Basic `{param}` substitution; complex nested templates partial |
| Alias | ✅ | `$`-prefixed and plain names |

### Rules

| Rule | Status | Notes |
|---|---|---|
| Cardinality (`x..y`) | ✅ | With trailing flags |
| Flag (`MS`, `SU`, `?!`, `N`, `TU`, `D`) | ✅ | Single and multi-path |
| Binding (`from valueSet (strength)`) | ✅ | URL and alias binding targets |
| Assignment (`= value`) | ✅ | All FshValue types |
| Contains (slicing) | ✅ | Named and URL-form, with cardinality and alias |
| Only (`only Type or Type`) | ✅ | Including Reference() types |
| Obeys | ✅ | With and without path |
| Caret (`^path = value`) | ✅ | Element-level and standalone; nested caret paths |
| Insert (RuleSet) | ✅ | Non-parameterized |
| Insert (ParamRuleSet) | 🟡 | Basic substitution; deeply nested params partial |
| AddElement | ✅ | Min, max, types, short, definition |
| PathRule | ✅ | Bare path for context setting |
| InstanceRule (Assignment) | ✅ | |
| InstanceRule (Insert) | ✅ | |

### Paths

| Path Form | Status | Notes |
|---|---|---|
| Simple element paths | ✅ | |
| Numeric array indexing (`[0]`) | ✅ | Represented as Slice internally |
| Soft indexing (`[+]`, `[=]`) | ✅ | |
| Slice names (`identifier[MRN]`) | ✅ | |
| Choice types (`value[x]`) | ✅ | |
| Extension paths | ✅ | |
| Caret paths (`^element`) | ✅ | Including nested (`^contact.name`) |
| Reference paths | ✅ | |

### Values

| Value | Status | Notes |
|---|---|---|
| String | ✅ | With escape sequences |
| Boolean | ✅ | |
| Integer | ✅ | |
| Decimal | ✅ | |
| Date / DateTime | ✅ | |
| Code (`#code`) | ✅ | |
| Coded (`system#code "display"`) | ✅ | URL and alias systems |
| Quantity (`n 'unit'`) | ✅ | UCUM single-quote units parsed and stripped |
| Reference | ✅ | |
| Canonical | ✅ | |
| InstanceRef (inline) | ✅ | `contained[+] = instance` |
| Ratio | ❌ | Not yet in FSH lexer |

---

## 3. Known Gaps

| Gap | Impact | Notes |
|---|---|---|
| Ratio value literals | Low | `numerator : denominator` syntax not lexed |
| `defineVariable` in rules | Low | FHIRPath 2.0 feature, not in SUSHI 3.x either |
| Deep ParamRuleSet template nesting | Medium | Multi-level `{param}` inside nested rules |
| SUSHI golden-file CI | Blocked | Requires Node.js + SUSHI in CI environment; tests are `#[ignore]` |
| Parser unit tests for SUSHI compat assertions | N/A | 68 parser unit tests cover entity/rule/path/value forms |
