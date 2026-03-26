## ADDED Requirements

### Requirement: LibrarySourceProvider wired into compile pipeline

`compile_with_libraries(source, options, provider)` SHALL compile a CQL library and recursively resolve all `include` directives by loading and compiling each dependency via the supplied `LibrarySourceProvider`. The resulting compiled library map SHALL be made available for evaluation.

#### Scenario: Compile main library with include
- **WHEN** `compile_with_libraries` is called on a CQL file that includes `FHIRHelpers`
- **THEN** `FHIRHelpers.cql` is resolved via the provider, compiled, and the resulting `CompiledLibrary` is available alongside the main library output

#### Scenario: Missing include returns compile error
- **WHEN** `compile_with_libraries` is called on a CQL file that includes `UnknownLib` and no provider can locate it
- **THEN** a compile error is returned identifying the missing library by name

#### Scenario: Circular include detected
- **WHEN** library A includes B and B includes A
- **THEN** compilation returns an error identifying the dependency cycle

#### Scenario: Existing compile() behavior unchanged
- **WHEN** `compile(source, options)` is called on a CQL file that uses `include`
- **THEN** the call succeeds (with any existing behavior) and does not attempt library resolution

### Requirement: FileLibrarySourceProvider discovers CQL files by library name

`FileLibrarySourceProvider` SHALL search a list of directories for a CQL source file matching a given `LibraryIdentifier`. For identifier `(name, version)` it SHALL try `Name-Version.cql` first, then `Name.cql`. Search is case-sensitive. Multiple search paths are composed via `CompositeLibrarySourceProvider`.

#### Scenario: Name-only file found
- **WHEN** the provider searches for library `FHIRHelpers` (no version) in `/path/to/cql/`
- **THEN** it returns the source of `/path/to/cql/FHIRHelpers.cql`

#### Scenario: Versioned file preferred
- **WHEN** both `FHIRHelpers.cql` and `FHIRHelpers-4.0.1.cql` exist in the search path
- **THEN** the versioned file is returned when version `4.0.1` is requested

#### Scenario: Unknown library returns None
- **WHEN** no file matching the library name exists in any search directory
- **THEN** the provider returns `None`

### Requirement: evaluate_elm_with_libraries resolves cross-library expressions

`evaluate_elm_with_libraries(lib, included, name, ctx)` SHALL evaluate a named expression from `lib` where `included` is a `HashMap<String, &elm::Library>` mapping include aliases to compiled libraries. When the engine encounters `ExpressionRef { library_name: Some(alias) }` or `FunctionRef { library_name: Some(alias) }`, it SHALL look up `alias` in `included` and evaluate the referenced expression or function in the found library.

#### Scenario: Cross-library expression ref resolves
- **WHEN** main library includes `CaseLogic` and evaluates `CaseLogic."Acute Case 1 with Stage 1 Hypertension Without Active Therapy"`
- **THEN** the engine looks up `CaseLogic` in the included map and evaluates the named expression from that library, returning the correct value

#### Scenario: Cross-library function ref resolves
- **WHEN** main library calls `FHIRHelpers.ToString(someCode)`
- **THEN** the engine looks up `FHIRHelpers` in the included map and evaluates `ToString` with the supplied argument

#### Scenario: Missing alias returns eval error
- **WHEN** the ELM references library alias `MissingLib` but the alias is not in `included`
- **THEN** evaluation returns an `EvalError` identifying the missing library alias

### Requirement: CLI auto-discovers CQL libraries from input file directory

When `rh cql eval <file>` or `rh cql validate <file>` is run, the CLI SHALL build a `FileLibrarySourceProvider` with the directory containing `<file>` as the default search path. Users MAY override or extend the search path with `--lib-path <dir>` (repeatable).

#### Scenario: Default library path is the input file's directory
- **WHEN** `rh cql eval /path/to/measures/MyLib.cql "Expr"` is run
- **THEN** the CLI searches `/path/to/measures/` for included CQL files

#### Scenario: --lib-path overrides the search root
- **WHEN** `rh cql eval MyLib.cql "Expr" --lib-path /shared/cql --lib-path ./local/`
- **THEN** the CLI builds a composite provider searching both `/shared/cql/` and `./local/` before the default

#### Scenario: Library not found emits actionable error
- **WHEN** a required include cannot be found on any search path
- **THEN** the CLI outputs an error indicating the missing library name and the paths that were searched
