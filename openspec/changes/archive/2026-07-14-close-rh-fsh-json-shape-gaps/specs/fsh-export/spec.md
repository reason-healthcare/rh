## MODIFIED Requirements

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

## ADDED Requirements

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
