## MODIFIED Requirements

### Requirement: Symbol resolution via ScopeManager

The semantic analyzer SHALL use a `ScopeManager` (extracted from the current `LibraryBuilder`) to resolve identifiers. The `ScopeManager` SHALL maintain a stack of `Scope` maps supporting:
- Pushing/popping scopes for nested contexts (queries, functions, let clauses)
- Registering symbols with their `SymbolKind` (Expression, Function, Parameter, CodeSystem, ValueSet, Code, Concept, QueryAlias, Let, Operand, Include)
- Resolving symbols by name with innermost-scope-wins shadowing
- Resolving qualified references (library-qualified identifiers via included libraries)
- Reclassifying `MemberInvocation` nodes as `QualifiedIdentifierRef` when the source identifier resolves to `SymbolKind::Include`

#### Scenario: Local definition resolution
- **WHEN** a CQL library defines `define A: 1` and `define B: A + 2`
- **THEN** the reference to `A` in `B`'s body resolves to the expression definition `A`

#### Scenario: Query alias scoping
- **WHEN** a CQL query `from [Observation] O where O.status = 'final'` is analyzed
- **THEN** `O` resolves as a `QueryAlias` within the query's where clause but is not visible outside the query

#### Scenario: Included library qualified reference
- **WHEN** a CQL library includes `FHIRHelpers` and references `FHIRHelpers.ToString(x)`
- **THEN** the qualified reference resolves to the `ToString` function from the `FHIRHelpers` compiled library

#### Scenario: MemberInvocation on include alias reclassified as QualifiedIdentifierRef
- **WHEN** a CQL library includes `SomeLib called CaseLogic` and an expression body contains `CaseLogic."My Expression"`
- **THEN** the semantic analyzer detects that `CaseLogic` resolves to `SymbolKind::Include`, reclassifies the `MemberInvocation` as a `QualifiedIdentifierRef { qualifier: "CaseLogic", name: "My Expression" }`, and does NOT emit an undefined-symbol diagnostic

#### Scenario: MemberInvocation on non-alias identifier remains property access
- **WHEN** a CQL library contains `Patient.name` where `Patient` is not a library alias
- **THEN** the node remains a `MemberInvocation` (property access) and is NOT reclassified
