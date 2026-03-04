# validator-cli-behavior Specification

## Purpose
TBD - created by archiving change validator-conformance-wave-1. Update Purpose after archive.

## Requirements
### Requirement: Validator CLI accepts conformance runtime options
The validator CLI SHALL expose runtime options required for conformance execution, including security-checks mode and terminology service configuration.

#### Scenario: CLI enables security-checks mode via option
- **WHEN** a user runs validation with a CLI option enabling security checks
- **THEN** the validator executes with security checks enabled for that run

#### Scenario: CLI accepts terminology server option
- **WHEN** a user provides a terminology server endpoint option for validation
- **THEN** the validator initializes terminology-backed checks using that endpoint

### Requirement: Validator CLI remains backward compatible by default
The validator CLI MUST preserve existing default behavior when no new conformance runtime options are provided.

#### Scenario: Existing invocation runs unchanged
- **WHEN** a user runs existing validation commands without new options
- **THEN** command behavior and exit semantics remain compatible with prior defaults

### Requirement: CLI reports effective conformance runtime configuration
The validator CLI SHALL report effective conformance-relevant runtime options in command output.

#### Scenario: Effective runtime configuration is printed
- **WHEN** validation starts
- **THEN** output includes the effective security-checks mode and whether terminology service is configured
