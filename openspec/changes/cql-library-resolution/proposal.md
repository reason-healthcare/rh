## Why

CQL libraries routinely `include` other libraries (FHIRHelpers, shared logic libraries, IG-specific helpers), but `rh cql eval` and `rh cql validate` currently ignore those includes entirely — any cross-library expression or function reference fails at evaluation time even though all the scaffold infrastructure (`LibraryManager`, `FileLibrarySourceProvider`, `LibraryIdentifier`, etc.) already exists in `rh-cql`. The gap is that the compile and evaluate pipelines have no way to accept a library source provider, so included libraries are never loaded or resolved.

## What Changes

- **New**: `compile_with_libraries(source, options, provider)` public API — compiles a CQL library and recursively resolves its `include` declarations using a `LibrarySourceProvider`
- **New**: `evaluate_elm_with_libraries(main_lib, libs_map, name, ctx)` — evaluates a named expression with cross-library `ExpressionRef` and `FunctionRef` resolution
- **New**: Library-aware semantic analysis — included library definitions (expressions, functions, value sets, code systems, codes, concepts) are registered into the scope manager so qualified references like `CaseLogic."My Expression"` resolve correctly
- **New**: `EvalContextBuilder::with_libraries(map)` — attach a compiled library map to the evaluation context
- **Modified**: `run_compile_pipeline` extended with an optional library source provider parameter
- **Modified**: `rh cql eval` CLI — auto-discovers CQL files in the current directory (and subdirectories) and builds a `FileLibrarySourceProvider`; reports clear errors when a required library cannot be found
- **Modified**: `rh cql validate` CLI — same auto-discovery for dependency resolution during validation
- **Modified**: ELM emitter — populates the `includes` field (currently hardcoded `None`) from `TypedLibrary.includes`

## Capabilities

### New Capabilities

- `cql-library-resolution`: Library discovery, loading, recursive compilation, and cross-library symbol resolution for the CQL compiler and evaluator

### Modified Capabilities

- `cql-eval-engine`: Engine gains a library map allowing cross-library `ExpressionRef` and `FunctionRef` evaluation
- `cql-semantic-analysis`: Semantic analyzer gains access to resolved included-library scope (expressions, functions, value sets, etc.) for qualified reference resolution

## Impact

- **`crates/rh-cql`**: New `compile_with_libraries` API, modified pipeline internals, extended `SemanticAnalyzer`, extended `Engine`/`EvalContext`, emitter `includes` field fix
- **`apps/rh-cli`**: `rh cql eval` and `rh cql validate` gain `--lib-path` flag (default: current directory) and recursive CQL file discovery
- **No breaking changes** to existing `compile()`, `validate()`, `evaluate_elm()` signatures — the new APIs are additive
- **Dependencies**: No new crate dependencies required; `FileLibrarySourceProvider` is already implemented
