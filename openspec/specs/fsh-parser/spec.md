## ADDED Requirements

### Requirement: Parse FSH source text into a typed AST
The system SHALL parse FSH source text (a string or file contents) into a `FshDocument` containing a `Vec<Spanned<FshEntity>>`. All 11 FSH entity types SHALL be supported: Alias, Profile, Extension, Logical, Resource, Instance, ValueSet, CodeSystem, Invariant, RuleSet, and ParamRuleSet.

#### Scenario: Parse a Profile entity
- **WHEN** the parser receives FSH text containing a `Profile` declaration with metadata and rules
- **THEN** a `FshEntity::Profile` is returned with the correct name, parent, id, title, description, and rule list

#### Scenario: Parse an Extension entity
- **WHEN** the parser receives FSH text containing an `Extension` declaration
- **THEN** a `FshEntity::Extension` is returned with all metadata and rules

#### Scenario: Parse an Instance entity
- **WHEN** the parser receives FSH text containing an `Instance` declaration with `InstanceOf` and `Usage`
- **THEN** a `FshEntity::Instance` is returned with `instance_of`, `usage`, and assignment rules

#### Scenario: Parse a ValueSet entity
- **WHEN** the parser receives FSH text containing a `ValueSet` declaration with include/exclude component rules
- **THEN** a `FshEntity::ValueSet` is returned with all component rules

#### Scenario: Parse a CodeSystem entity
- **WHEN** the parser receives FSH text containing a `CodeSystem` declaration with concept rules
- **THEN** a `FshEntity::CodeSystem` is returned with all concept definitions

#### Scenario: Parse a RuleSet entity
- **WHEN** the parser receives FSH text containing a `RuleSet` declaration
- **THEN** a `FshEntity::RuleSet` is returned with its contained rules

#### Scenario: Parse a ParamRuleSet entity
- **WHEN** the parser receives FSH text containing a parameterized `RuleSet` declaration with named parameters
- **THEN** a `FshEntity::ParamRuleSet` is returned with parameter names and templated rules

### Requirement: Parse all FSH rule types
The system SHALL parse all FSH rule types into typed Rust variants. Supported rule types SHALL include: CardRule, FlagRule, BindingRule, AssignmentRule, ContainsRule, OnlyRule, ObeysRule, CaretValueRule, InsertRule, AddElementRule, PathRule, ConceptRule, ValueSetComponentRule, and MappingRule.

#### Scenario: Parse a cardinality rule
- **WHEN** a rule line has the form `* path M..N` (with optional flags)
- **THEN** a `CardRule` is returned with correct `min`, `max`, and `flags`

#### Scenario: Parse an assignment rule with a coded value
- **WHEN** a rule line has the form `* path = system#code "display"`
- **THEN** an `AssignmentRule` is returned with a `FshValue::Code` containing `system`, `code`, and `display`

#### Scenario: Parse a binding rule
- **WHEN** a rule line has the form `* path from ValueSetName (required)`
- **THEN** a `BindingRule` is returned with the correct value set name and binding strength

#### Scenario: Parse a caret value rule
- **WHEN** a rule line has the form `* path ^property = value`
- **THEN** a `CaretValueRule` is returned with the correct path, caret path, and value

#### Scenario: Parse an insert rule
- **WHEN** a rule line has the form `* insert RuleSetName`
- **THEN** an `InsertRule` is returned referencing the named RuleSet

#### Scenario: Parse an add element rule
- **WHEN** a rule line has the form `* path[x] 0..1 SomeType "short description"`
- **THEN** an `AddElementRule` is returned with correct path, cardinality, type, and description

### Requirement: Track source locations on all AST nodes
The parser SHALL attach `SourceRange` (start and end `SourceLocation` with line, column, and byte offset) to every `Spanned<T>` wrapper in the AST.

#### Scenario: Source location on entity
- **WHEN** a `Profile` is parsed from line 3 of a source file
- **THEN** the returned `Spanned<FshEntity>` has `loc.start.line == 3`

#### Scenario: Source location on rule
- **WHEN** a rule is parsed from a specific line in the source
- **THEN** the rule's `SourceRange` reflects the correct start line and column

### Requirement: Report parse errors with source location
The parser SHALL return a `FshError::Parse` containing the error message and `SourceLocation` when input cannot be parsed.

#### Scenario: Invalid entity keyword
- **WHEN** the input contains an unrecognized top-level keyword
- **THEN** the parser returns `Err(FshError::Parse { location, message })`

#### Scenario: Malformed cardinality
- **WHEN** a rule line contains a malformed cardinality expression
- **THEN** the parser returns `Err(FshError::Parse { location, message })`

### Requirement: Parse FSH paths into typed segments
The parser SHALL parse FSH element paths into a `FshPath` consisting of typed `FshPathSegment` variants: `Name`, `Index`, `Slice`, `ChoiceType`, and `Extension`.

#### Scenario: Parse a dotted path
- **WHEN** a path like `subject.reference` is parsed
- **THEN** a `FshPath` with two `Name` segments is returned

#### Scenario: Parse a path with slice
- **WHEN** a path like `component[systolicBP].value[x]` is parsed
- **THEN** a `FshPath` with `Name`, `Slice`, `Name`, and `ChoiceType` segments is returned

### Requirement: Parse multiline string literals
The parser SHALL correctly parse FSH triple-quoted multiline strings (`"""..."""`) as `FshValue::Str`, preserving internal whitespace.

#### Scenario: Multiline string in description
- **WHEN** a `Description` metadata line contains a `"""..."""` value spanning multiple lines
- **THEN** the string content is captured correctly with internal newlines preserved
