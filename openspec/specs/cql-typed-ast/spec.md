## ADDED Requirements

### Requirement: TypedNode wrapper carries semantic metadata on every node

Every node in the Typed AST SHALL be wrapped in a `TypedNode<T>` struct that carries:
- `node_id`: A stable `NodeId` unique within the compilation unit
- `data_type`: The resolved `DataType` for this node
- `span`: A `SourceSpan` with start and end `SourceLocation` (line, column, byte offset)
- `meta`: A `SemanticMeta` record capturing semantic decisions made during analysis
- `inner`: The underlying typed expression or definition `T`

#### Scenario: Typed node carries resolved type
- **WHEN** a CQL expression `2 + 3` is analyzed
- **THEN** the resulting `TypedNode` for the binary addition has `data_type` of `DataType::System(SystemType::Integer)`

#### Scenario: Typed node carries source span with start and end
- **WHEN** a CQL expression spanning lines 5-7 is analyzed
- **THEN** the resulting `TypedNode` has `span.start` pointing to line 5 and `span.end` pointing to line 7

#### Scenario: Typed node carries stable node ID
- **WHEN** the same CQL library is analyzed twice with identical source
- **THEN** each `TypedNode` receives the same `NodeId` in both analyses

### Requirement: TypedExpression enum mirrors parser AST Expression variants

The Typed AST SHALL define a `TypedExpression` enum with variants corresponding to each parser `Expression` variant. Every variant that exists in `parser::ast::Expression` SHALL have a corresponding `TypedExpression` variant. The typed variants SHALL reference other `TypedNode<TypedExpression>` for child expressions, not raw parser AST nodes.

#### Scenario: All parser expression variants have typed equivalents
- **WHEN** a new variant is added to `parser::ast::Expression`
- **THEN** compilation fails until a corresponding `TypedExpression` variant is added and handled in the semantic analyzer's exhaustive match

#### Scenario: Typed expressions reference typed children
- **WHEN** a binary expression `a + b` is represented in the Typed AST
- **THEN** both operands are `TypedNode<TypedExpression>`, not `parser::ast::Expression`

### Requirement: SemanticMeta captures analysis decisions

The `SemanticMeta` struct SHALL record the semantic decisions made during analysis of each node:
- `resolved_overload`: Which operator/function overload was selected (if applicable)
- `implicit_conversions`: List of implicit conversions applied to operands
- `list_promotion`: Whether list promotion or demotion was applied
- `resolved_symbol`: The resolved symbol reference (if this node is an identifier)

#### Scenario: Overload resolution recorded
- **WHEN** the expression `2 + 3.0` is analyzed and the `+` operator resolves to `Add(Decimal, Decimal)` with implicit `ToDecimal` on the left operand
- **THEN** the `SemanticMeta` on the addition node records `resolved_overload` as `Add(Decimal, Decimal)` and `implicit_conversions` includes `ToDecimal` applied to the left operand

#### Scenario: No semantic decisions on literals
- **WHEN** a literal `42` is analyzed
- **THEN** the `SemanticMeta` has empty `resolved_overload`, empty `implicit_conversions`, and no `list_promotion`

### Requirement: TypedLibrary represents a fully analyzed CQL library

The Typed AST SHALL define a `TypedLibrary` struct containing:
- Library identifier and version
- Using definitions (model references)
- Include definitions (library dependencies)
- Terminology definitions (codesystem, valueset, code, concept)
- Parameter definitions with resolved types
- Context definitions
- Expression and function definitions as `TypedNode<TypedExpression>`

#### Scenario: Complete library representation
- **WHEN** a CQL library with identifier, using, include, parameter, and expression definitions is analyzed
- **THEN** the resulting `TypedLibrary` contains all definitions with resolved types on every expression node

### Requirement: Typed AST is immutable after construction

The Typed AST types SHALL be immutable — all fields owned, no interior mutability. Once the semantic analyzer produces a `TypedLibrary`, it SHALL NOT be modified.

#### Scenario: Immutable typed AST
- **WHEN** a `TypedLibrary` is produced by the semantic analyzer
- **THEN** all fields are accessible via shared references only; no `&mut` access is provided
