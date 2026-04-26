## 1. Typed AST Definition

- [x] 1.1 Create `src/semantics/` directory and `src/semantics/mod.rs` with submodule declarations for `typed_ast` and `scope`
- [x] 1.2 Define `NodeId` type (stable, hash-based identifier) in `src/semantics/typed_ast.rs`
- [x] 1.3 Define `SourceSpan` struct with start and end `SourceLocation` (line, column, byte offset) in `src/semantics/typed_ast.rs`
- [x] 1.4 Define `SemanticMeta` struct with `resolved_overload`, `implicit_conversions`, `list_promotion`, `resolved_symbol` fields in `src/semantics/typed_ast.rs`
- [x] 1.5 Define `TypedNode<T>` wrapper struct with `node_id: NodeId`, `data_type: DataType`, `span: SourceSpan`, `meta: SemanticMeta`, `inner: T` in `src/semantics/typed_ast.rs`
- [x] 1.6 Define `TypedExpression` enum mirroring all `parser::ast::Expression` variants (Literal, IdentifierRef, QualifiedIdentifierRef, UnaryExpression, BinaryExpression, TernaryExpression, DateTimeComponentFrom, TypeExpression, TimingExpression, FunctionInvocation, MemberInvocation, IndexInvocation, Query, Retrieve, IfThenElse, Case, IntervalExpression, ListExpression, TupleExpression, Instance, Let, Parenthesized) with children as `TypedNode<TypedExpression>`
- [x] 1.7 Define `TypedStatement` enum (TypedExpressionDef, TypedFunctionDef) with typed bodies
- [x] 1.8 Define `TypedLibrary` struct with library identifier, version, using defs, include defs, terminology defs, parameter defs, context defs, and `Vec<TypedNode<TypedStatement>>`
- [x] 1.9 Add `semantics` module declaration to `src/lib.rs` and re-export key types
- [x] 1.10 Write unit tests verifying `TypedNode` construction, `NodeId` stability, and `SemanticMeta` defaults

## 2. Scope Management Extraction

- [x] 2.1 Create `src/semantics/scope.rs` with `Symbol`, `Scope`, `SymbolKind` structs moved from `builder.rs`
- [x] 2.2 Implement `ScopeManager` struct with `push_scope()`, `pop_scope()`, `register_symbol()`, `resolve_symbol()` methods
- [x] 2.3 Implement qualified reference resolution (library-qualified identifiers) in `ScopeManager`
- [x] 2.4 Implement query alias scope management (push/pop for query contexts) in `ScopeManager`
- [x] 2.5 Write unit tests for scope nesting, shadowing, qualified resolution, and query alias scoping
- [x] 2.6 Update `builder.rs` to use `ScopeManager` from `semantics/scope.rs` instead of inline scope logic (temporary compatibility bridge)

## 3. Semantic Analyzer

- [x] 3.1 Create `src/semantics/analyzer.rs` with `SemanticAnalyzer` struct holding `ScopeManager`, `ModelInfoProvider` reference, `TypeResolver`, `OperatorResolver`, `ConversionRegistry`, `CompilerOptions`, and diagnostics collector
- [x] 3.2 Implement `SemanticAnalyzer::analyze()` entry point: accept `parser::ast::Library`, return `(TypedLibrary, Vec<CqlCompilerException>)`
- [x] 3.3 Implement library-level analysis: process using defs, include defs, register included library symbols
- [x] 3.4 Implement terminology definition analysis: codesystem, valueset, code, concept definitions with type resolution
- [x] 3.5 Implement parameter definition analysis with type resolution and default expression analysis
- [x] 3.6 Implement context definition analysis with model-driven type resolution
- [x] 3.7 Implement expression definition analysis: analyze body expression, register in scope with resolved type
- [x] 3.8 Implement function definition analysis: analyze parameters, body, register overload
- [x] 3.9 Implement `analyze_expression()` exhaustive match over `parser::ast::Expression` variants dispatching to variant-specific analyzers
- [x] 3.10 Implement literal analysis: resolve types for all literal variants (Null, Boolean, Integer, Long, Decimal, String, Date, DateTime, Time, Quantity, Ratio, Code)
- [x] 3.11 Implement identifier/reference analysis: resolve via `ScopeManager`, record resolved symbol in `SemanticMeta`
- [x] 3.12 Implement unary expression analysis: resolve operator overload via OperatorResolver, apply implicit conversions, record in SemanticMeta
- [x] 3.13 Implement binary expression analysis: resolve operator overload, apply implicit conversions, handle type promotion
- [x] 3.14 Implement ternary expression analysis: resolve operator overload, record conversions
- [x] 3.15 Implement function invocation analysis: resolve function by name, match overload, apply argument conversions
- [x] 3.16 Implement member invocation analysis: resolve property type via `ModelInfoProvider`
- [x] 3.17 Implement index invocation analysis
- [x] 3.18 Implement query analysis: push query scope, analyze sources, let clauses, where clause, return clause, sort clause, relationship clauses, pop scope
- [x] 3.19 Implement retrieve analysis: resolve data type from model, resolve terminology references
- [x] 3.20 Implement if/then/else analysis: analyze condition (Boolean), then/else branches, unify result types
- [x] 3.21 Implement case analysis: analyze comparand, case items, else, unify result types
- [x] 3.22 Implement interval expression analysis: analyze low/high bounds, resolve point type
- [x] 3.23 Implement list expression analysis: analyze elements, resolve common element type
- [x] 3.24 Implement tuple expression analysis: analyze element expressions, build tuple type
- [x] 3.25 Implement instance expression analysis: resolve class type from model, analyze property values
- [x] 3.26 Implement type expression analysis (Is, As, Cast, Convert): resolve target type
- [x] 3.27 Implement timing expression analysis: resolve timing phrase semantics, operand types
- [x] 3.28 Implement DateTimeComponentFrom analysis: resolve component extraction type
- [x] 3.29 Replace FHIR type string-matching heuristics (`infer_fhir_result_type`) with `ModelInfoProvider`-driven type resolution
- [x] 3.30 Write integration tests: analyze sample CQL libraries, verify TypedLibrary output types match expectations
- [x] 3.31 Write integration tests: verify diagnostics emitted for undefined references, type mismatches, ambiguous overloads

## 4. ELM Emitter Rewrite

- [x] 4.1 Create `src/emit/mod.rs` with `ElmEmitter` struct (holds `local_id_counter`, `CompilerOptions`, optional source-map builder)
- [x] 4.2 Implement `ElmEmitter::emit()` entry point: accept `TypedLibrary`, return `elm::Library`
- [x] 4.3 Implement library-level emission: identifiers, usings, includes, parameters, codesystems, valuesets, codes, concepts, contexts, statements
- [x] 4.4 Create `src/emit/literals.rs`: emit Literal values, Quantity, Ratio — implement `Negate(Literal(n))` for negative literals
- [x] 4.5 Create `src/emit/operators.rs`: emit unary, binary, ternary, nary operators — emit system functions (`Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`, `Predecessor`, `Successor`) as native ELM types instead of `FunctionRef`
- [x] 4.6 Create `src/emit/operators.rs`: implement integer division `Decimal` promotion (`ToDecimal` wrappers on operands)
- [x] 4.7 Create `src/emit/queries.rs`: emit Query, Retrieve, SortClause, LetClause, RelationshipClause
- [x] 4.8 Create `src/emit/clinical.rs`: emit CodeSystemRef, ValueSetRef, CodeRef, ConceptRef
- [x] 4.9 Create `src/emit/types.rs`: emit type expressions (Is, As, Convert), IntervalExpression, ListExpression, TupleExpression, Instance
- [x] 4.10 Create `src/emit/conditionals.rs`: emit If, Case expressions
- [x] 4.11 Implement locator population from `TypedNode.span` when `EnableLocators` is set
- [x] 4.12 Implement `resultTypeName` population from `TypedNode.data_type` when `EnableResultTypes` is set
- [x] 4.13 Implement annotation population when `EnableAnnotations` is set
- [x] 4.14 Verify no emitter submodule exceeds 500 lines
- [x] 4.15 Write unit tests for each emitter submodule against known ELM output expectations
- [x] 4.16 Write conformance comparison tests: emit ELM from sample CQL, compare with Java reference translator output for the 10 comparison files

## 5. Pipeline Wiring and Migration

- [x] 5.1 Refactor `builder.rs` into thin orchestrator: `parse → analyze → emit` pipeline, delegating to `SemanticAnalyzer` and `ElmEmitter`
- [x] 5.2 Update `compiler.rs` `compile()` to use new three-stage pipeline
- [x] 5.3 Update `compiler.rs` `compile_with_model()` to use new pipeline
- [x] 5.4 Update `compiler.rs` `validate()` to use `SemanticAnalyzer` directly (parse + analyze, no emit)
- [x] 5.5 Update `compiler.rs` `compile_to_json()` to use new pipeline
- [x] 5.6 Run full conformance suite against new pipeline, compare with old pipeline output
- [x] 5.7 Remove old `translate_expr_recursive` dispatch from builder.rs
- [x] 5.8 Remove or significantly reduce old `translator.rs` (replaced by `src/emit/`)
- [x] 5.9 Update `lib.rs` re-exports: add new types (`SemanticAnalyzer`, `TypedLibrary`, `ScopeManager`, `ElmEmitter`), deprecate or remove old re-exports (`ExpressionTranslator`)
- [x] 5.10 Verify all existing examples in `examples/` compile and run with updated API
- [x] 5.11 Reduce `builder.rs` to <500 lines (thin orchestrator + library dependency management)

## 6. Parser Conformance Improvements

- [x] 6.1 Add `predecessor of` / `successor of` keyword parsing as unary expressions
- [x] 6.2 Add `^` power operator in expression parser with correct precedence (above multiplicative, below unary)
- [x] 6.3 Add `minimum <TypeSpecifier>` / `maximum <TypeSpecifier>` expression parsing
- [x] 6.4 Add `aggregate` clause parsing in query expressions
- [x] 6.5 Fix `let` clause parsing in queries (multiple let clauses, correct scoping)
- [x] 6.6 Add `starts` / `ends` timing phrase qualifier parsing
- [x] 6.7 Add `occurs` timing phrase qualifier parsing
- [x] 6.8 Add `properly` modifier parsing for timing/interval phrases
- [x] 6.9 Write parser unit tests for each new production
- [x] 6.10 Run jvmTest conformance suite, verify ≥90% pass rate (≥107/119 files)
- [x] 6.11 Verify no regressions in previously passing 90 test files

## 7. Source-Map Infrastructure

- [x] 7.1 Create `src/sourcemap.rs` with `SourceMap`, `SourceDocument`, `ElmNodeMeta`, `SourceElmMapping` structs per PLAN.md §6.1 schema
- [x] 7.2 Implement stable hash-based ID generation for `doc_id` and `elm_node_id`
- [x] 7.3 Implement `SourceElmMapping` role variants: `direct`, `implicit-conversion`, `desugared`, `synthetic`
- [x] 7.4 Implement source-map builder in `ElmEmitter`: record `NodeId → ElmNodeMeta` mappings during emission
- [x] 7.5 Add `compile_to_elm_with_sourcemap()` API to `compiler.rs` returning `(elm::Library, SourceMap)`
- [x] 7.6 Implement JSON serialization/deserialization for `SourceMap` (sidecar `.elm.sourcemap.json`)
- [x] 7.7 Write tests verifying source-map round-trips through JSON
- [x] 7.8 Write tests verifying CQL span → ELM node correlation for sample expressions

## 8. Explain Infrastructure

- [x] 8.1 Create `src/explain/mod.rs` with submodule declarations
- [x] 8.2 Implement `explain_parse()`: walk parser AST, produce indented tree text with node types, source locations, literal values
- [x] 8.3 Implement `explain_compile()`: walk `TypedLibrary`, produce narrative describing resolved types, overload selections, implicit conversions per expression node
- [x] 8.4 Implement `explain_eval()` stub returning "Evaluation engine not yet available" error
- [x] 8.5 Add `explain_parse()` and `explain_compile()` to `compiler.rs` public API
- [x] 8.6 Write tests verifying explain output for sample CQL (simple expressions, overload resolution, implicit conversions)

## 9. Eval Engine

### 9.A Foundation

- [x] 9.1 Create `src/eval/mod.rs` with submodule declarations
- [x] 9.2 Define `Value` enum with all CQL runtime value types (Null, Boolean, Integer, Long, Decimal, String, Date, DateTime, Time, Quantity, Ratio, Code, Concept, List, Tuple, Interval)
- [x] 9.3 Implement `Value` display, equality, and equivalence per CQL spec
- [x] 9.4 Implement three-valued logic (And, Or, Not, Implies, Xor with null propagation)

### 9.B Context & Providers

- [x] 9.5 Define `EvalContext` struct with deterministic clock, timezone, parameter values, context value, provider references
- [x] 9.6 Define `DataProvider` trait with `retrieve()` method
- [x] 9.7 Define `TerminologyProvider` trait with `in_valueset()`, `expand_valueset()`, `lookup()` methods
- [x] 9.8 Implement `InMemoryDataProvider` for testing
- [x] 9.9 Implement `InMemoryTerminologyProvider` for testing

### 9.C Arithmetic & Comparison Operators

- [x] 9.10 Create `src/eval/operators.rs`: implement arithmetic operators (Add, Subtract, Multiply, Divide, Modulo, Negate, Abs, Ceiling, Floor, Truncate, Round, Power, Ln, Exp, Log, Predecessor, Successor, MinValue, MaxValue)
- [x] 9.11 Create `src/eval/operators.rs`: implement comparison operators (Equal, Equivalent, Less, Greater, LessOrEqual, GreaterOrEqual) with null handling

### 9.D String & Date/Time Operators

- [x] 9.12 Create `src/eval/operators.rs`: implement string operators (Concatenate, Combine, Split, Length, Upper, Lower, StartsWith, EndsWith, Matches, Indexer, PositionOf, Substring)
- [x] 9.13 Create `src/eval/operators.rs`: implement date/time operators (construction, component extraction, date arithmetic, SameAs, SameOrBefore, SameOrAfter, duration calculations)

### 9.E Type Conversion Operators

- [x] 9.14 Create `src/eval/operators.rs`: implement type conversion operators (ToBoolean, ToInteger, ToDecimal, ToLong, ToString, ToDate, ToDateTime, ToTime, ToQuantity, ToConcept, Is, As, Convert)

### 9.F Interval & List Operators

- [x] 9.15 Create `src/eval/intervals.rs`: implement interval operators (Contains, In, Includes, IncludedIn, Overlaps, Meets, Starts, Ends, Union, Intersect, Except, Width, Start, End, PointFrom, Expand, Collapse)
- [x] 9.16 Create `src/eval/lists.rs`: implement list operators (Exists, Count, Sum, Min, Max, Avg, First, Last, IndexOf, Contains, In, Flatten, Distinct, Sort, Filter, ForEach, SingletonFrom, Union, Intersect, Except)

### 9.G Query Evaluation

- [x] 9.17 Create `src/eval/queries.rs`: implement single-source query evaluation (from, where, return, sort, let)

### 9.H Evaluation Entry Points & Trace

- [x] 9.18 Implement `evaluate_elm()` entry point: accept `elm::Library`, expression name, `EvalContext`, return `Result<Value>`
- [x] 9.19 Implement `evaluate_elm_with_trace()`: return `(Value, Vec<TraceEvent>)` with step-by-step trace events
- [x] 9.20 Define `TraceEvent` struct with `event_id`, `elm_node_id`, `op`, `inputs`, `output`, `children`
- [x] 9.21 Wire `explain_eval()` to use `evaluate_elm_with_trace()` once the eval engine is functional

### 9.I Tests

- [x] 9.22 Write unit tests for each operator family (arithmetic, comparison, string, date/time, type conversion, interval, list)
- [x] 9.23 Write integration tests: evaluate sample CQL expressions end-to-end (compile → eval)

## 10. CLI Updates

- [x] 10.1 Add `explain` subcommand to `apps/rh-cli/src/cql.rs` with `parse` and `compile` modes
- [x] 10.2 Add `eval` subcommand to `apps/rh-cli/src/cql.rs` with `--expression`, `--trace` options
- [x] 10.3 Add source-map output option to `compile` subcommand (`--source-map` flag)
- [x] 10.4 Write CLI integration tests for `explain parse`, `explain compile`, and `eval` subcommands

## 11. Documentation and Cleanup

- [x] 11.1 Update `crates/rh-cql/ARCHITECTURE.md` to document the three-stage pipeline, Typed AST, and module structure
- [x] 11.2 Update `openspec/planning/rh-cql/PLAN.md` to mark Phases A-C as restructured, Phase D (explain) as implemented, Phase E (eval) as implemented
- [x] 11.3 Update `crates/rh-cql/README.md` with new API surface (`compile_to_elm_with_sourcemap`, `explain_parse`, `explain_compile`, `evaluate_elm`)
- [x] 11.4 Run `cargo test -p rh-cql` and verify all tests pass
- [x] 11.5 Run `cargo clippy -p rh-cql` and resolve any warnings
- [x] 11.6 Verify all examples in `examples/` compile and produce expected output
