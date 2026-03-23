## ADDED Requirements

### Requirement: SemanticAnalyzer produces TypedLibrary from parser AST

The `SemanticAnalyzer` SHALL accept a `parser::ast::Library` and produce a `TypedLibrary`. The analyzer SHALL perform a single walk of the parser AST, resolving all symbols, types, operator overloads, and implicit conversions, and producing a fully typed intermediate representation.

#### Scenario: Analyze a simple expression definition
- **WHEN** a CQL library containing `define X: 2 + 3` is analyzed
- **THEN** the resulting `TypedLibrary` contains an expression definition `X` whose body is a `TypedNode<TypedExpression>` with `data_type` of `DataType::System(SystemType::Integer)`

#### Scenario: Analyze fails on undefined reference
- **WHEN** a CQL library containing `define X: Y + 1` is analyzed and `Y` is not defined
- **THEN** the analyzer emits a semantic error diagnostic identifying the undefined symbol `Y` with its source location

### Requirement: Symbol resolution via ScopeManager

The semantic analyzer SHALL use a `ScopeManager` (extracted from the current `LibraryBuilder`) to resolve identifiers. The `ScopeManager` SHALL maintain a stack of `Scope` maps supporting:
- Pushing/popping scopes for nested contexts (queries, functions, let clauses)
- Registering symbols with their `SymbolKind` (Expression, Function, Parameter, CodeSystem, ValueSet, Code, Concept, QueryAlias, Let, Operand)
- Resolving symbols by name with innermost-scope-wins shadowing
- Resolving qualified references (library-qualified identifiers via included libraries)

#### Scenario: Local definition resolution
- **WHEN** a CQL library defines `define A: 1` and `define B: A + 2`
- **THEN** the reference to `A` in `B`'s body resolves to the expression definition `A`

#### Scenario: Query alias scoping
- **WHEN** a CQL query `from [Observation] O where O.status = 'final'` is analyzed
- **THEN** `O` resolves as a `QueryAlias` within the query's where clause but is not visible outside the query

#### Scenario: Included library qualified reference
- **WHEN** a CQL library includes `FHIRHelpers` and references `FHIRHelpers.ToString(x)`
- **THEN** the qualified reference resolves to the `ToString` function from the `FHIRHelpers` compiled library

### Requirement: Type inference resolves types on all expression nodes

The semantic analyzer SHALL resolve the `DataType` for every expression node using the existing `TypeResolver` and `ModelInfoProvider`. Every `TypedNode` in the output SHALL have a non-`Unknown` `data_type` unless the expression genuinely cannot be typed (in which case a diagnostic is emitted).

#### Scenario: Arithmetic type inference
- **WHEN** the expression `2 + 3.0` is analyzed
- **THEN** the result type is `DataType::System(SystemType::Decimal)` and the left operand `2` has an implicit `ToDecimal` conversion recorded in `SemanticMeta`

#### Scenario: Property access type inference
- **WHEN** the expression `O.status` is analyzed where `O` is a FHIR `Observation`
- **THEN** the result type is resolved from the FHIR R4 model info for the `status` property of `Observation`

#### Scenario: Unknown type with diagnostic
- **WHEN** a property `O.nonExistentField` is analyzed where `nonExistentField` does not exist in the model
- **THEN** the `data_type` is `DataType::Unknown` and a diagnostic warning is emitted

### Requirement: Operator overload resolution uses OperatorResolver

The semantic analyzer SHALL use the existing `OperatorResolver` to resolve binary and unary operator overloads. When multiple overloads are candidates, the analyzer SHALL select the most specific match per CQL spec rules and record the selected overload in `SemanticMeta.resolved_overload`.

#### Scenario: Binary operator overload
- **WHEN** the expression `2 + 3` is analyzed
- **THEN** the `+` operator resolves to `Add(Integer, Integer)` returning `Integer`

#### Scenario: Overload with implicit conversion
- **WHEN** the expression `2 + 3.0` is analyzed
- **THEN** the `+` operator resolves to `Add(Decimal, Decimal)` with implicit `ToDecimal` conversion on the left operand

### Requirement: Implicit conversions applied via ConversionRegistry

The semantic analyzer SHALL use the existing `ConversionRegistry` to apply implicit conversions when operand types do not match operator/function signatures. Applied conversions SHALL be recorded in `SemanticMeta.implicit_conversions`. The analyzer SHALL respect `CompilerOption::DisableImplicitConversions`, `DisableListPromotion`, and `DisableListDemotion` options.

#### Scenario: Implicit integer to decimal conversion
- **WHEN** an integer value is passed where a decimal is expected
- **THEN** the analyzer applies a `ToDecimal` implicit conversion and records it in the node's `SemanticMeta`

#### Scenario: Implicit conversions disabled
- **WHEN** `CompilerOption::DisableImplicitConversions` is set and an integer is passed where a decimal is expected
- **THEN** the analyzer emits a type mismatch error instead of applying an implicit conversion

### Requirement: FHIR type resolution uses ModelInfoProvider

The semantic analyzer SHALL resolve FHIR model types using the `ModelInfoProvider` trait (reused from `provider.rs`). Property access, retrieve data types, and context types SHALL be resolved through the model provider rather than string-matching heuristics.

#### Scenario: Retrieve data type resolution
- **WHEN** a retrieve expression `[Observation]` is analyzed with FHIR R4 model
- **THEN** the result type is `DataType::List(Box::new(DataType::Model { namespace: "FHIR", name: "Observation" }))`

#### Scenario: Context type resolution
- **WHEN** the context is set to `Patient` in a FHIR R4 model
- **THEN** the context type resolves to `DataType::Model { namespace: "FHIR", name: "Patient" }`

### Requirement: Diagnostics emitted as structured CqlCompilerException

The semantic analyzer SHALL emit diagnostics using the existing `CqlCompilerException` type with `Severity` (Error, Warning, Info), `ExceptionType::Semantic`, and `SourceLocator` pointing to the source location of the issue. All diagnostics SHALL be collected and returned alongside the `TypedLibrary` (analysis does not abort on first error).

#### Scenario: Multiple errors collected
- **WHEN** a CQL library has three undefined references
- **THEN** the analyzer emits three separate diagnostic errors, one per undefined reference, and returns all three alongside a partial `TypedLibrary`

#### Scenario: Diagnostics include source location
- **WHEN** an error occurs at line 10, column 5 of the CQL source
- **THEN** the diagnostic's `SourceLocator` points to line 10, column 5

### Requirement: Wave-2 semantic registration covers targeted operator/function set

Semantic analysis SHALL register and resolve wave-2 function-syntax operators and unary forms (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`, `Product`, `GeometricMean`, `Repeat`, and interval `Size` where applicable) so they compile through normal overload resolution.

#### Scenario: Wave-2 function resolves during semantic analysis
- **WHEN** a CQL expression uses a wave-2 function (for example `Product({1,2,3})`)
- **THEN** semantic analysis resolves the call without unknown-function diagnostics

#### Scenario: Wave-2 temporal function resolves with operand typing
- **WHEN** a CQL expression uses `Precision(x)` or `LowBoundary(x)`
- **THEN** semantic analysis resolves overloads based on operand type and records the selected signature

### Requirement: Wave-2 overload selection remains deterministic

For wave-2 functions/operators with multiple candidates, semantic resolution SHALL select deterministic overloads for equivalent typed inputs and apply existing implicit-conversion policy consistently.

#### Scenario: Equivalent inputs resolve to identical overload
- **WHEN** the same wave-2 expression is analyzed repeatedly with the same input types
- **THEN** the same overload is selected each time

#### Scenario: Conversion policy is respected
- **WHEN** an implicit conversion is required for a wave-2 operator and conversions are disabled by compiler option
- **THEN** semantic analysis emits a type error rather than silently coercing values
