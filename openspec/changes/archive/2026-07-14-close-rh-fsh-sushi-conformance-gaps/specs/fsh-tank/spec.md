## ADDED Requirements

### Requirement: Resolve dependency package StructureDefinitions for FSH compilation
The `FshTank` or compile pipeline SHALL make dependency package StructureDefinitions available for profile parent and instance base resource resolution.

#### Scenario: Resolve dependency profile parent
- **WHEN** a local profile declares a parent from a dependency package by name, id, or canonical URL
- **THEN** resolution finds the dependency StructureDefinition and records its base resource type

#### Scenario: Resolve external InstanceOf profile
- **WHEN** an Instance declares `InstanceOf` as a profile from a dependency package
- **THEN** export can determine the underlying FHIR base resource type from the dependency StructureDefinition

#### Scenario: Missing dependency is reported
- **WHEN** a required dependency package cannot be loaded from the configured package cache
- **THEN** compilation reports a structured non-fatal error identifying the missing package and unresolved profile

### Requirement: Index definitions by all SUSHI lookup keys
The tank resolution layer SHALL index local and dependency definitions by FSH name, id, canonical URL, and aliases expanded from metadata and rules.

#### Scenario: Lookup by canonical URL
- **WHEN** a profile references `http://example.org/StructureDefinition/my-profile`
- **THEN** the definition lookup returns the matching local or dependency StructureDefinition

#### Scenario: Lookup by id
- **WHEN** a profile references a StructureDefinition id instead of its FSH name
- **THEN** the definition lookup returns the same definition as a name lookup

