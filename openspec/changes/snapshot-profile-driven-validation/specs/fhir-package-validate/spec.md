## MODIFIED Requirements

### Requirement: validate processor runs rh-validator over all source FHIR resources

When the `validate` processor is invoked as a lifecycle hook, it SHALL run
`rh-validator` over every FHIR resource in the in-memory resource map and fail
the pipeline if any resource has an issue of ERROR severity. When failing, the
processor error SHALL include the resource label, first concrete ERROR issue,
and a count of additional ERROR issues when present. For a given resource JSON,
package validation SHALL surface the same validator issues that
`rh validate resource` surfaces, except for additional package-local context
that is explicitly loaded by the validate processor.

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

#### Scenario: Package validation catches standalone structural error
- **WHEN** a package build runs the `validate` hook over a resource that `rh validate resource` reports as having a nested unknown property
- **THEN** package validation reports the same nested unknown-property ERROR and fails the build

#### Scenario: Package validation catches standalone expression language error
- **WHEN** a package build runs the `validate` hook over a resource that `rh validate resource` reports as having an invalid `Expression.language`
- **THEN** package validation reports the same expression-language ERROR and fails the build

#### Scenario: Package build without validate hook is unchanged
- **WHEN** a package build does not configure the `validate` lifecycle hook
- **THEN** this change does not make package build run resource validation implicitly
