## ADDED Requirements

### Requirement: Export profile instances with SUSHI-compatible resource identity
The exporter SHALL emit Instance resources using the underlying FHIR base `resourceType` rather than the profile name when `InstanceOf` references a local or dependency profile.

#### Scenario: Local profile instance uses base resource type
- **WHEN** an Instance declares `InstanceOf: CancerPatient` and `CancerPatient` ultimately derives from `Patient`
- **THEN** the exported JSON has `"resourceType": "Patient"`

#### Scenario: Dependency profile instance uses base resource type
- **WHEN** an Instance declares `InstanceOf: USCorePractitionerProfile` and that profile derives from `Practitioner`
- **THEN** the exported JSON has `"resourceType": "Practitioner"`

#### Scenario: Reference uses resolved base resource type
- **WHEN** one Instance references another Instance whose `InstanceOf` is a profile
- **THEN** the exported reference uses `BaseResource/id` rather than `ProfileName/id`

### Requirement: Export SUSHI-compatible instance JSON shapes
The exporter SHALL use FHIR metadata and resolved profile element information to choose array/scalar, primitive shadow field, Coding/CodeableConcept, Quantity, Reference, Canonical, and contained-resource JSON shapes compatible with SUSHI.

#### Scenario: Repeating field assignment emits array
- **WHEN** a rule assigns a value to a repeating FHIR element without an explicit index
- **THEN** the exported JSON contains an array value matching SUSHI output

#### Scenario: CodeableConcept assignment wraps coding
- **WHEN** a coded FSH value is assigned to a CodeableConcept element
- **THEN** the exported JSON contains `coding: [{ system, code, display }]`

#### Scenario: Quantity assignment emits UCUM fields
- **WHEN** a quantity with a UCUM unit is assigned to a Quantity element
- **THEN** the exported JSON includes SUSHI-compatible `value`, `unit`, `system`, and `code` fields where applicable

#### Scenario: Primitive extension emits shadow field
- **WHEN** a rule assigns an extension to a primitive element
- **THEN** the exported JSON emits the corresponding `_element` companion structure matching SUSHI output

#### Scenario: Nested extension emits extension array
- **WHEN** a rule assigns a nested extension path
- **THEN** the exported JSON includes each required `extension[]` wrapper, `url`, and `value[x]` field

### Requirement: Export SUSHI-compatible StructureDefinition details
The exporter SHALL produce StructureDefinition differentials with SUSHI-compatible root elements, base definitions, constraints, extension contexts, root cardinality, and caret metadata.

#### Scenario: Obeys rule emits root constraint
- **WHEN** a Profile or Extension applies an invariant through an `obeys` rule
- **THEN** the exported differential includes a `constraint` entry with key, severity, human, expression, and source matching SUSHI

#### Scenario: Extension context is exported
- **WHEN** an Extension declares context caret rules
- **THEN** the exported StructureDefinition contains the expected `context` entries

#### Scenario: Logical model baseDefinition is preserved
- **WHEN** a Logical or Resource entity derives from `Base`
- **THEN** the exported StructureDefinition uses the same `baseDefinition` SUSHI emits

### Requirement: Export project metadata resources needed for SUSHI parity
The exporter or compile pipeline SHALL emit project-level metadata resources that SUSHI produces from project configuration when compiling an IG project.

#### Scenario: ImplementationGuide resource emitted
- **WHEN** project configuration contains an IG id, canonical URL, package id, version, status, publisher, and dependencies
- **THEN** the compiled package includes an `ImplementationGuide/<id>` resource comparable to SUSHI output

#### Scenario: CodeSystem count emitted
- **WHEN** a CodeSystem has concepts
- **THEN** the exported CodeSystem includes `count` matching the number of top-level concepts SUSHI reports

#### Scenario: Definition instance canonical URL emitted
- **WHEN** an Instance has `Usage: #definition` and project canonical configuration
- **THEN** the exported resource includes the canonical `url` SUSHI would assign for its resource type and id

