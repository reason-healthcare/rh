## ADDED Requirements

### Requirement: publisher.toml declares hook processors per pipeline stage

A `publisher.toml` file in the source directory SHALL be read by `rh publish build` and `rh
publish pack` to determine which named processors run at each lifecycle stage. If
`publisher.toml` is absent, no processors are run.

#### Scenario: publisher.toml with before_build processors
- **WHEN** `publisher.toml` contains `[hooks]\nbefore_build = ["snapshot", "validate"]`
- **THEN** `rh publish build` runs the `snapshot` processor then the `validate` processor before assembling output

#### Scenario: Missing publisher.toml skips all hooks
- **WHEN** no `publisher.toml` exists in the source directory
- **THEN** `rh publish build` runs with no hook processors and emits an informational message

### Requirement: Pipeline stages are before_build, after_build, before_pack, after_pack

The following hook stages SHALL be supported:
- `before_build`: runs before resource assembly begins
- `after_build`: runs after the expanded output directory is written
- `before_pack`: runs before `.tgz` creation
- `after_pack`: runs after `.tgz` creation

#### Scenario: after_build processor sees completed output directory
- **WHEN** a processor is registered under `after_build`
- **THEN** it receives a context in which the expanded output directory has been fully written

#### Scenario: before_pack processor runs before tarball creation
- **WHEN** a processor is registered under `before_pack`
- **THEN** it runs after the build output directory exists but before the `.tgz` is created

### Requirement: Unknown processor names fail at pipeline startup

If `publisher.toml` declares a processor name that is not a registered built-in, the publisher
SHALL fail immediately at startup before reading any resources.

#### Scenario: Unknown processor name aborts before any work
- **WHEN** `publisher.toml` contains `before_build = ["unknown-processor"]`
- **THEN** the command exits with a non-zero status immediately with a message identifying the unknown processor name, without reading or writing any files

### Requirement: First processor failure aborts the pipeline

Processors within a stage run sequentially in declaration order. If any processor returns an
error, the pipeline SHALL abort immediately without running subsequent processors or stages.

#### Scenario: First failure prevents subsequent processors from running
- **WHEN** `before_build = ["snapshot", "validate"]` and `snapshot` fails
- **THEN** `validate` does not run and the build does not proceed

#### Scenario: All processors succeed and build continues
- **WHEN** all processors in `before_build` succeed
- **THEN** the build assembly step proceeds normally

### Requirement: Processors share an in-memory resource map

All processors for a given build invocation SHALL operate on a shared mutable resource map
(`filename stem → serde_json::Value`). Changes made by one processor are visible to subsequent
processors in the same run.

#### Scenario: snapshot processor output is visible to validate processor
- **WHEN** `before_build = ["snapshot", "validate"]` and `snapshot` adds snapshot elements to a StructureDefinition
- **THEN** the `validate` processor sees the StructureDefinition with the snapshot already populated
