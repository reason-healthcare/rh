# Branch Split Plan

This plan captures the intended final state from the exploratory
`fix-package-canonical` branch and splits it into smaller OpenSpec-aligned PRs.

## PR 1: Packaging Canonical URL Handling

Scope:

- Treat `packager.toml` `canonical` as the literal package canonical base.
- Keep generated `package.json` emitting both `url` and `canonical`.
- Derive `ImplementationGuide.url` as:
  `{canonical}/ImplementationGuide/{ImplementationGuide.id}`.
- Ensure `rh package init` generates `ImplementationGuide.url` from the
  generated `ImplementationGuide.id`, not the raw package name.
- Warn, but do not fail, when:
  - `canonical` looks like an `ImplementationGuide` resource URL.
  - `ImplementationGuide.url` differs from the derived URL.
  - existing canonical resource `url` fields differ from
    `{canonical}/{resourceType}/{id}`.
- Use generated R4 metadata to detect canonical resources with root `url`
  fields instead of maintaining a hardcoded resource-type list.
- Improve package validation failure messages by surfacing the first validation
  error directly.
- Include the minimal `crossbeam-epoch` dependency update required for
  `cargo audit`.

OpenSpec change: `package-canonical-url-handling`.

## PR 2: CQL FHIR JSON Evaluation

Scope:

- Add FHIR JSON conversion support for CQL evaluation, converting raw JSON
  resources into CQL runtime values while preserving FHIR object/list structure.
- Add FHIR choice-element aliases so CQL model paths like `Observation.value`
  resolve against raw JSON fields like `valueQuantity`, `valueBoolean`, or
  `valueCodeableConcept`.
- Add FHIR-aware CQL casts and conversions for JSON-backed values, including
  `as FHIR.boolean`, `as FHIR.CodeableConcept`, and `CodeableConcept` to CQL
  `Concept` conversion.

Suggested OpenSpec change: `cql-fhir-json-evaluation`.

## PR 3: Validator Coverage Improvements

Scope:

- Unknown R4 resource and property validation.
- Choice element validation.
- Reference target validation.
- Fixed and pattern value validation.
- Slice-aware cardinality, binding, and fixed/pattern rule validation.
- QuestionnaireResponse validation improvements.
- Terminology and ValueSet validation improvements.
- Selected primitive format and invariant coverage.
- Validator conformance quick-win tests and runner coverage.
- Public API compatibility cleanup for cached validator internals, keeping
  shared cached values internal where possible.

Suggested OpenSpec change: `validator-coverage-improvements`.

## Exclusions From The Split

- Do not include unrelated OpenSpec archive cleanup in these PRs unless a
  separate process cleanup change is opened.
- Do not include broad Measure, Bundle, or narrative validation expansion unless
  a follow-up OpenSpec change explicitly scopes it.
- Do not remove `canonical` from generated `package.json`.
