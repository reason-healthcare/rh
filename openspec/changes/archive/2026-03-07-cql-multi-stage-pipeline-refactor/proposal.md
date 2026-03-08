## Why

The rh-cql crate's compiler uses a single-pass architecture that tangles semantic analysis with ELM generation. This makes it impossible to add explain mode (semantic decision tracing), source-maps (CQL span ↔ ELM node correlation), or eval instrumentation without invasive changes threading context through every translation function. The core technical debt is a duplicated dispatch pattern — `ExpressionTranslator::translate_expression` and `LibraryBuilder::translate_expr_recursive` do the same structural traversal with interleaved concerns — resulting in a 9500-line translator monolith and a 3700-line builder doing too many things. Restructuring into a multi-stage pipeline (Parse → Semantic Analysis → ELM Emit) with a Typed AST intermediate representation unlocks all planned capabilities while eliminating the duplicated dispatch and improving maintainability.

## What Changes

- **New**: Typed AST intermediate representation annotated with resolved types, source locations, semantic decisions (overload resolution, implicit conversions, list promotion)
- **New**: Semantic analysis pass producing Typed AST from parser AST, absorbing symbol resolution, type inference, operator overload resolution, and implicit conversion logic
- **New**: Source-map infrastructure mapping CQL spans to ELM nodes via Typed AST node IDs
- **New**: Explain mode with parse, compile, and eval explanation output
- **New**: ELM evaluation engine with runtime value model, three-valued logic, and data/terminology provider traits
- **Refactor**: ELM emitter rewritten to consume Typed AST instead of raw parser AST — pure structural mapping, no semantic logic — split into focused submodules
- **Refactor**: `builder.rs` reduced from 3700-line god-object to thin orchestrator (<500 lines)
- **Refactor**: Scope/symbol management extracted into dedicated `semantics/scope.rs` module
- **Fix**: System functions (`Abs`, `Ceiling`, `Floor`, etc.) emit as native ELM types instead of `FunctionRef`
- **Fix**: Negative literals use `Negate(Literal(n))` form per spec
- **Fix**: Integer division promotes operands to `Decimal`
- **Fix**: Parser conformance gaps — `predecessor of`/`successor of`, `^` power operator, `minimum`/`maximum Type`, `aggregate` clause, `let` clause in queries, timing phrase improvements
- **New**: CLI `explain` and `eval` subcommands
- **BREAKING**: Internal API restructuring — `LibraryBuilder` replaced by `SemanticAnalyzer` + orchestrator; `ExpressionTranslator` replaced by Typed AST-consuming emitter. Public `compile()` and `validate()` APIs preserved.

## Capabilities

### New Capabilities

- `cql-typed-ast`: Typed AST intermediate representation with resolved types, source locations, stable node IDs, and semantic decision metadata — the new IR between parsing and ELM generation
- `cql-semantic-analysis`: Dedicated semantic analysis pass that walks parser AST and produces Typed AST, performing symbol resolution, type inference, operator overload resolution, implicit conversion application, and FHIR type resolution
- `cql-source-maps`: CQL span ↔ ELM node correlation infrastructure enabling debugger/tracing support, implementing the PLAN.md §6.1 schema
- `cql-explain`: Three-mode explanation output — `explain_parse()` (AST pretty-printer), `explain_compile()` (semantic decision narrative), `explain_eval()` (step trace, depends on eval engine)
- `cql-eval-engine`: ELM evaluation engine with runtime value model, three-valued logic, deterministic clock injection, data/terminology provider traits, query execution, and trace-instrumented evaluation
- `cql-elm-emitter`: Refactored ELM emitter consuming Typed AST, split into focused submodules, with conformance fixes for system function emission, negative literals, and integer division promotion
- `cql-parser-conformance`: Parser production additions for missing CQL syntax — targeting ≥90% jvmTest conformance

### Modified Capabilities

_(No existing specs are CQL-related — all capabilities are new)_

## Impact

- **Crate**: `rh-cql` — major internal restructuring; public API preserved with additions
- **Files rewritten**: `translator.rs` (~9500 lines → split submodules), `builder.rs` (~3700 lines → <500 line orchestrator + new `semantics/` modules)
- **Files preserved**: `parser/` (AST, lexer, expression, statement, span), `elm/` (ELM model), `datatype.rs`, `types.rs`, `operators.rs`, `conversion.rs`, `library.rs`, `preprocessor.rs`, `provider.rs`, `modelinfo.rs`, `options.rs`, `output.rs` — approximately 65-70% of existing code
- **New modules**: `semantics/` (typed_ast, scope, analyzer), `emit/` (submodules by expression family), `sourcemap.rs`, `explain/`, `eval/`
- **CLI**: `apps/rh-cli/src/cql.rs` gains `explain` and `eval` subcommands
- **Dependencies**: Potential new dependencies for eval engine (e.g., `chrono` for date/time, `rust_decimal` if not already used)
- **Tests**: Existing conformance infrastructure reused; new tests for semantic analysis, source-maps, explain output, and eval engine
- **Documentation**: `PLAN.md` and `ARCHITECTURE.md` updated to reflect multi-stage pipeline adoption
