## ADDED Requirements

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
The exporter SHALL convert each `Instance` entity into a FHIR resource JSON object whose resource type is determined by `instanceOf`, with properties set from the Instance's assignment rules.

#### Scenario: Instance assignment rule sets property
- **WHEN** an Instance has `* status = #active` as an assignment rule
- **THEN** the output JSON contains `"status": "active"`

#### Scenario: Instance nested path assignment
- **WHEN** an Instance has `* subject.reference = "Patient/123"` as an assignment rule
- **THEN** the output JSON contains `{ "subject": { "reference": "Patient/123" } }`

#### Scenario: Instance resource type from instanceOf
- **WHEN** an Instance declares `InstanceOf: Observation`
- **THEN** the output JSON has `"resourceType": "Observation"`

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
