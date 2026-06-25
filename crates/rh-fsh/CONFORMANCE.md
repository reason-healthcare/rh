# rh-fsh Conformance

**Last updated**: 2026-06-25 (metadata/CLI wave)
**SUSHI version**: 3.19.0 locally (`fsh-sushi`; implements FSH 3.0.0)
**Test suites**:
- Fixture golden comparison (`tests/sushi_compat.rs`)
- Project comparison runner (`conformance/scripts/run_sushi_comparison.py`)

This document tracks rh-fsh's compatibility with the SUSHI reference implementation.
For a full feature breakdown, see the table below.

---

## 1. Current Results

### 1.1 Project-level SUSHI comparison

The project runner clones real IG repositories, runs reference `fsh-sushi`, runs
`rh-fsh` through `rh --format json fsh compile`, normalizes expected volatile
fields, and compares resources by `(resourceType, id)`.

Latest run:

```bash
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py --timeout-seconds 600
```

Report files:
- `crates/rh-fsh/conformance/results/latest-summary.md`
- `crates/rh-fsh/conformance/results/latest-summary.json`

| Project | Status | Threshold | FSH files | SUSHI resources | rh-fsh resources | Missing | Extra | Mismatch |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| CARIN Blue Button | different | pass | 71 | 134 | 134 | 22 | 22 | 112 |
| mCODE | different | pass | 57 | 350 | 350 | 0 | 0 | 329 |
| Da Vinci CRD | different | pass | 69 | 85 | 86 | 7 | 8 | 74 |
| Da Vinci DTR | different | pass | 39 | 75 | 80 | 0 | 5 | 64 |
| Da Vinci PAS | different | pass | 20 | 158 | 161 | 14 | 17 | 142 |
| IPS | different | pass | 123 | 118 | 215 | 0 | 97 | 117 |

Baseline thresholds are intentionally set to the current post-wave counts so
regressions are visible while the project-level comparison remains non-blocking.
The dependency/profile resolution wave reduced mCODE missing/extra resources
from 83/82 to 1/0 and reduced Da Vinci DTR missing/extra resources from 2/6 to
1/5. The metadata/CLI wave now emits project `ImplementationGuide` resources,
which improves missing counts by one across the default project set. The new IG
resources currently add one mismatch per project because detailed IG metadata
such as contact/page/global fields is not yet SUSHI-identical.

Categorized latest results:

| Project | Resource identity | JSON shape | StructureDefinition | Metadata | Terminology | IG generation | Other |
|---|---:|---:|---:|---:|---:|---:|---:|
| CARIN Blue Button | 44 | 47 | 16 | 13 | 12 | 1 | 23 |
| mCODE | 0 | 270 | 53 | 3 | 1 | 1 | 1 |
| Da Vinci CRD | 4 | 27 | 27 | 0 | 3 | 1 | 27 |
| Da Vinci DTR | 0 | 22 | 25 | 3 | 5 | 1 | 13 |
| Da Vinci PAS | 28 | 47 | 81 | 3 | 1 | 1 | 12 |
| IPS | 0 | 72 | 32 | 6 | 0 | 1 | 103 |

### 1.2 Fixture corpus

The SUSHI compatibility test suite compares rh-fsh output against SUSHI-generated
golden files. All fixtures are in `tests/fixtures/`, organized by FSH feature category.

#### Fixture inventory (2026-06-12)

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

### 1.3 How to run

```bash
# Run project-level comparison (requires Node.js, npx, git, and built rh CLI):
cargo build -p rh-cli
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py

# Run fast profile identity smoke fixture:
python3 crates/rh-fsh/conformance/scripts/run_sushi_comparison.py \
  --fixture profile-identity-smoke

# Generate SUSHI golden files (requires Node.js and npx):
./scripts/generate-fsh-goldens.sh

# Run SUSHI compat tests:
cargo test -p rh-fsh --test sushi_compat -- --include-ignored

# Parser unit tests (no SUSHI required):
cargo test -p rh-fsh --lib
```

Unit tests (83 cases) cover parser, resolver, dependency loading, definition
indexing, and export behavior without requiring SUSHI.

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

### Project-runner coverage notes

Recent fixes made while bringing up the project runner:

- Invariant entities now accept `* expression = ...`, `* severity = ...`,
  `* description = ...`, and `* xpath = ...` rule-style fields, including
  multiline quoted expressions.
- Alias expansion now applies to `Parent:` and `InstanceOf:` metadata.
- Instance export now resolves local profile parents to base FHIR resource types
  and uses the resolved type for references.
- Project compiles ingest `sushi-config.yaml` dependencies, load cached package
  StructureDefinitions, and resolve dependency-backed profile instances such as
  US Core, CRD, SDC, Genomics Reporting, and mCODE profile chains to base FHIR
  resource types.
- Project compiles emit minimal `ImplementationGuide` resources from normalized
  `sushi-config.yaml` metadata, CodeSystem `count` values, safe deterministic
  CLI output filenames, and JSON compile envelopes containing resources plus
  non-fatal diagnostics.
- The parser accepts `$` aliases that begin with OID-style digits, and the tank
  permits duplicate names across different FSH entity kinds while still rejecting
  same-kind duplicates.
- The lightweight R4 resource registry includes additional resource names needed
  by mCODE and DTR.

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
| Project-level exporter parity | High | mCODE and DTR still differ from SUSHI in resource identity and JSON shape |
| IG metadata parity | Medium | Minimal `ImplementationGuide/*` resources are generated, but detailed SUSHI metadata still differs |
| Array/scalar JSON shape fidelity | High | Remaining mismatches include arrays vs scalars for fields such as `supportedProfile`, `targetProfile`, and nested instance fields |
| Primitive extension shadow fields | Medium | SUSHI emits `_field` companion arrays for extensions on primitive values; rh-fsh has partial support |
| Extension JSON fidelity | Medium | Nested extension paths can miss wrapping arrays, `url`, or `value[x]` shape in complex examples |
| Parser unit tests for SUSHI compat assertions | N/A | 70 parser unit tests cover entity/rule/path/value forms |
