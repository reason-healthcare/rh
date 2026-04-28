## ADDED Requirements

### Requirement: validate processor runs rh-validator over all source FHIR resources

When the `validate` processor is invoked as a lifecycle hook, it SHALL run `rh-validator` over
every FHIR resource in the in-memory resource map and fail the pipeline if any resource has an
issue of ERROR severity.

#### Scenario: Resource with ERROR severity fails the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces an ERROR-severity issue for any resource
- **THEN** the processor returns an error, the pipeline aborts, and the error details are printed to stderr

#### Scenario: Resource with WARNING severity does not fail the pipeline
- **WHEN** the `validate` processor runs and rh-validator produces only WARNING-severity issues
- **THEN** the processor succeeds, warnings are printed, and the pipeline continues

#### Scenario: All resources valid continues the pipeline
- **WHEN** the `validate` processor runs and all resources pass validation
- **THEN** the processor succeeds and the pipeline continues

### Requirement: validate processor loads dependency packages for validation context

The `validate` processor SHALL load the FHIR packages listed in `package.json` `dependencies`
from the configured packages directory to provide StructureDefinition context for validation.

#### Scenario: Dependency StructureDefinitions are available during validation
- **WHEN** `package.json` declares `"hl7.fhir.us.core": "6.1.0"` as a dependency and that package is present
- **THEN** US Core profiles from that package are available as validation context

#### Scenario: Missing dependency package fails processor with named error
- **WHEN** a declared dependency package is not present in the packages directory
- **THEN** the `validate` processor fails with a message identifying the missing package before attempting validation

### Requirement: validate processor is configurable via publisher.toml

The `validate` processor SHALL read a `[validate]` configuration block from `publisher.toml`
to control validation behaviour.

#### Scenario: Terminology server is configured
- **WHEN** `publisher.toml` contains `[validate]\nterminology_server = "https://tx.fhir.org/r4"`
- **THEN** the `validate` processor passes the terminology server URL to rh-validator for binding validation

#### Scenario: skip_invariants disables invariant checks
- **WHEN** `publisher.toml` contains `[validate]\nskip_invariants = true`
- **THEN** the `validate` processor runs with invariant validation disabled

#### Scenario: Default configuration when [validate] block is absent
- **WHEN** `publisher.toml` has no `[validate]` block
- **THEN** the `validate` processor runs with default rh-validator settings (no terminology server, invariants enabled)
