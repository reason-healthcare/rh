## Why

Current R4 structural validation catches root-level unknown properties but does
not consistently validate nested backbone/datatype fields or terminology-coded
primitive values from the active base/profile snapshots. This creates drift
between `rh validate resource`, package validation, and HAPI/IG Publisher-style
findings for common authoring errors such as invalid nested
`PlanDefinition.action.relatedAction` fields and unsupported
`Expression.language` codes.

## What Changes

- Replace root-only R4 shape checks with recursive, snapshot/profile-driven
  structural validation for base R4 resources and resolvable `meta.profile`
  profiles.
- Validate unknown JSON properties at nested resource, backbone element, and
  datatype paths using active StructureDefinition snapshot element paths.
- Validate concrete choice fields and primitive extension siblings from the
  same snapshot-derived element model.
- Add focused known-code validation for `Expression.language` using the R4
  expression-language code system so unsupported codes such as
  `text/cql-identifier` are reported.
- Ensure package validation reports the same resource-level issues as
  `rh validate resource` when the `validate` hook is configured.
- Do not add new QuestionnaireResponse-vs-Questionnaire semantic validation in
  this change; that remains separate from snapshot/profile structural
  validation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `validator-resource-conformance`: Extend resource conformance validation from
  root-level metadata checks to recursive base/profile snapshot checks.
- `fhir-package-validate`: Require package validate hook behavior to surface
  the same validator issues as standalone resource validation for the resources
  it validates.

## Impact

- Affected crates: `rh-validator`, `rh-packager`, generated R4 metadata/codegen
  only if additional snapshot-derived metadata is required.
- Affected behavior: resources with nested invalid fields or invalid
  `Expression.language` values may now fail validation where they previously
  passed.
- Compatibility: no change to whether `rh package build` runs validation; the
  validate hook remains the activation mechanism.
