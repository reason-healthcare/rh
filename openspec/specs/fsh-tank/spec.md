## ADDED Requirements

### Requirement: Collect parsed documents into an indexed FshTank
The system SHALL provide a `FshTank` that accepts one or more `FshDocument` values and indexes all entities by name. All 11 entity types SHALL be stored in separate `IndexMap` collections preserving declaration order.

#### Scenario: Add a single document
- **WHEN** a `FshDocument` containing a Profile and a ValueSet is added to an empty `FshTank`
- **THEN** the tank contains exactly one profile and one value set, accessible by name

#### Scenario: Add multiple documents
- **WHEN** multiple `FshDocument` values (from separate files) are added to a `FshTank`
- **THEN** all entities from all documents are accessible by name in the tank

#### Scenario: Duplicate entity name is an error
- **WHEN** two documents each define an entity with the same name and type
- **THEN** `add_document` returns `Err` containing a `FshError::DuplicateEntity`

### Requirement: Look up entities by name via `fish()`
The `FshTank` SHALL provide a `fish(name)` method that returns a reference to an entity matching the given name, searching across all entity type collections.

#### Scenario: Find a profile by name
- **WHEN** `fish("MyProfile")` is called and a Profile named "MyProfile" exists
- **THEN** `Some(FshEntityRef::Profile(...))` is returned

#### Scenario: Unknown name returns None
- **WHEN** `fish("Unknown")` is called and no entity with that name exists
- **THEN** `None` is returned

### Requirement: Resolve aliases throughout all rule values
The `FshResolver` SHALL substitute all alias names with their canonical URLs in every rule value across all entities in the tank.

#### Scenario: Alias substitution in binding rule
- **WHEN** an `Alias: $SCT = http://snomed.info/sct` is defined and a binding rule references `$SCT`
- **THEN** after resolution, the binding rule's system value is `http://snomed.info/sct`

#### Scenario: Unresolved alias is an error
- **WHEN** a rule references an alias name that has no corresponding `Alias` definition
- **THEN** `resolve` returns `Err` containing `FshError::UnresolvedAlias`

### Requirement: Inline RuleSet rules into referencing entities
The `FshResolver` SHALL replace every `InsertRule` in an entity's rule list with the inline expansion of the referenced `RuleSet`'s rules. Expansion SHALL be performed in topological dependency order.

#### Scenario: Simple RuleSet insert
- **WHEN** a Profile has `* insert CommonRules` and `CommonRules` is defined in the tank
- **THEN** after resolution, the Profile's rule list contains the rules from `CommonRules` in place of the `InsertRule`

#### Scenario: Nested RuleSet insert
- **WHEN** `RuleSet A` inserts `RuleSet B` and a Profile inserts `RuleSet A`
- **THEN** after resolution, the Profile contains the fully expanded rules from both A and B in correct order

#### Scenario: RuleSet cycle detection
- **WHEN** `RuleSet A` inserts `RuleSet B` and `RuleSet B` inserts `RuleSet A`
- **THEN** `resolve` returns `Err` containing `FshError::RuleSetCycle` naming both rulesets

### Requirement: Expand parameterized RuleSets with argument substitution
The `FshResolver` SHALL expand `ParamRuleSet` references by substituting named parameters into rule paths and values before inlining.

#### Scenario: Parameter substitution in rules
- **WHEN** a `ParamRuleSet` defines a rule using a parameter placeholder and is inserted with a concrete argument
- **THEN** after resolution, the inlined rules contain the concrete argument value in place of the placeholder
