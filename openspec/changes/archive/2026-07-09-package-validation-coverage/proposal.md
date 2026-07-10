## Why

Package validation currently depends on `rh-validator`, but the package-facing
checks can miss common FHIR authoring errors or return generic failures that are
hard to act on in a packaging pipeline. This change scopes validator coverage to
the checks most relevant when building and checking FHIR packages.

## What Changes

- Improve `rh package` validate processor failures so the returned error
  includes the first concrete validation issue and a count of additional errors.
- Add validator coverage for R4 resource shape errors that matter in packages:
  unknown resource types, unknown properties, and invalid choice-element usage.
- Add profile-driven validation coverage for package resources:
  fixed/pattern values, reference target type checks, slice-aware cardinality,
  slice-aware binding, and slice-aware fixed/pattern checks.
- Improve QuestionnaireResponse validation against packaged Questionnaire
  resources, including required answers, answer type checks, answer option /
  answerValueSet checks where locally resolvable, and reference constraints.
- Add selected Questionnaire item authoring checks for core R4 rules such as
  `que-5` answer option applicability and incomplete answer option Coding
  warnings.
- Improve local terminology and ValueSet validation for package-local resources,
  including ValueSet expansion/compose lookup and version-tolerant canonical
  matching.
- Preserve existing severity behavior: ERROR issues fail the validate processor;
  WARNING issues are surfaced but do not fail the package pipeline.
- Keep R4 resource-shape validation in core `rh-validator` behavior so it
  applies to both standalone resource validation and package validation, while
  keeping package-local resource loading in the package validate processor only.
- Exclude broad Measure, MeasureReport, Bundle, and narrative validation
  expansion from this proposal unless a separate OpenSpec change scopes them.
- Avoid public Rust API churn unless needed; prefer internal cached structures
  and compatibility-preserving constructors/helpers for new rule internals.

## Capabilities

### New Capabilities

- `validator-resource-conformance`: Core `rh-validator` resource/profile
  validation behavior needed by package validation.

### Modified Capabilities

- `fhir-package-validate`: Package validate processor diagnostics and its use of
  `rh-validator` coverage for package resources.

## Impact

- Affected crates: `rh-packager`, `rh-validator`; CLI validation tests may be
  touched only where they exercise the same validator behavior.
- Affected package behavior: packages that already run the validate lifecycle
  hook may now surface additional FHIR validation issues. ERROR severity issues
  fail as before; WARNING severity issues remain non-blocking.
- Affected dependency behavior: package dependencies and local package resources
  provide validation context for profiles, Questionnaires, and ValueSets where
  available.
- Compatibility: no intended package manifest changes, no canonical/url handling
  changes, no CQL behavior changes, and no broad Measure/Bundle/narrative
  validation expansion.
