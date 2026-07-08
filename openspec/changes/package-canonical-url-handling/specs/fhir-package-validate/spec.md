## MODIFIED Requirements

### Requirement: validate processor runs rh-validator over all source FHIR resources

When the `validate` processor is invoked as a lifecycle hook, it SHALL run
`rh-validator` over every FHIR resource in the in-memory resource map and fail
the pipeline if any resource has an issue of ERROR severity. When failing, the
processor SHALL return an error message that includes the first concrete
validation error and indicate when additional errors were found.

#### Scenario: Resource with ERROR severity fails the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces an ERROR-severity issue for any resource
- **THEN** the processor returns an error, the pipeline aborts, and the returned error includes the resource label and first validation error

#### Scenario: Multiple ERROR severity issues summarize remaining errors
- **WHEN** the `validate` processor runs and rh-validator produces multiple ERROR-severity issues
- **THEN** the returned error includes the first validation error and reports how many additional errors were found

#### Scenario: Resource with WARNING severity does not fail the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces only WARNING-severity issues
- **THEN** the processor succeeds, warnings are printed, and the pipeline continues

#### Scenario: All resources valid continues the pipeline
- **WHEN** the `validate` processor runs and all resources pass validation
- **THEN** the processor succeeds and the pipeline continues
