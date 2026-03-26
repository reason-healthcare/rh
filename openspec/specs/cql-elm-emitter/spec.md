## ADDED Requirements

### Requirement: ElmEmitter consumes TypedLibrary and produces elm::Library

The `ElmEmitter` SHALL accept a `TypedLibrary` (from the semantic analyzer) and produce an `elm::Library`. The emitter SHALL perform only structural mapping — no symbol resolution, no type inference, no conversion logic. Every semantic decision SHALL already be encoded in the Typed AST nodes.

#### Scenario: Emit simple expression definition
- **WHEN** a `TypedLibrary` with expression definition `X: 2 + 3` (typed as Integer) is emitted
- **THEN** the resulting `elm::Library` contains an `ExpressionDef` with name `"X"` whose expression is an `Add` with two `Literal` operands

#### Scenario: Emitter has no access to symbol tables
- **WHEN** the `ElmEmitter` is constructed
- **THEN** it does not require or accept a `ScopeManager`, `ModelInfoProvider`, `TypeResolver`, `OperatorResolver`, or `ConversionRegistry`

### Requirement: System functions emit as native ELM types

System functions (`Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`, `Predecessor`, `Successor`) SHALL emit as their dedicated ELM expression types, not as `FunctionRef` nodes.

#### Scenario: Abs emits as native ELM type
- **WHEN** a Typed AST node for `Abs(-5)` is emitted
- **THEN** the ELM output is an `Abs` expression, not a `FunctionRef` with name `"Abs"`

#### Scenario: Power emits as native ELM type
- **WHEN** a Typed AST node for `2 ^ 3` is emitted
- **THEN** the ELM output is a `Power` expression with two operands

### Requirement: Negative literals emit as Negate wrapping Literal

Negative numeric literals SHALL emit as `Negate(Literal(n))` where `n` is the positive value, not as `Literal(-n)`.

#### Scenario: Negative integer literal
- **WHEN** the CQL literal `-5` is emitted
- **THEN** the ELM output is `Negate` wrapping `Literal { valueType: Integer, value: "5" }`

#### Scenario: Negative decimal literal
- **WHEN** the CQL literal `-3.14` is emitted
- **THEN** the ELM output is `Negate` wrapping `Literal { valueType: Decimal, value: "3.14" }`

### Requirement: Integer division promotes operands to Decimal

When integer division is emitted, the emitter SHALL wrap integer operands in `ToDecimal` conversion nodes to match CQL spec behavior where division always produces Decimal.

#### Scenario: Integer division promotion
- **WHEN** the CQL expression `10 / 3` is emitted (both operands Integer)
- **THEN** the ELM output is `Divide` with both operands wrapped in `ToDecimal` conversions

### Requirement: Emitter split into focused submodules

The emitter SHALL be organized into submodules by expression family, with no single file exceeding 500 lines:
- `mod.rs` — `ElmEmitter` struct, library-level emission, entry point
- `literals.rs` — literal values and quantity expressions
- `operators.rs` — unary, binary, ternary, nary operators
- `queries.rs` — query, retrieve, sort, let, relationship clauses
- `clinical.rs` — code, concept, valueset, codesystem references
- `types.rs` — type expressions (is, as, cast, convert), interval, list, tuple
- `conditionals.rs` — if/then/else, case expressions

#### Scenario: Submodule size constraint
- **WHEN** the emitter is fully implemented
- **THEN** no single source file in `src/emit/` exceeds 500 lines

### Requirement: ELM annotations and locators populated from TypedNode

When `CompilerOption::EnableAnnotations` is set, the emitter SHALL populate ELM annotation fields from the Typed AST's source spans. When `CompilerOption::EnableLocators` is set, the emitter SHALL populate the `locator` field on ELM elements with the CQL source location in `line:col-line:col` format. When `CompilerOption::EnableResultTypes` is set, the emitter SHALL populate `resultTypeName` from the Typed AST's resolved `DataType`.

#### Scenario: Locator populated from typed node span
- **WHEN** a typed expression spans lines 3:10 to 3:15 and `EnableLocators` is set
- **THEN** the emitted ELM element has `locator: "3:10-3:15"`

#### Scenario: Result type populated from typed node
- **WHEN** a typed expression has `data_type` of `System(Integer)` and `EnableResultTypes` is set
- **THEN** the emitted ELM element has `resultTypeName: "{urn:hl7-org:elm-types:r1}Integer"`

### Requirement: ELM emitter builds source-map side-channel during emission

When source-map generation is enabled, the emitter SHALL record `NodeId → ElmNodeMeta` mappings as it produces each ELM node. The emitter SHALL return the source-map data alongside the `elm::Library`.

#### Scenario: Source map populated during emission
- **WHEN** a `TypedLibrary` is emitted with source-map enabled
- **THEN** the emitter returns both an `elm::Library` and a list of `ElmNodeMeta` entries correlated to Typed AST `NodeId` values

### Requirement: Wave-2 temporal and uncertainty functions emit canonical ELM nodes

The emitter SHALL map wave-2 temporal and uncertainty functions (`TimeOfDay`, `Precision`, `LowBoundary`, `HighBoundary`) to canonical/native ELM expression forms and SHALL NOT fall back to unresolved `FunctionRef` output when a native form exists.

#### Scenario: TimeOfDay emits canonical ELM node
- **WHEN** a typed expression containing `TimeOfDay()` is emitted
- **THEN** the resulting ELM uses the canonical `TimeOfDay` expression form

#### Scenario: Precision emits canonical ELM node
- **WHEN** a typed expression containing `Precision(x)` is emitted
- **THEN** the resulting ELM uses the canonical `Precision` expression form

### Requirement: Wave-2 aggregate and list-control functions emit executable forms

The emitter SHALL produce executable ELM for `Product`, `GeometricMean`, `Repeat`, and interval `Size` expressions consistent with eval dispatch expectations.

#### Scenario: Product and GeometricMean emit executable nodes
- **WHEN** typed AST nodes for `Product` or `GeometricMean` are emitted
- **THEN** ELM output contains executable aggregate expression nodes expected by runtime dispatch

#### Scenario: Repeat and interval Size emit executable nodes
- **WHEN** typed AST nodes for `Repeat` or interval `Size` are emitted
- **THEN** ELM output contains expression forms that route to dedicated eval handlers

### Requirement: ELM includes field populated from TypedLibrary

The `ElmEmitter` SHALL populate the `elm::Library.includes` field from the `TypedLibrary.includes` collection. For each include declaration, the emitter SHALL produce an `IncludeDef` with `path`, `version`, and `localIdentifier` (alias) matching the CQL source. This field SHALL NOT be hardcoded to `None`.

#### Scenario: Single include emits IncludeDef
- **WHEN** a `TypedLibrary` has one include `include FHIRHelpers version '4.0.1' called FHIRHelpers`
- **THEN** the emitted `elm::Library.includes` contains one `IncludeDef` with `path: "FHIRHelpers"`, `version: "4.0.1"`, `localIdentifier: "FHIRHelpers"`

#### Scenario: Include with custom alias preserved
- **WHEN** a `TypedLibrary` has one include `include SomeLib called CaseLogic`
- **THEN** the emitted `IncludeDef` has `localIdentifier: "CaseLogic"` and `path: "SomeLib"`

#### Scenario: No includes emits empty list not None
- **WHEN** a `TypedLibrary` has no include declarations
- **THEN** `elm::Library.includes` is an empty list (not `None`)
