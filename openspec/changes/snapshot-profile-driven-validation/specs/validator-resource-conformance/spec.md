## MODIFIED Requirements

### Requirement: Validator detects R4 resource shape errors

The validator SHALL report deterministic issues for unknown R4 resource types,
unknown properties on known R4 resource types, and invalid use of FHIR choice
elements. These checks SHALL be core validator behavior and apply whenever
`rh-validator` validates an R4 resource, including standalone resource
validation and package validation. Property and choice validation SHALL use the
active base/profile StructureDefinition snapshot paths so nested backbone
elements and datatypes are checked as well as root resource fields.

#### Scenario: Unknown resource type is reported
- **WHEN** a resource has a `resourceType` that is not an R4 resource type
- **THEN** validation reports an ERROR identifying the unknown resource type

#### Scenario: Unknown root property is reported
- **WHEN** a known R4 resource contains a root property that is not defined for that resource type
- **THEN** validation reports an ERROR identifying the property path

#### Scenario: Unknown nested backbone property is reported
- **WHEN** a PlanDefinition contains `action.relatedAction.description`
- **THEN** validation reports an ERROR identifying `PlanDefinition.action.relatedAction.description` as an unknown property

#### Scenario: Invalid choice element is reported
- **WHEN** a resource uses a choice element name that is not valid for the containing resource type
- **THEN** validation reports an ERROR identifying the invalid choice path

#### Scenario: Valid nested backbone property is accepted
- **WHEN** a PlanDefinition contains `action.relatedAction.actionId`, `relationship`, and `offsetDuration`
- **THEN** validation does not report unknown-property errors for those fields

## ADDED Requirements

### Requirement: Validator validates known Expression language codes

The validator SHALL validate `Expression.language` values against the locally
known R4 expression-language code system when the expression language value is
present on a resource, backbone element, datatype, or extension value.

#### Scenario: Unknown Expression language code is reported
- **WHEN** a resource contains an Expression with `language` set to `text/cql-identifier`
- **THEN** validation reports an ERROR identifying the unknown expression-language code

#### Scenario: Known Expression language code is accepted
- **WHEN** a resource contains an Expression with `language` set to `text/cql` or `text/fhirpath`
- **THEN** validation does not report an expression-language code error

#### Scenario: Expression language validation does not require remote terminology
- **WHEN** no terminology server is configured
- **THEN** validation still reports locally known invalid expression-language codes
