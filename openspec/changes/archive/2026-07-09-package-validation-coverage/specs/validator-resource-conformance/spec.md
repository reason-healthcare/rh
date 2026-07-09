## ADDED Requirements

### Requirement: Validator detects R4 resource shape errors

The validator SHALL report deterministic issues for unknown R4 resource types,
unknown properties on known R4 resource types, and invalid use of FHIR choice
elements. These checks SHALL be core validator behavior and apply whenever
`rh-validator` validates an R4 resource, including standalone resource
validation and package validation.

#### Scenario: Unknown resource type is reported
- **WHEN** a resource has a `resourceType` that is not an R4 resource type
- **THEN** validation reports an ERROR identifying the unknown resource type

#### Scenario: Unknown property is reported
- **WHEN** a known R4 resource contains a property that is not defined for that resource type
- **THEN** validation reports an ERROR identifying the property path

#### Scenario: Invalid choice element is reported
- **WHEN** a resource uses a choice element name that is not valid for the containing resource type
- **THEN** validation reports an ERROR identifying the invalid choice path

### Requirement: Validator enforces profile fixed and pattern constraints

The validator SHALL enforce fixed and pattern constraints from applicable
StructureDefinition snapshots for package resources.

#### Scenario: Fixed value mismatch is reported
- **WHEN** a resource element differs from a `fixed[x]` value required by an applicable profile
- **THEN** validation reports an ERROR at the constrained element path

#### Scenario: Pattern mismatch is reported
- **WHEN** a resource element does not satisfy a `pattern[x]` value required by an applicable profile
- **THEN** validation reports an ERROR at the constrained element path

### Requirement: Validator enforces reference target constraints

The validator SHALL validate Reference values against target profile/type
constraints that are resolvable from base R4 metadata, package-local profiles,
or installed dependency profiles.

#### Scenario: Reference target type mismatch is reported
- **WHEN** a profile constrains a Reference to Patient and the resource contains `Organization/example`
- **THEN** validation reports an ERROR identifying the reference path and expected target type

#### Scenario: Valid local reference target passes target type check
- **WHEN** a profile constrains a Reference to Patient and the resource contains `Patient/example`
- **THEN** validation does not report a target type error for that reference

### Requirement: Validator applies slice-aware profile rules

The validator SHALL match sliced elements using applicable discriminators and
apply slice-specific cardinality, binding, fixed, and pattern rules to matched
slices.

#### Scenario: Slice cardinality violation is reported
- **WHEN** a required slice is absent from a profiled resource
- **THEN** validation reports an ERROR for the missing slice

#### Scenario: Slice-specific pattern is applied
- **WHEN** a repeated element matches a named slice and violates that slice's pattern constraint
- **THEN** validation reports an ERROR for the matched slice path

#### Scenario: Element matching no closed slice is reported
- **WHEN** a closed slicing definition is used and an element matches no allowed slice
- **THEN** validation reports an ERROR identifying the unsliced element

### Requirement: Validator validates QuestionnaireResponse against Questionnaire

The validator SHALL validate QuestionnaireResponse resources against their
referenced Questionnaire when that Questionnaire is available locally or from
loaded package context. Standalone validation SHALL NOT implicitly scan sibling
files or package resources for Questionnaire context; callers that need local
context SHALL register or otherwise load that context explicitly.

#### Scenario: Missing required answer is reported
- **WHEN** a Questionnaire item requires an answer and the QuestionnaireResponse omits it
- **THEN** validation reports an ERROR for the missing answer

#### Scenario: Answer type mismatch is reported
- **WHEN** a Questionnaire item expects a boolean answer and the QuestionnaireResponse provides a string answer
- **THEN** validation reports an ERROR for the mismatched answer type

#### Scenario: Answer option mismatch is reported
- **WHEN** a Questionnaire item restricts answers to listed answer options and the QuestionnaireResponse uses a different answer
- **THEN** validation reports an ERROR for the unsupported answer

#### Scenario: Questionnaire reference constraint is enforced
- **WHEN** a Questionnaire item restricts reference answer resource types and the QuestionnaireResponse uses a disallowed reference type
- **THEN** validation reports an ERROR for the reference answer

### Requirement: Validator resolves local ValueSets for binding checks

The validator SHALL use package-local and dependency-loaded ValueSet resources
for binding checks when the binding ValueSet can be resolved without remote
terminology access. Versioned and unversioned references to the same ValueSet
canonical SHALL match for local lookup. Standalone validation SHALL NOT
implicitly scan sibling files or package resources for ValueSet context; callers
that need local context SHALL register or otherwise load that context
explicitly.

#### Scenario: Local expansion contains code
- **WHEN** a required binding references a local ValueSet expansion that contains the resource code
- **THEN** validation does not report a binding error for that code

#### Scenario: Local compose contains code
- **WHEN** a required binding references a local ValueSet compose include that contains the resource code
- **THEN** validation does not report a binding error for that code

#### Scenario: Required binding code missing from local ValueSet is reported
- **WHEN** a required binding references a local ValueSet and the resource code is not included
- **THEN** validation reports an ERROR for the binding violation

#### Scenario: Versioned ValueSet canonical matches local ValueSet
- **WHEN** a binding references `http://example.org/ValueSet/colors|1.0.0` and the package contains `http://example.org/ValueSet/colors`
- **THEN** validation uses the local ValueSet for the binding check

### Requirement: Validator keeps broad validation areas out of scope

This change SHALL NOT introduce broad Measure, MeasureReport, Bundle, or
narrative validation behavior beyond checks already provided by existing
validation code.

#### Scenario: Measure validation is not expanded
- **WHEN** this change is implemented
- **THEN** no new Measure or MeasureReport validation category is claimed by this capability

#### Scenario: Bundle validation is not expanded
- **WHEN** this change is implemented
- **THEN** no new broad Bundle validation category is claimed by this capability

#### Scenario: Narrative validation is not expanded
- **WHEN** this change is implemented
- **THEN** no new narrative validation category is claimed by this capability
