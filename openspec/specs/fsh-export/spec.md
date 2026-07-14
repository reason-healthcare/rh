# fsh-export Specification

## Purpose

Define the FSH-to-FHIR export behavior for supported FSH entities.

## Requirements

### Requirement: Export Profiles and Extensions as FHIR StructureDefinitions
The exporter SHALL convert each `Profile` and `Extension` entity in the resolved `FshTank` into a FHIR `StructureDefinition` JSON resource containing a `differential` section built from the entity's rules.

#### Scenario: Profile with cardinality rule
- **WHEN** a Profile with `* subject 1..1` is exported
- **THEN** the output StructureDefinition JSON contains a `differential.element` entry for `subject` with `min: 1` and `max: "1"`

#### Scenario: Profile with flag rule
- **WHEN** a Profile with `* code MS` is exported
- **THEN** the output StructureDefinition JSON contains a differential element for `code` with `mustSupport: true`

#### Scenario: Profile with binding rule
- **WHEN** a Profile with `* status from StatusVS (required)` is exported
- **THEN** the output StructureDefinition JSON contains a differential element for `status` with `binding.strength: "required"` and `binding.valueSet` referencing `StatusVS`

#### Scenario: Profile with type restriction (only rule)
- **WHEN** a Profile with `* value[x] only Quantity` is exported
- **THEN** the output StructureDefinition JSON contains a differential element for `value[x]` with `type` restricted to `Quantity`

#### Scenario: Profile metadata in StructureDefinition
- **WHEN** a Profile with `Id`, `Title`, and `Description` metadata is exported
- **THEN** the output JSON has `id`, `title`, and `description` fields matching the FSH metadata

#### Scenario: Extension structure
- **WHEN** an Extension entity is exported
- **THEN** the output StructureDefinition has `type: "Extension"` and `baseDefinition` pointing to the FHIR Extension base

### Requirement: Export Instances as FHIR resource JSON
The exporter SHALL convert each `Instance` entity into a FHIR resource JSON
object whose resource type is determined by `instanceOf`, with properties set
from its assignment rules. For every assigned path, including paths inside
contained, inline, Bundle, and Parameters resources, the exporter SHALL use
resolved FHIR and profile metadata to select the compatible recursive JSON
shape.

#### Scenario: Instance assignment rule sets property
- **WHEN** an Instance has `* status = #active` as an assignment rule
- **THEN** the output JSON contains `"status": "active"`

#### Scenario: Instance nested path assignment
- **WHEN** an Instance has `* subject.reference = "Patient/123"` as an assignment rule
- **THEN** the output JSON contains `{ "subject": { "reference": "Patient/123" } }`

#### Scenario: Instance resource type from instanceOf
- **WHEN** an Instance declares `InstanceOf: Observation`
- **THEN** the output JSON has `"resourceType": "Observation"`

#### Scenario: Recursive embedding uses the same field shape
- **WHEN** an Instance assigns a repeating or typed field inside a contained,
  inline, Bundle-entry, or Parameters-part resource
- **THEN** that embedded field has the same array/scalar and datatype JSON shape
  as the field would have in a top-level resource of the same type

#### Scenario: Explicit assignment overrides profile defaults
- **WHEN** core FHIR metadata, a dependency profile, or a local profile supplies
  a default for a path and the Instance explicitly assigns that effective path
- **THEN** the exported value reflects the explicit Instance assignment without
  losing stable indexed or sliced repetitions

#### Scenario: Nested extension preserves parent and primitive context
- **WHEN** an Instance assigns a nested extension or an extension on a primitive
  field within a recursive resource path
- **THEN** the output contains parent-scoped extension URLs and the correctly
  indexed `extension[]` or primitive `_field` companion structure

### Requirement: Preserve shape-safe profile defaults
The exporter SHALL apply structural and profile-derived defaults only through
the same schema-shaped instance tree used for explicit assignments. It SHALL
apply applicable defaults in deterministic precedence order and SHALL NOT emit
an unused optional extension placeholder.

#### Scenario: Local profile value supersedes dependency default
- **WHEN** a dependency profile and a local profile both define a value for the
  same effective Instance path
- **THEN** the local profile value is retained unless an explicit Instance
  assignment supplies a more specific value

#### Scenario: Optional extension is not materialized without use
- **WHEN** a profile describes an optional named extension and no applicable
  default or Instance assignment uses that extension
- **THEN** the exported resource does not contain an empty extension placeholder

### Requirement: Export ValueSets as FHIR ValueSet JSON
The exporter SHALL convert each `ValueSet` entity into a FHIR `ValueSet` JSON resource with a `compose` section built from include and exclude component rules.

#### Scenario: ValueSet with include by system
- **WHEN** a ValueSet has an include rule for a code system URL
- **THEN** the output JSON contains `compose.include` with the correct `system`

#### Scenario: ValueSet with concept filter
- **WHEN** a ValueSet includes concepts with a filter (e.g., `where concept is-a #code`)
- **THEN** the output JSON `compose.include` contains a `filter` array with the correct `property`, `op`, and `value`

#### Scenario: ValueSet with exclude rule
- **WHEN** a ValueSet has an exclude rule
- **THEN** the output JSON contains a `compose.exclude` entry

### Requirement: Export CodeSystems as FHIR CodeSystem JSON
The exporter SHALL convert each `CodeSystem` entity into a FHIR `CodeSystem` JSON resource with a `concept` array built from `ConceptRule` entries.

#### Scenario: CodeSystem with flat concepts
- **WHEN** a CodeSystem defines several concepts without hierarchy
- **THEN** the output JSON contains a `concept` array with one entry per concept, each having `code` and `display`

#### Scenario: CodeSystem with hierarchical concepts
- **WHEN** a CodeSystem defines a concept with child concepts using indented rules
- **THEN** the output JSON contains nested `concept` entries reflecting the hierarchy

### Requirement: Export Mappings as FHIR ConceptMap JSON
The exporter SHALL convert each `Mapping` entity into a FHIR `ConceptMap` JSON resource with `group` entries built from `MappingRule` entries.

#### Scenario: Mapping entity produces ConceptMap
- **WHEN** a Mapping entity with source, target, and mapping rules is exported
- **THEN** the output JSON has `resourceType: "ConceptMap"` with `group[].element[]` reflecting the mapping rules

### Requirement: Emit differential-only StructureDefinitions by default
The exporter SHALL emit `differential`-only StructureDefinitions (no `snapshot` section) by default. Snapshot generation SHALL be opt-in via `CompilerOptions`.

#### Scenario: Default output has no snapshot
- **WHEN** a Profile is exported with default `CompilerOptions`
- **THEN** the output JSON does NOT contain a `snapshot` key

### Requirement: Export entities in parallel
The exporter SHALL use parallel iteration (via rayon) to export independent entities concurrently, with shared FHIR definitions wrapped in `Arc` for safe cross-thread access.

#### Scenario: Parallel export produces correct output
- **WHEN** a tank with 50 Profiles and 100 Instances is exported with `parallel: true`
- **THEN** the output `FhirPackage` contains exactly 50 StructureDefinitions and 100 instances, all correctly populated

### Requirement: Collect non-fatal export errors without aborting
The exporter SHALL collect errors for individual failing entities into `FhirPackage::errors` and continue exporting remaining entities rather than halting on the first error.

#### Scenario: One invalid Profile does not block others
- **WHEN** a tank contains one Profile with an unresolvable parent and nine valid Profiles
- **THEN** the output `FhirPackage` contains nine StructureDefinitions and one entry in `FhirPackage::errors`

### Requirement: Pre-index FHIR R4 base StructureDefinitions for O(1) element lookup
The `FhirDefs` registry SHALL load all base FHIR R4 StructureDefinitions from `rh-hl7_fhir_r4_core` at construction time and index their elements by path, enabling O(1) lookup without repeated tree traversal.

#### Scenario: Element lookup by path
- **WHEN** `fhir_defs.get_element("Observation", "status")` is called
- **THEN** an `ElementSummary` for `Observation.status` is returned without traversing the full StructureDefinition tree

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
