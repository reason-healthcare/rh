## 1. Semantic Analyzer — Include Alias Resolution

- [x] 1.1 In `SemanticAnalyzer::analyze_member_invocation`, after resolving the source expression: if source is `IdentifierRef` and it resolves to `SymbolKind::Include`, reclassify the node as `QualifiedIdentifierRef { qualifier, name }` instead of `MemberInvocation`
- [x] 1.2 Add a test in `semantics/tests` (or the existing `analyzer.rs` test module) verifying that `CaseLogic."My Expr"` where `CaseLogic` is a library alias produces a `QualifiedIdentifierRef` node
- [x] 1.3 Add a test verifying that `Patient.name` where `Patient` is NOT a library alias remains a `MemberInvocation` (regression guard)

## 2. ELM Emitter — Populate Includes Field

- [x] 2.1 In `emit/mod.rs`, replace the hardcoded `includes: None` with a loop over `TypedLibrary.includes` that builds `elm::IncludeDef { path, version, local_identifier }` entries
- [x] 2.2 Add a test verifying that a library with one `include X called Y` emits an `IncludeDef` with `path: "X"` and `local_identifier: "Y"`
- [x] 2.3 Add a test verifying that a library with no includes emits an empty `includes` list

## 3. Compile Pipeline — Library Source Provider Integration

- [x] 3.1 Add an `Option<&dyn LibrarySourceProvider>` parameter to `run_compile_pipeline` (internal); when `None`, skip library resolution (existing behavior)
- [x] 3.2 After the parse step in `run_compile_pipeline`, when a provider is supplied: extract include declarations from the parsed AST header, invoke `LibraryManager` to recursively compile each dependency, and collect the resulting `HashMap<alias, CompiledLibrary>`
- [x] 3.3 Pass the resolved library map into `SemanticAnalyzer::new` / `analyze` so that qualified expression lookups can resolve against included library scopes
- [x] 3.4 Add `compile_with_libraries(source, options, provider: &dyn LibrarySourceProvider) -> Result<CompileOutputWithLibs, CompileErrors>` as a public API in `compiler.rs`, where `CompileOutputWithLibs` bundles the main `CompileOutput` with the dependency map
- [x] 3.5 Ensure existing `compile()` and `validate()` continue to compile with no library source provider (no behavioral change)

## 4. Evaluator — Cross-Library Reference Resolution

- [x] 4.1 Add `evaluate_elm_with_libraries(lib, included: &HashMap<String, &elm::Library>, name, ctx) -> Result<Value>` to `eval/engine.rs`
- [x] 4.2 In `Engine::evaluate_expression_ref`, when `library_name` is `Some(alias)`, look up `alias` in the included library map and delegate to an evaluation of that library's named expression
- [x] 4.3 In `Engine::evaluate_function_ref`, when `library_name` is `Some(alias)`, look up the function in the included library and evaluate it with the supplied arguments
- [x] 4.4 When `alias` is not found in the included map, return an `EvalError::LibraryNotFound { alias }` with a clear message
- [x] 4.5 Add a unit test: compile two CQL strings (main + included), call `evaluate_elm_with_libraries`, verify cross-library expression resolves to the expected value
- [x] 4.6 Add a unit test verifying that a missing alias in the included map surfaces as `EvalError::LibraryNotFound`

## 5. CLI — Auto-Discovery of Library Files

- [x] 5.1 In `apps/rh-cli/src/cql.rs`, extract the directory of the input CQL file and construct a `FileLibrarySourceProvider` for that directory
- [x] 5.2 Add `--lib-path <dir>` (repeatable) flag to both `rh cql eval` and `rh cql validate` subcommands; build a `CompositeLibrarySourceProvider` from the default directory plus any `--lib-path` values
- [x] 5.3 Wire the provider into `compile_with_libraries` call from the CLI (replacing the existing `compile()` call)
- [x] 5.4 Wire the resulting `included` map into `evaluate_elm_with_libraries` call from the CLI
- [x] 5.5 When a required include is not found, emit a CLI error message listing the library name and all searched paths before exiting

## 6. Integration Tests

- [x] 6.1 Add an integration test in `apps/rh-cli/tests/` or `crates/rh-cql/tests/` that compiles and evaluates a two-file CQL example (main library + one include) end-to-end and asserts the correct result
- [x] 6.2 Verify `cargo test -p rh-cql` and `cargo test -p rh-cli` pass with no regressions
