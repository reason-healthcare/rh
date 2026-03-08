## Context

The rh-cql crate currently compiles CQL to ELM using a single-pass architecture where `LibraryBuilder` (3700 lines) orchestrates semantic analysis interleaved with ELM generation via `ExpressionTranslator` (9500 lines). This was a pragmatic choice — ARCHITECTURE.md documents it as chosen for simplicity and performance. However, the architecture has produced a duplicated dispatch pattern: `ExpressionTranslator::translate_expression` handles structural AST→ELM mapping while `LibraryBuilder::translate_expr_recursive` re-implements the same pattern with symbol resolution and type inference interleaved. This coupling blocks three planned capabilities (explain mode, source-maps, eval instrumentation) that require capturing semantic decisions separately from ELM generation.

The parser (nom-based, 75.6% jvmTest conformance) and ELM model (~110 expression variants, spec-faithful serde) are solid and cleanly separated. The type system (`DataType`, `TypeResolver`, `OperatorResolver`, `ConversionRegistry`) is well-designed. These components are preserved unchanged.

Current public API surface: `compile()`, `compile_with_model()`, `compile_to_json()`, `validate()`. These are preserved and extended with new functions.

## Goals / Non-Goals

**Goals:**

- Introduce a Typed AST intermediate representation that carries resolved types, source locations, and semantic decision metadata between parsing and ELM emission
- Create a dedicated semantic analysis pass that produces the Typed AST, eliminating the duplicated dispatch in builder.rs and translator.rs
- Rewrite the ELM emitter to consume Typed AST — pure structural mapping with no semantic logic, split into focused submodules
- Enable source-map generation (CQL span ↔ ELM node correlation) via Typed AST node IDs
- Enable explain mode (semantic decision narrative) via Typed AST annotation metadata
- Build an ELM evaluation engine with runtime value model, three-valued logic, and provider traits
- Close parser conformance gaps to ≥90% jvmTest pass rate
- Fix known ELM conformance issues (system function emission, negative literals, integer division promotion)
- Preserve all existing public API functions; add new ones additively
- Keep the migration incremental — tests pass after each stage

**Non-Goals:**

- Rewriting the parser — it's clean and reusable as-is
- Rewriting the ELM model types — they're spec-faithful and well-structured
- Rewriting `datatype.rs`, `types.rs`, `operators.rs`, `conversion.rs` — these are solid, reused by the new semantic analyzer
- Supporting CQL versions other than 1.5 (current scope)
- LSP integration (future work building on this foundation)
- Incremental/partial compilation (future work)
- Multi-threaded compilation pipeline (not needed at current scale)
- XML ELM output (JSON-first per PLAN.md)

## Decisions

### 1. Typed AST as a distinct IR (not annotated parser AST)

**Decision**: Create a new `TypedExpression` enum in `semantics/typed_ast.rs` that mirrors the parser AST structure but wraps every node in a `TypedNode<T>` carrying `DataType`, `SourceSpan` (start + end), `NodeId`, and `SemanticMeta`.

**Alternatives considered**:
- *Annotate parser AST in-place with `Option<DataType>` fields*: Rejected — pollutes the clean parser AST with optional fields, makes it unclear which nodes have been through semantic analysis, violates single-responsibility.
- *Use ELM nodes directly as IR*: Rejected — ELM is a serialization format designed for interchange, not an internal compiler IR. It lacks semantic decision metadata and its structure doesn't map 1:1 to CQL constructs.

**Rationale**: A separate Typed AST keeps the parser AST immutable and simple, gives the semantic analyzer full ownership of its output, and provides a natural place to attach explain-mode metadata without cluttering ELM types. The `NodeId` on every typed node enables source-map correlation without propagating IDs through the ELM emitter's logic.

### 2. Three-stage pipeline: Parse → Analyze → Emit

**Decision**: Replace the current single-pass (parse → interleaved analyze+emit) with three explicit stages:
1. **Parse**: `CqlParser::parse()` → `ast::Library` (unchanged)
2. **Analyze**: `SemanticAnalyzer::analyze()` → `TypedLibrary` (new)
3. **Emit**: `ElmEmitter::emit()` → `elm::Library` (rewritten, simplified)

**Alternatives considered**:
- *Keep single-pass, instrument with callbacks*: Rejected — would require threading a context/observer through every translation function, effectively rebuilding a second pass inside the first.
- *Two-pass (parse → combined analyze+emit with instrumentation)*: Rejected — doesn't solve the core problem of tangled concerns, just adds instrumentation overhead to the existing mess.

**Rationale**: Clean separation enables each stage to be tested, optimized, and extended independently. The ~15% compile overhead from an intermediate allocation is acceptable for a toolkit (not a hot-path JIT). This matches PLAN.md's original design intent.

### 3. SemanticAnalyzer absorbs both builder and translator resolution logic

**Decision**: The new `SemanticAnalyzer` struct replaces both `LibraryBuilder`'s semantic logic and `ExpressionTranslator`'s structural traversal with a single walk of the parser AST. It holds: scope stack, model provider reference, type resolver, operator resolver, conversion registry, compiler options, and diagnostics collector.

**Alternatives considered**:
- *Keep LibraryBuilder for scope management, have SemanticAnalyzer delegate to it*: Rejected — LibraryBuilder's API is too coupled to ELM generation (methods return `elm::Expression`). Cleaner to extract scope management into a focused `ScopeManager` used by the analyzer.

**Rationale**: Eliminates the duplicated dispatch pattern entirely. One exhaustive match over AST Expression variants, producing Typed AST nodes with all semantic decisions resolved.

### 4. Scope management extracted into `semantics/scope.rs`

**Decision**: Extract `Symbol`, `Scope`, `SymbolKind`, `ScopeManager` (push/pop scope, resolve symbol, register definition) from builder.rs into a standalone module. `ScopeManager` is a stack of `Scope` maps with standard nesting/shadowing rules.

**Rationale**: Scope management is a well-defined, independently testable concept. Both the semantic analyzer and future LSP integration need it without dragging in compilation logic.

### 5. ELM emitter as pure structural mapping in submodules

**Decision**: The new emitter in `src/emit/` consumes `TypedLibrary` and produces `elm::Library`. It has no access to symbol tables, type resolvers, or conversion registries — every decision has already been made and recorded in Typed AST nodes. Split into submodules by expression family:
- `mod.rs` — `ElmEmitter` struct, library-level emission, entry point
- `literals.rs` — literal and value expressions
- `operators.rs` — unary, binary, ternary, nary operators (including system functions as native ELM types)
- `queries.rs` — query, retrieve, sort, let
- `clinical.rs` — code, concept, valueset, retrieve with terminology
- `types.rs` — type expressions (is, as, cast, convert), interval, list, tuple
- `conditionals.rs` — if/then/else, case

**Rationale**: Each submodule is small (<500 lines), focused, and independently testable. The emitter's simplicity (no semantic logic) makes it easy to audit for ELM conformance correctness.

### 6. Source-map schema follows PLAN.md §6.1

**Decision**: Implement the source-map schema from PLAN.md exactly:
- `SourceMap` with `SourceDocument[]`, `ElmNodeMeta[]`, `SourceElmMapping[]`
- `ElmNodeMeta`: `elm_node_id`, `elm_path`, `elm_kind`, `parent_id`
- `SourceElmMapping`: `doc_id`, `span`, `role` (direct/implicit-conversion/desugared/synthetic), `elm_node_ids[]`
- Stable hash-based IDs
- Sidecar JSON output (`*.elm.sourcemap.json`)

The ELM emitter maps each Typed AST `NodeId` to the ELM element it produces, building the source-map as a side-channel during emission.

**Rationale**: The schema is well-designed and already documented. No reason to deviate.

### 7. Eval engine operates on deserialized ELM (not Typed AST)

**Decision**: The evaluation engine takes `elm::Library` as input, not `TypedLibrary`. It evaluates ELM expressions via pattern matching on the `Expression` enum.

**Alternatives considered**:
- *Evaluate Typed AST directly*: Rejected — only works with rh-cql's own output. An ELM-based evaluator can process any conformant ELM (from Java translator, other tools, or hand-written).
- *Compile Typed AST to a bytecode IR for eval*: Rejected — overengineered for current needs. Direct ELM interpretation is sufficient.

**Rationale**: ELM is the canonical interchange format per the CQL spec. An ELM-first evaluator maximizes interoperability. The Typed AST's `resultTypeName` population ensures ELM nodes carry type information needed for type-directed dispatch in the evaluator. Source-map lookup provides CQL span correlation for trace events when available.

### 8. Eval engine provider traits for data and terminology

**Decision**: Define `DataProvider` and `TerminologyProvider` traits in `src/eval/`:

```rust
trait DataProvider {
    fn retrieve(&self, context: &str, data_type: &str, 
                code_path: Option<&str>, codes: Option<&[Code]>,
                date_path: Option<&str>, date_range: Option<&Interval>)
        -> Result<Vec<Value>>;
}

trait TerminologyProvider {
    fn in_valueset(&self, code: &Code, valueset: &str) -> Result<bool>;
    fn expand_valueset(&self, valueset: &str) -> Result<Vec<Code>>;
    fn lookup(&self, code: &Code, property: &str) -> Result<Option<Value>>;
}
```

**Rationale**: Separates the eval engine from any specific FHIR server or terminology service. In-memory providers for testing, HTTP providers for production.

### 9. Incremental migration strategy

**Decision**: Migrate in this order to keep tests green throughout:
1. Define Typed AST types (additive, no breaking changes)
2. Extract scope management (move, not rewrite)
3. Build semantic analyzer alongside existing builder (new code, doesn't replace yet)
4. Build new ELM emitter alongside existing translator (new code)
5. Wire new pipeline into `compile()` behind a feature flag or as an alternate path
6. Validate conformance parity with existing pipeline
7. Remove old builder translation logic and old translator
8. Add source-maps, explain, eval as purely additive features

**Rationale**: At no point is the crate in a broken state. Steps 3-4 run in parallel with the existing pipeline until proven equivalent.

### 10. Fix ELM conformance during emitter rewrite

**Decision**: Address all known ELM output differences from the Java reference translator as part of the new emitter, not as patches to the old one:
- System functions (`Abs`, `Ceiling`, `Floor`, `Truncate`, `Round`, `Ln`, `Exp`, `Log`, `Power`) → emit as native ELM expression types
- Negative literals → `Negate(Literal(n))` instead of `Literal(-n)`
- Integer division → promote operands to Decimal via `ToDecimal` conversion
- `predecessor of` / `successor of` → `Predecessor` / `Successor` ELM types (also requires parser fix)

**Rationale**: The new emitter is being written from scratch against the Typed AST. It's cleaner to get it right from the start than to patch the old translator and then port the patches.

## Risks / Trade-offs

**[Typed AST maintenance burden]** → The Typed AST mirrors the parser AST, creating a parallel type hierarchy. When new CQL syntax is added to the parser, corresponding Typed AST variants must be added.
*Mitigation*: The Typed AST uses a generic `TypedNode<T>` wrapper, so new expression-level variants only need one new enum variant (not a full new struct). Derive macros reduce boilerplate.

**[Migration period complexity]** → During steps 3-6, both old and new pipelines coexist, doubling the compilation code temporarily.
*Mitigation*: The old pipeline remains the default. New pipeline runs as a parallel validation path. Once conformance parity is proven, old code is deleted in a single step.

**[~15% compile performance overhead]** → The intermediate Typed AST allocation adds overhead vs. single-pass.
*Mitigation*: CQL compilation is not a hot path (libraries compiled once, evaluated many times). Benchmark before/after to quantify actual impact. Arena allocation can reduce this if needed.

**[Eval engine scope creep]** → Full CQL evaluation is a large surface area (300+ operators, query semantics, timezone handling).
*Mitigation*: Start with core operators (arithmetic, comparison, logical, string, date/time). Query evaluation single-source only. Expand based on conformance test results. The eval engine is additive and doesn't block other capabilities.

**[Parser conformance ≥90% may require grammar refactoring]** → Some missing productions (`aggregate`, complex timing) may require non-trivial parser changes.
*Mitigation*: Parser changes are independent of the pipeline refactor. They can proceed in parallel or after the pipeline migration.

**[Existing re-exports change]** → `lib.rs` currently re-exports `LibraryBuilder`, `ExpressionTranslator`, `Symbol`, `Scope`, `SymbolKind`. These types will change or move.
*Mitigation*: Re-export new types under old names where possible during transition. Document the migration in a changelog entry. The primary public API (`compile`, `validate`, `compile_to_json`) is preserved unchanged.
