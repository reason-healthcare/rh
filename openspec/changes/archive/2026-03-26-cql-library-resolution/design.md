## Context

`rh-cql` ships with a complete library management subsystem (`crates/rh-cql/src/library/`):
- `LibrarySourceProvider` trait + `FileLibrarySourceProvider` (searches paths for `Name.cql` / `Name-version.cql`), `MemoryLibrarySourceProvider`, `CompositeLibrarySourceProvider`
- `LibraryManager` — caching, DFS cycle detection, topological sort of dependencies
- `CompiledLibrary` — O(1)-indexed wrapper around `elm::Library`
- `LibraryIdentifier` — `(name, version)`, flexible `matches()` for version-optional lookup

None of this is wired into the `compile()` or `evaluate_elm()` pipeline. As a result, any CQL library that uses `include` statements (FHIRHelpers, shared value set libraries, IG-specific logic modules) cannot be compiled or evaluated. The `rh cql eval` command fails with `Expression 'CaseLogic' not found in library` because the include alias is registered as a symbol but never resolved to a real library, so the emitter treats it as an expression ref, and the engine cannot find it.

## Goals / Non-Goals

**Goals:**
- Wire `LibraryManager` + `FileLibrarySourceProvider` into `run_compile_pipeline` so included libraries are compiled recursively before the main library
- Expose additive APIs: `compile_with_libraries(source, options, provider)` and `evaluate_elm_with_libraries(lib, libs_map, name, ctx)`
- Fix semantic analysis so that `MemberInvocation` nodes where the source is an `IdentifierRef` resolving to `SymbolKind::Include` are reclassified as `QualifiedIdentifierRef` — the canonical node the emitter and engine already handle
- Populate the ELM `includes` field in the emitter from `TypedLibrary.includes`
- Auto-discover sibling `.cql` files in `rh cql eval` and `rh cql validate` via an optional `--lib-path` flag (defaulting to the CWD of the input file)
- No breaking changes to existing `compile()`, `validate()`, `evaluate_elm()` callsites

**Non-Goals:**
- Network-based library fetching (e.g., from a FHIR package registry)
- Semver-range resolution (version matching remains exact or version-optional)
- Multi-source query (join) evaluation — already deferred in `cql-eval-engine`
- Parser-level disambiguation of property access vs. qualified identifier (no scope at parse time)

## Decisions

### D1: Semantic layer fixes `MemberInvocation` → `QualifiedIdentifierRef`, not the parser

The parser cannot distinguish `Patient.name` (property access) from `CaseLogic."My Expr"` (library-qualified reference) because both are syntactically `IdentifierRef.member`. The distinction requires knowing whether `CaseLogic` is a `SymbolKind::Include`. That information is only available in the semantic analyzer's scope. Therefore:

- The semantic analyzer, when analyzing `MemberInvocation` where the source is an `IdentifierRef`, first checks if the identifier resolves to `SymbolKind::Include`. If yes, it reclassifies the node as `QualifiedIdentifierRef`.
- The ELM emitter already handles `TypedExpression::QualifiedIdentifierRef` → `ExpressionRef { library_name: Some(...) }`. No emitter changes are needed for this path.
- Alternative considered: fix in the parser via a two-pass approach. Rejected — the parser is intentionally scope-free and adding a scope phase there duplicates the semantic layer.

### D2: Additive API surface — do not change existing `compile()` / `evaluate_elm()` signatures

Changing the signatures of `compile()` or `evaluate_elm()` would break the CLI and all existing tests. Instead:

- `compile_with_libraries(source, options, provider: &dyn LibrarySourceProvider) -> Result<CompileOutput, CompileErrors>` is a new function
- `evaluate_elm_with_libraries(lib, included: &HashMap<String, &elm::Library>, name, ctx) -> Result<Value>` is a new function
- The existing `compile()` and `evaluate_elm()` delegate to the new forms with a no-op `MemoryLibrarySourceProvider` and an empty map respectively, or continue as-is

### D3: Library map keyed by alias string in the evaluator

The ELM specification uses `LibraryRef` with a `localIdentifier` (the alias). At evaluation time, `ExpressionRef { library_name: Some("CaseLogic") }` needs to resolve `CaseLogic` to a compiled `elm::Library`. The engine receives a `&HashMap<String, Library>` (reference to a map of owned `elm::Library` values) where the key is the alias as declared in the main library's `include X called Y` — matching the ELM `localId` stored on the `IncludeDef` node.

The public `evaluate_elm_with_libraries` signature is:
```rust
pub fn evaluate_elm_with_libraries(
    library: &Library,
    included: &HashMap<String, Library>,
    name: &str,
    ctx: &EvalContext,
) -> Result<Value, EvalError>
```

`CompileOutputWithLibs.included` is `HashMap<String, elm::Library>` (owned values) so callers can pass `&out.included` directly without managing borrows. Using owned values rather than `&elm::Library` avoids lifetime complexity at the cost of a trivial allocation — acceptable since the map is constructed once per compile call.

Alternative considered: thread `LibraryIdentifier` → `CompiledLibrary` and look up by full identifier. Rejected — the alias string is what the ELM AST uses at runtime; the full identifier is only needed during loading.

### D4: CLI auto-discovery default — scan the input file's directory and its subdirectories

When running `rh cql eval path/to/Foo.cql`, the most common expectation is that included libraries live alongside `Foo.cql`. The CLI builds a `FileLibrarySourceProvider` using the directory of the input file. Users can override with `--lib-path <dir>`. Multiple `--lib-path` flags compose into a `CompositeLibrarySourceProvider`.

### D5: `run_compile_pipeline` takes an optional `&dyn LibrarySourceProvider`

Rather than adding a new pipeline function, `run_compile_pipeline` gains an optional library source provider parameter (`Option<&dyn LibrarySourceProvider>`). When `None`, behavior is identical to today. When `Some(provider)`, after parsing the library header the pipeline resolves all `include` directives recursively via the provider and `LibraryManager`, building a `HashMap<String, CompiledLibrary>` that is passed into semantic analysis.

## Risks / Trade-offs

- **Circular include detection** → `LibraryManager` already implements DFS cycle detection; any cycle returns an error that surfaces as a compile diagnostic.
- **Version conflicts** (two includes requiring different versions of the same library) → Not handled in this change; version-flexible matching is best-effort and first-found wins. Document as a known limitation.
- **Performance** (large dependency trees) → `LibraryManager` caches already-compiled libraries by `LibraryIdentifier`. Should be acceptable for local-file graphs; O(n) per unique library.
- **Semantic fallthrough** (include alias also happens to be a locally-defined expression name) → `SymbolKind::Include` is registered before expression definitions; inner-scope wins shadowing means a local expression named `CaseLogic` would shadow the alias. This is CQL-spec correct behavior.

## Open Questions

- Should `compile_with_libraries` return the full dependency graph (all `CompiledLibrary` instances) or only the main library's `CompileOutput`? For now: return only the main library's output plus a `HashMap<alias, CompiledLibrary>` adjacency structure so callers can pass it to `evaluate_elm_with_libraries`.
- Should the CLI print a warning when a required include cannot be found on the library path, before failing? Probably yes, but UX details are deferred to the tasks milestone.
