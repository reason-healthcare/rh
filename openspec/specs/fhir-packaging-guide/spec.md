# fhir-packaging-guide

## Purpose

Defines the end-user guide for assembling a FHIR package with the `rh package` pipeline.

## Requirements

### Requirement: Guide document exists

`apps/rh-cli/docs/PACKAGER.md` SHALL provide a step-by-step FHIR package walkthrough, while
`crates/rh-packager` documentation SHALL focus on library internals and link to that CLI guide.

#### Scenario: Guide is available from CLI documentation
- **WHEN** a user opens `apps/rh-cli/docs/`
- **THEN** `PACKAGER.md` SHALL contain a step-by-step package guide

#### Scenario: Crate README links to the guide
- **WHEN** a user reads `crates/rh-packager/README.md`
- **THEN** it SHALL link to `apps/rh-cli/docs/PACKAGER.md`

### Requirement: Guide covers project setup

The guide SHALL explain the required `package.json`, `ImplementationGuide.json`, and
`packager.toml` files before processor execution.

#### Scenario: package.json structure is shown
- **WHEN** a user reads the setup section
- **THEN** it SHALL show a complete `package.json` with name, version, FHIR versions, and dependencies

#### Scenario: Complete processor configuration is shown
- **WHEN** a user reads the setup section
- **THEN** it SHALL show `packager.toml` wiring `fsh`, `snapshot`, `validate`, `cql`, and `shell` in order

### Requirement: Guide covers the FSH processor

The guide SHALL demonstrate authoring a FHIR Shorthand profile and running the `fsh` processor
to compile it into JSON resources available to later pipeline stages.

#### Scenario: FSH source and output are described
- **WHEN** a user reads the FSH step
- **THEN** it SHALL show an example `.fsh` profile and explain its resulting FHIR JSON output

### Requirement: Guide covers the snapshot processor

The guide SHALL explain snapshot expansion and any required `[snapshot]` configuration.

#### Scenario: Snapshot expansion and configuration are shown
- **WHEN** a user reads the snapshot step
- **THEN** it SHALL explain differential expansion and show relevant package configuration

### Requirement: Guide covers the validate processor

The guide SHALL explain `validate` conformance checks and the pipeline behavior on validation
failure.

#### Scenario: Validation behavior is described
- **WHEN** a user reads the validate step
- **THEN** it SHALL explain checked resources and that validation errors halt the pipeline

### Requirement: Guide covers the CQL processor

The guide SHALL demonstrate authoring CQL, running the `cql` processor, and embedding emitted
ELM in a FHIR `Library` resource.

#### Scenario: CQL source and output are shown
- **WHEN** a user reads the CQL step
- **THEN** it SHALL show an example library and explain the upserted `Library` resource

### Requirement: Guide covers the shell processor

The guide SHALL demonstrate a `[processors.<name>]` shell processor and document
`PACKAGER_SOURCE_DIR`, `PACKAGER_OUTPUT_DIR`, `PACKAGER_WORKDIR`, and `PACKAGER_PACKAGE_NAME`.

#### Scenario: Shell configuration and environment are shown
- **WHEN** a user reads the shell processor step
- **THEN** it SHALL show complete configuration and the available environment variables

### Requirement: Guide includes an extended custom-hook example

The guide SHALL include an extending-the-pipeline example with a build-date stamping hook and a
validation hook that reads and writes `$PACKAGER_WORKDIR/resources/`.

#### Scenario: Stamp and validation hooks are complete
- **WHEN** a user reads the extended example
- **THEN** it SHALL show scripts that stamp resources and fail when the tag is absent

#### Scenario: Hook wiring and failure contract are explained
- **WHEN** a user reads the extended example
- **THEN** it SHALL show `packager.toml` hook placement and explain that non-zero exits abort the pipeline

### Requirement: Guide shows final package output

The guide SHALL show the output directory structure and `.tgz` artifact produced by
`rh package build`.

#### Scenario: Build command and output layout are shown
- **WHEN** a user reads the final output section
- **THEN** it SHALL show the build command, output package tree, and resulting archive name
