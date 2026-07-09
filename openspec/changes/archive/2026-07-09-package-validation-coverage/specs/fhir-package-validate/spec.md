## MODIFIED Requirements

### Requirement: validate processor runs rh-validator over all source FHIR resources

When the `validate` processor is invoked as a lifecycle hook, it SHALL run
`rh-validator` over every FHIR resource in the in-memory resource map and fail
the pipeline if any resource has an issue of ERROR severity. When failing, the
processor error SHALL include the resource label, first concrete ERROR issue,
and a count of additional ERROR issues when present.

#### Scenario: Resource with ERROR severity fails the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces an ERROR-severity issue for any resource
- **THEN** the processor returns an error, the pipeline aborts, and the error details are printed to stderr
- **AND** the returned error message includes the resource label and first concrete ERROR issue

#### Scenario: Multiple ERROR severity issues summarize additional count
- **WHEN** the `validate` processor sees more than one ERROR-severity issue across package resources
- **THEN** the returned error message includes the first ERROR issue and the number of additional ERROR issues

#### Scenario: Resource with WARNING severity does not fail the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces only WARNING-severity issues
- **THEN** the processor succeeds, warnings are printed, and the pipeline continues

#### Scenario: All resources valid continues the pipeline
- **WHEN** the `validate` processor runs and all resources pass validation
- **THEN** the processor succeeds and the pipeline continues

## ADDED Requirements

### Requirement: validate processor provides package-local validation context

The `validate` processor SHALL make package-local resources available as
validation context for checks that can resolve resources locally, including
StructureDefinitions, Questionnaires, and ValueSets. This package-local context
loading is specific to the validate processor and SHALL NOT imply that
standalone resource validation discovers sibling package resources implicitly.

#### Scenario: Local ValueSet is available for binding validation
- **WHEN** a package contains a ValueSet and a profile binding references that ValueSet by canonical URL
- **THEN** package validation can use the local ValueSet for binding validation without requiring a terminology server

#### Scenario: Local Questionnaire is available for QuestionnaireResponse validation
- **WHEN** a package contains a Questionnaire and a QuestionnaireResponse references it
- **THEN** package validation can validate the QuestionnaireResponse against the local Questionnaire

### Requirement: validate processor preserves severity-based pipeline contract

The `validate` processor SHALL preserve existing severity semantics for all
newly surfaced validator checks: ERROR issues fail the processor and WARNING
issues do not fail the processor.

#### Scenario: New structural ERROR fails package validation
- **WHEN** package validation detects an unknown required resource shape error classified as ERROR
- **THEN** the validate processor fails according to the existing ERROR behavior

#### Scenario: New warning remains non-blocking
- **WHEN** package validation detects an issue classified as WARNING
- **THEN** the validate processor prints the warning and continues if no ERROR issues exist
