# rh-cql Refactor Tasks

This document captures the follow-up refactor plan for the current `rh-cql` implementation.

## 1. Compilation Pipeline Consolidation

- [x] 1.1 Extract a shared internal compilation pipeline in `src/compiler.rs` that performs parse, semantic analysis, ELM emission, diagnostics collection, and optional source-map generation
- [x] 1.2 Reimplement `compile()`, `compile_with_model()`, `validate()`, `compile_to_json()`, `compile_to_elm_with_sourcemap()`, and `explain_compile()` on top of that shared pipeline
- [x] 1.3 Remove duplicated compile work in the CLI `compile` command when `--source-map` is enabled
- [x] 1.4 Add regression tests verifying compile, validate, explain, and source-map generation produce consistent diagnostics for the same input

## 2. CLI Structure Cleanup

- [x] 2.1 Extract reusable CQL command services from `apps/rh-cli/src/cql.rs` for compile, validate, eval, explain, and info flows
- [x] 2.2 Centralize compile result and diagnostic formatting instead of repeating similar output logic in each command handler
- [x] 2.3 Remove `std::process::exit` from helper functions and return status to the top-level CLI entrypoint
- [x] 2.4 Add focused CLI tests for command outcomes and exit behavior after the refactor
- [x] 2.5 Update the CLI README

## 3. Parser Extraction

- [x] 3.1 Add a shared helper for left-associative precedence parsing in `src/parser/expression.rs`
- [x] 3.2 Extract shared query-tail parsing for `let`, relationship, `where`, `return`, `aggregate`, and `sort` clauses
- [x] 3.3 Split `src/parser/expression.rs` into smaller submodules such as precedence, query, literals, selectors, and retrieve parsing
- [x] 3.4 Preserve or expand parser coverage to verify no grammar regressions after the split

## 4. Evaluation Engine and Operator Modularization

- [x] 4.1 Refactor `src/eval/engine.rs` so it primarily handles dispatch, bindings, and trace recording
- [x] 4.2 Extract expression-family helpers for logical, control-flow, list, interval, query, and terminology evaluation paths
- [x] 4.3 Split `src/eval/operators.rs` into domain modules such as arithmetic, comparison, string, temporal, and conversion
- [x] 4.4 Keep shared null-propagation and error helpers in a small common evaluation utility layer
- [x] 4.5 Preserve current operator and evaluation behavior with unit and integration tests during the split

## 5. Definition Lookup and Caching

- [x] 5.1 Add internal indexes to `CompiledLibrary` for expressions, parameters, includes, and functions
- [x] 5.2 Refactor evaluation lookup to use indexed access instead of repeated statement scans and expression cloning
- [x] 5.3 Implement function lookup APIs fully, or remove placeholder public methods until function indexing is ready
- [x] 5.4 Add benchmarks or targeted tests covering repeated lookup and evaluation scenarios

## 6. Library Module Decomposition

- [ ] 6.1 Split `src/library.rs` into smaller modules for identifiers, sources, providers, compiled-library access, and library management
- [ ] 6.2 Keep public exports in `src/lib.rs` stable while reorganizing internal files
- [ ] 6.3 Move provider implementations into dedicated submodules and keep filesystem, memory, and composite behaviors clearly separated
- [ ] 6.4 Add tests around provider behavior and compiled-library lookup before and after the split

## 7. Warning and Dead-Code Cleanup

- [ ] 7.1 Remove currently reported warnings such as redundant semicolons, unused imports, and dead helper methods
- [ ] 7.2 Delete or wire up dead evaluation trace helpers in `src/eval/engine.rs`
- [ ] 7.3 Run `cargo test -p rh-cql --quiet` and `cargo clippy -p rh-cql --all-targets --all-features -- -D warnings`
- [ ] 7.4 Keep the crate warning-free before starting larger structural refactors

## 8. Documentation Refresh

- [ ] 8.1 Update `crates/rh-cql/ARCHITECTURE.md` to match the actual `nom` parser and the current semantic-analysis and emit pipeline
- [ ] 8.2 Document the unified compilation pipeline once it exists
- [ ] 8.3 Clarify the status of deprecated translator and builder APIs in crate-level documentation
- [ ] 8.4 Update any README or plan references that still describe superseded architecture

## 9. Unified Diagnostic System

- [ ] 9.1 Define a consistent `Diagnostic` type used across Parse, Semantic, Emit, and Eval stages
- [ ] 9.2 Include `SourceSpan`, `Severity`, and unique error codes in the unified diagnostic structure
- [ ] 9.3 Implement diagnostic aggregation in the consolidated pipeline to ensure consistent reporting

## 10. Performance and Benchmarking

- [ ] 10.1 Establish baseline benchmarks for compilation and evaluation before implementing indexing changes
- [ ] 10.2 Add performance regression tests to CI to track lookup and evaluation speed improvements
- [ ] 10.3 Verify that the refactor provides measurable improvements for large library lookup

## 11. "Golden File" Verification

- [ ] 11.1 Implement or expand "golden" integration tests comparing ELM output against known-good baselines
- [ ] 11.2 Use golden files to verify that the multi-stage pipeline produces bit-identical or semantically equivalent output during refactoring
- [ ] 11.3 Add verification tests for evaluation results against stable result sets

## 12. Internal Context Consolidation

- [ ] 12.1 Introduce a `CompilationContext` to streamline function signatures in the compiler and semantics modules
- [ ] 12.2 Consolidate options, providers, and registries into a shared context object passed through the pipeline
- [ ] 12.3 Reduce parameter drilling in the evaluation engine using a consolidated `EvalContext`

## 13. XML Parsing Modernization

- [ ] 13.1 Refactor `src/modelinfo_xml.rs` to use a more modular approach for parsing `TypeInfo` variants
- [ ] 13.2 Split the large 1300+ line parser into smaller submodules or explore a more declarative parsing strategy

## Recommended Order

- [ ] A. Complete warning cleanup first so structural regressions are easier to spot
- [ ] B. Consolidate the compilation pipeline before parser or evaluation refactors
- [ ] C. Establish baseline benchmarks and golden files to protect against regressions
- [ ] D. Refactor CLI code after the pipeline is unified
- [ ] E. Add definition indexes before deeper evaluation cleanup
- [ ] F. Split parser, evaluation, and library modules after behavior is covered by tests
- [ ] G. Refresh documentation after the code structure stabilizes
