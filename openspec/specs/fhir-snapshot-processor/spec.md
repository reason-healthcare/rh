# fhir-snapshot-processor

## Purpose

Defines the behaviour of the `snapshot` lifecycle hook processor, which generates StructureDefinition snapshots using `rh-foundation` `SnapshotGenerator` during a publish pipeline.

## Requirements

### Requirement: snapshot processor generates snapshots for all StructureDefinitions

When the `snapshot` processor is invoked as a lifecycle hook, it SHALL identify every
`StructureDefinition` resource in the in-memory resource map that lacks a `snapshot` element
and run `rh-foundation` `SnapshotGenerator` to produce a full snapshot, updating the resource
in-place in the shared resource map.

#### Scenario: StructureDefinition without snapshot is expanded
- **WHEN** the `snapshot` processor runs and a `StructureDefinition` in the resource map has no `snapshot.element` array
- **THEN** the resource is updated in the shared map with a populated `snapshot.element` array

#### Scenario: StructureDefinition with existing snapshot is left unchanged
- **WHEN** the `snapshot` processor runs and a `StructureDefinition` already has a non-empty `snapshot.element` array
- **THEN** the resource is not modified

#### Scenario: Non-StructureDefinition resources are not affected
- **WHEN** the `snapshot` processor runs
- **THEN** resources with `resourceType` other than `StructureDefinition` are not modified

### Requirement: snapshot processor loads dependency packages for base definition resolution

The `snapshot` processor SHALL load FHIR packages listed in `package.json` `dependencies` from
the configured packages directory to resolve base StructureDefinition references required for
snapshot generation.

#### Scenario: Base FHIR R4 definitions are loaded from core package
- **WHEN** `package.json` declares a dependency on `hl7.fhir.r4.core`
- **THEN** the `snapshot` processor loads base resource definitions from that package for differential merging

#### Scenario: Missing base definition fails with clear error
- **WHEN** a StructureDefinition's `baseDefinition` cannot be resolved from loaded packages
- **THEN** the processor fails with an error identifying the unresolvable base URL

### Requirement: snapshot processor failure aborts the pipeline

If snapshot generation fails for any StructureDefinition, the processor SHALL return an error
that aborts the pipeline per the lifecycle hooks abort-on-first-failure behaviour.

#### Scenario: Snapshot generation error aborts build
- **WHEN** the `snapshot` processor encounters a StructureDefinition whose differential contains an invalid constraint
- **THEN** the processor returns an error with the StructureDefinition URL and the build is aborted
