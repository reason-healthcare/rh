//! Public CQL compiler API.
//!
//! This module provides the main entry point for compiling CQL source code to ELM.
//! It integrates the parser, semantic analyzer, ELM emitter, and output generation
//! into a single, easy-to-use API through a shared internal compilation pipeline.
//!
//! # Pipeline
//!
//! All public functions are thin wrappers over the shared internal
//! [`run_compile_pipeline`] function which performs:
//!
//! 1. **Parse** — CQL source → AST via [`CqlParser`]
//! 2. **Analyze** — AST → [`TypedLibrary`] + diagnostics via [`SemanticAnalyzer`]
//! 3. **Emit** (optional) — [`TypedLibrary`] → [`elm::Library`] via [`ElmEmitter`]
//! 4. **Source map** (optional) — emitter side-channel populated during emit
//!
//! # Example
//!
//! ```
//! use rh_cql::{compile, CompilerOptions};
//!
//! let source = r#"
//!     library Example version '1.0.0'
//!     define Greeting: 'Hello, CQL!'
//!     define Answer: 42
//! "#;
//!
//! // Compile with default options
//! let result = compile(source, None);
//! assert!(result.is_ok());
//!
//! let output = result.unwrap();
//! assert!(output.library.identifier.is_some());
//! assert!(output.errors.is_empty());
//! ```

use crate::elm;
use crate::options::CompilerOptions;
use crate::output::{library_to_compact_json, library_to_json_with_options};
use crate::parser::CqlParser;
use crate::reporting::{Diagnostic, Severity};
use crate::semantics::typed_ast::TypedLibrary;
use std::collections::HashMap;
use std::sync::Arc;

// ── CompilationContext ───────────────────────────────────────────────────────

/// Shared compilation environment passed through the compiler and semantics
/// pipeline stages.
///
/// `CompilationContext` consolidates [`CompilerOptions`] and an optional
/// [`ModelInfoProvider`][crate::provider::ModelInfoProvider] so that the same
/// context object can be constructed once and threaded through
/// `run_compile_pipeline`, `SemanticAnalyzer`, and the ELM emitter without
/// repeating the same pair of arguments at every call site.
///
/// # Example
///
/// ```rust
/// use rh_cql::{CompilationContext, CompilerOptions, SignatureLevel};
///
/// let ctx = CompilationContext::new(
///     CompilerOptions::default().with_signature_level(SignatureLevel::All),
///     None,
/// );
/// assert!(ctx.model_provider().is_none());
/// ```
#[derive(Clone, Default)]
pub struct CompilationContext {
    /// Compiler options controlling translation behaviour.
    pub options: CompilerOptions,
    /// Optional model provider; `None` falls back to the bundled FHIR R4
    /// provider at the point of use.
    model_provider: Option<Arc<dyn crate::provider::ModelInfoProvider>>,
}

impl CompilationContext {
    /// Create a new context with the given options and an optional model
    /// provider.
    pub fn new(
        options: CompilerOptions,
        model_provider: Option<Arc<dyn crate::provider::ModelInfoProvider>>,
    ) -> Self {
        CompilationContext {
            options,
            model_provider,
        }
    }

    /// Return the model provider if one was supplied, or `None`.
    pub fn model_provider(&self) -> Option<&Arc<dyn crate::provider::ModelInfoProvider>> {
        self.model_provider.as_ref()
    }

    /// Resolve to a concrete provider: the stored one, or the default FHIR R4
    /// provider.
    pub(crate) fn resolve_provider(&self) -> Arc<dyn crate::provider::ModelInfoProvider> {
        self.model_provider
            .clone()
            .unwrap_or_else(|| Arc::new(crate::provider::fhir_r4_provider_from_package()))
    }
}

impl std::fmt::Debug for CompilationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilationContext")
            .field("options", &self.options)
            .field(
                "model_provider",
                if self.model_provider.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

// ── Internal shared pipeline ────────────────────────────────────────────────

/// Controls whether (and how) ELM is emitted after semantic analysis.
enum PipelineEmitMode {
    /// Skip ELM emission entirely (validate-only or explain paths).
    None,
    /// Emit ELM; do **not** build a source map.
    Elm,
    /// Emit ELM and build a source map.
    ElmWithSourceMap {
        /// Optional canonical URI written into [`SourceDocument`] entries.
        library_uri: Option<String>,
    },
}

/// Configuration for [`run_compile_pipeline`].
struct PipelineConfig {
    /// Shared compilation environment (options + optional model provider).
    context: CompilationContext,
    emit_mode: PipelineEmitMode,
    /// Pre-compiled included libraries keyed by alias.  Populated by
    /// `compile_with_libraries` after resolving include declarations; empty
    /// for the standard `compile()` / `validate()` paths.
    pre_compiled_includes: HashMap<String, elm::Library>,
}

/// Output produced by [`run_compile_pipeline`].
struct PipelineOutput {
    /// Populated when `emit_mode` is [`PipelineEmitMode::None`]; the typed
    /// library has **not** been consumed by the emitter.
    typed_library: Option<TypedLibrary>,
    /// Populated when `emit_mode` is [`PipelineEmitMode::Elm`] or
    /// [`PipelineEmitMode::ElmWithSourceMap`].
    library: Option<elm::Library>,
    /// Populated when `emit_mode` is [`PipelineEmitMode::ElmWithSourceMap`].
    source_map: Option<crate::sourcemap::SourceMap>,
    /// All diagnostics emitted during parse + analysis, converted to the
    /// unified [`Diagnostic`] type.
    diagnostics: Vec<Diagnostic>,
    /// Compiled included libraries keyed by their local alias, populated when
    /// a `library_provider` was supplied in the [`PipelineConfig`].
    included_libs: HashMap<String, elm::Library>,
}

/// Shared internal compilation pipeline.
///
/// Parse → Analyze → (optionally) Emit ELM → (optionally) build source map.
/// All public compiler functions delegate to this.
fn run_compile_pipeline(
    source: &str,
    config: PipelineConfig,
) -> Result<PipelineOutput, CompilationError> {
    // 1. Parse
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    // 2. Resolve model provider
    let provider = config.context.resolve_provider();

    // 3. Semantic analysis — register pre-compiled included library symbols.
    let mut analyzer = crate::semantics::analyzer::SemanticAnalyzer::new(
        Arc::clone(&provider),
        config.context.options.clone(),
    );
    for (alias, dep_lib) in &config.pre_compiled_includes {
        analyzer.register_included_library(alias, dep_lib);
    }
    let (typed_library, raw_diagnostics) = analyzer.analyze(ast);

    let diagnostics: Vec<Diagnostic> = raw_diagnostics.into_iter().map(Diagnostic::from).collect();
    let included_libs = config.pre_compiled_includes;

    // 4. Optionally emit ELM and/or source map
    match config.emit_mode {
        PipelineEmitMode::None => Ok(PipelineOutput {
            typed_library: Some(typed_library),
            library: None,
            source_map: None,
            diagnostics,
            included_libs,
        }),

        PipelineEmitMode::Elm => {
            let mut emitter = crate::emit::ElmEmitter::new(config.context.options.clone());
            let library = emitter.emit(typed_library);
            Ok(PipelineOutput {
                typed_library: None,
                library: Some(library),
                source_map: None,
                diagnostics,
                included_libs,
            })
        }

        PipelineEmitMode::ElmWithSourceMap { library_uri } => {
            let mut emitter = crate::emit::ElmEmitter::new(config.context.options.clone());
            let library = emitter.emit(typed_library);
            let mut source_map = emitter.take_source_map();

            // Populate doc_id from the library identifier
            let lib_id = library
                .identifier
                .as_ref()
                .and_then(|i| i.id.as_deref())
                .unwrap_or("");
            let lib_version = library
                .identifier
                .as_ref()
                .and_then(|i| i.version.as_deref())
                .unwrap_or("");
            let uri = library_uri.as_deref().unwrap_or("");
            let doc_id = crate::sourcemap::generate_doc_id(lib_id, lib_version, uri);

            source_map
                .source_documents
                .push(crate::sourcemap::SourceDocument {
                    doc_id: doc_id.clone(),
                    uri: uri.to_string(),
                    checksum: None,
                    line_index: None,
                });

            for mapping in &mut source_map.mappings {
                if mapping.doc_id.is_empty() {
                    mapping.doc_id = doc_id.clone();
                }
            }

            Ok(PipelineOutput {
                typed_library: None,
                library: Some(library),
                source_map: Some(source_map),
                diagnostics,
                included_libs,
            })
        }
    }
}

/// Recursively compile all `include` declarations using a [`LibrarySourceProvider`],
/// returning a map of `alias → compiled elm::Library`.
///
/// The `resolution_stack` tracks in-progress compilations for cycle detection.
fn resolve_includes_to_elm(
    includes: &[crate::parser::ast::IncludeDef],
    provider: &dyn crate::library::LibrarySourceProvider,
    options: &CompilerOptions,
    model_provider: &Arc<dyn crate::provider::ModelInfoProvider>,
    resolution_stack: &mut Vec<String>,
) -> Result<HashMap<String, elm::Library>, CompilationError> {
    let mut result = HashMap::new();

    for inc in includes {
        let alias = inc.alias.clone().unwrap_or_else(|| inc.path.clone());

        if resolution_stack.iter().any(|p| p == &inc.path) {
            return Err(CompilationError::LibraryNotFound {
                name: inc.path.clone(),
                searched_paths: vec![format!(
                    "circular dependency — '{}' is already being compiled",
                    inc.path
                )],
            });
        }

        let lib_id =
            crate::library::LibraryIdentifier::new(&inc.path, inc.version.as_deref());

        let source = provider.get_source(&lib_id).ok_or_else(|| {
            CompilationError::LibraryNotFound {
                name: inc.path.clone(),
                searched_paths: vec![],
            }
        })?;

        let parser = CqlParser::new();
        let dep_ast = parser
            .parse(&source.source)
            .map_err(|e| CompilationError::Parse(e.to_string()))?;

        resolution_stack.push(inc.path.clone());
        let transitive = resolve_includes_to_elm(
            &dep_ast.includes,
            provider,
            options,
            model_provider,
            resolution_stack,
        )?;
        resolution_stack.pop();

        let mut analyzer = crate::semantics::analyzer::SemanticAnalyzer::new(
            Arc::clone(model_provider),
            options.clone(),
        );
        for (dep_alias, dep_lib) in &transitive {
            analyzer.register_included_library(dep_alias, dep_lib);
        }
        let (typed_dep, _diags) = analyzer.analyze(dep_ast);
        let mut emitter = crate::emit::ElmEmitter::new(options.clone());
        let dep_elm = emitter.emit(typed_dep);

        result.insert(alias, dep_elm);
    }

    Ok(result)
}

/// The result of compiling CQL source code.
///
/// Contains the translated ELM library along with any errors or warnings
/// that occurred during compilation.  Diagnostics use the unified
/// [`Diagnostic`] type which carries a [`crate::reporting::DiagnosticCode`],
/// pipeline [`crate::reporting::DiagnosticStage`], optional source span, and
/// severity.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// The translated ELM library.
    pub library: elm::Library,
    /// Error-level diagnostics from compilation.
    pub errors: Vec<Diagnostic>,
    /// Warning-level diagnostics from compilation.
    pub warnings: Vec<Diagnostic>,
    /// Info-level messages from compilation.
    pub messages: Vec<Diagnostic>,
}

impl CompilationResult {
    /// Returns true if compilation completed without errors.
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns true if compilation had any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Returns true if compilation had any messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Get the total count of all issues (errors + warnings + messages).
    pub fn issue_count(&self) -> usize {
        self.errors.len() + self.warnings.len() + self.messages.len()
    }

    /// Convert the library to JSON string with pretty formatting.
    pub fn to_json(&self) -> Result<String, CompilationError> {
        let options = CompilerOptions::default();
        library_to_json_with_options(&self.library, &options)
            .map_err(|e| CompilationError::Output(e.to_string()))
    }

    /// Convert the library to compact JSON string.
    pub fn to_compact_json(&self) -> Result<String, CompilationError> {
        library_to_compact_json(&self.library).map_err(|e| CompilationError::Output(e.to_string()))
    }
}

/// Errors that can occur during CQL compilation.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CompilationError {
    /// Parse error in CQL source.
    #[error("Parse error: {0}")]
    Parse(String),

    /// Semantic error during translation.
    #[error("Semantic error: {0}")]
    Semantic(String),

    /// Error generating output.
    #[error("Output error: {0}")]
    Output(String),

    /// A required `include`d library could not be found by the provider.
    #[error("Library '{name}' not found on the library path")]
    LibraryNotFound {
        /// The library name (path) that was not found.
        name: String,
        /// Paths that were searched (may be empty if the provider does not
        /// report them).
        searched_paths: Vec<String>,
    },
}

/// Compile CQL source code to ELM.
///
/// This is the main entry point for the CQL compiler. Uses the multi-stage
/// pipeline: parse → semantic analysis → ELM emission.
///
/// # Arguments
///
/// * `source` - The CQL source code to compile.
/// * `options` - Optional compiler options. If None, default options are used.
///
/// # Returns
///
/// Returns a `CompilationResult` containing the ELM library and any
/// errors or warnings. If parsing fails, returns a `CompilationError`.
///
/// # Example
///
/// ```
/// use rh_cql::{compile, CompilerOptions, SignatureLevel};
///
/// let source = "library Test version '1.0' define X: 1 + 2";
///
/// // With default options
/// let result = compile(source, None).unwrap();
/// assert!(result.is_success());
///
/// // With custom options
/// let options = CompilerOptions::debug()
///     .with_signature_level(SignatureLevel::All);
/// let result = compile(source, Some(options)).unwrap();
/// ```
pub fn compile(
    source: &str,
    options: Option<CompilerOptions>,
) -> Result<CompilationResult, CompilationError> {
    compile_with_model(source, options, None)
}

/// Compile CQL source code to ELM with an optional custom model provider.
///
/// This allows specifying a custom model provider for type resolution.
/// For most FHIR-based CQL, `compile()` (which uses FHIR R4 by default) is
/// sufficient.
///
/// # Arguments
///
/// * `source` - The CQL source code to compile.
/// * `options` - Optional compiler options. If None, default options are used.
/// * `model_provider` - Optional model provider for type resolution. When
///   `None`, the bundled FHIR R4 provider is used.
///
/// # Returns
///
/// Returns a `CompilationResult` containing the ELM library and any
/// errors or warnings.
pub fn compile_with_model(
    source: &str,
    options: Option<CompilerOptions>,
    model_provider: Option<Arc<dyn crate::provider::ModelInfoProvider>>,
) -> Result<CompilationResult, CompilationError> {
    let options = options.unwrap_or_default();
    let output = run_compile_pipeline(
        source,
        PipelineConfig {
            context: CompilationContext::new(options.clone(), model_provider),
            emit_mode: PipelineEmitMode::Elm,
            pre_compiled_includes: HashMap::new(),
        },
    )?;

    let library = output
        .library
        .expect("pipeline with Elm mode must return a library");
    let (errors, warnings, messages) = categorize_exceptions(&output.diagnostics, &options);

    Ok(CompilationResult {
        library,
        errors,
        warnings,
        messages,
    })
}

/// Compile CQL source code directly to JSON ELM.
///
/// This is a convenience function that compiles CQL and immediately
/// serializes the result to JSON.
///
/// # Arguments
///
/// * `source` - The CQL source code to compile.
/// * `options` - Optional compiler options.
/// * `pretty` - Whether to format the JSON with indentation.
///
/// # Returns
///
/// Returns the ELM as a JSON string, or an error if compilation fails.
///
/// # Example
///
/// ```
/// use rh_cql::compile_to_json;
///
/// let source = "library Test version '1.0' define X: 42";
/// let json = compile_to_json(source, None, true).unwrap();
/// assert!(json.contains("\"id\": \"Test\""));
/// ```
pub fn compile_to_json(
    source: &str,
    options: Option<CompilerOptions>,
    pretty: bool,
) -> Result<String, CompilationError> {
    let result = compile(source, options)?;

    if !result.is_success() {
        // Return the first error message
        if let Some(err) = result.errors.first() {
            return Err(CompilationError::Semantic(err.message.clone()));
        }
    }

    if pretty {
        result.to_json()
    } else {
        result.to_compact_json()
    }
}

/// The result of compiling CQL with an attached source map.
#[derive(Debug, Clone)]
pub struct SourceMapCompilationResult {
    /// The translated ELM library.
    pub library: elm::Library,
    /// Source-map correlating CQL spans to ELM nodes.
    pub source_map: crate::sourcemap::SourceMap,
    /// Error-level diagnostics from compilation.
    pub errors: Vec<Diagnostic>,
    /// Warning-level diagnostics from compilation.
    pub warnings: Vec<Diagnostic>,
    /// Info-level messages from compilation.
    pub messages: Vec<Diagnostic>,
}

impl SourceMapCompilationResult {
    /// Returns true if compilation completed without errors.
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns true if compilation had any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the total count of all issues (errors + warnings + messages).
    pub fn issue_count(&self) -> usize {
        self.errors.len() + self.warnings.len() + self.messages.len()
    }

    /// Convert the library to JSON string with pretty formatting.
    pub fn to_json(&self) -> Result<String, CompilationError> {
        let options = CompilerOptions::default();
        library_to_json_with_options(&self.library, &options)
            .map_err(|e| CompilationError::Output(e.to_string()))
    }

    /// Convert the library to compact JSON string.
    pub fn to_compact_json(&self) -> Result<String, CompilationError> {
        library_to_compact_json(&self.library).map_err(|e| CompilationError::Output(e.to_string()))
    }

    /// Serialize the source map to a sidecar JSON string (`*.elm.sourcemap.json`).
    pub fn source_map_json(&self) -> Result<String, CompilationError> {
        self.source_map
            .to_json()
            .map_err(|e| CompilationError::Output(e.to_string()))
    }
}

/// Compile CQL source code to ELM and produce a source map.
///
/// Uses the shared multi-stage pipeline: parse → semantic analysis
/// (`SemanticAnalyzer`) → ELM emission (`ElmEmitter`).  The emitter records a
/// [`SourceMap`] as a side-channel during emission, correlating [`SourceSpan`]
/// on each [`TypedNode`] to the ELM node ids it produces.
///
/// # Arguments
///
/// * `source` - The CQL source code to compile.
/// * `options` - Optional compiler options. If `None`, default options are used.
/// * `library_uri` - Optional canonical URI used to populate `doc_id` in the
///   source map. If `None` an empty URI is used.
///
/// # Returns
///
/// Returns a [`SourceMapCompilationResult`] containing the ELM library,
/// attached source map, and any diagnostics.
///
/// # Example
///
/// ```
/// use rh_cql::compile_to_elm_with_sourcemap;
///
/// let source = "library Test version '1.0' define X: 1 + 2";
/// let result = compile_to_elm_with_sourcemap(source, None, None).unwrap();
/// assert!(result.is_success());
/// // The source map is always present (may be empty when spans are absent)
/// let _json = result.source_map_json().unwrap();
/// ```
pub fn compile_to_elm_with_sourcemap(
    source: &str,
    options: Option<CompilerOptions>,
    library_uri: Option<&str>,
) -> Result<SourceMapCompilationResult, CompilationError> {
    let options = options.unwrap_or_default();
    let output = run_compile_pipeline(
        source,
        PipelineConfig {
            context: CompilationContext::new(options.clone(), None),
            emit_mode: PipelineEmitMode::ElmWithSourceMap {
                library_uri: library_uri.map(str::to_owned),
            },
            pre_compiled_includes: HashMap::new(),
        },
    )?;

    let library = output
        .library
        .expect("pipeline with ElmWithSourceMap mode must return a library");
    let source_map = output
        .source_map
        .expect("pipeline with ElmWithSourceMap mode must return a source map");
    let (errors, warnings, messages) = categorize_exceptions(&output.diagnostics, &options);

    Ok(SourceMapCompilationResult {
        library,
        source_map,
        errors,
        warnings,
        messages,
    })
}

// ---------------------------------------------------------------------------
// compile_with_libraries
// ---------------------------------------------------------------------------

/// The result of compiling a CQL library whose `include` declarations were
/// resolved via a [`LibrarySourceProvider`].
///
/// Bundles the main [`CompilationResult`] with the dependency map so the
/// caller can pass both to [`evaluate_elm_with_libraries`][crate::evaluate_elm_with_libraries].
#[derive(Debug, Clone)]
pub struct CompileOutputWithLibs {
    /// The compiled main library and its diagnostics.
    pub result: CompilationResult,
    /// Compiled included libraries keyed by the local alias declared in the
    /// main library's `include X called <alias>` statement.
    pub included: HashMap<String, elm::Library>,
}

/// Compile a CQL library and recursively resolve all `include` declarations
/// using the supplied [`LibrarySourceProvider`].
///
/// This is the additive entry point for library-aware compilation.  The
/// existing [`compile`] / [`validate`] functions are unchanged; this function
/// adds library resolution on top.
///
/// # Arguments
///
/// * `source`   — CQL source of the *main* library.
/// * `options`  — Optional compiler options.
/// * `provider` — Source provider used to locate included `.cql` files.
///
/// # Returns
///
/// Returns a [`CompileOutputWithLibs`] bundling the main [`CompilationResult`]
/// with the `included` dependency map on success, or a [`CompilationError`]
/// (including [`CompilationError::LibraryNotFound`]) on failure.
///
/// # Example
///
/// ```
/// use rh_cql::{compile_with_libraries, CompilerOptions};
/// use rh_cql::library::{MemoryLibrarySourceProvider, LibraryIdentifier};
///
/// let helper_src = "library Helper version '1.0' define Answer: 42";
/// let main_src = r#"
///     library Main version '1.0'
///     include Helper version '1.0' called H
///     define MyAnswer: H.Answer
/// "#;
///
/// let provider = MemoryLibrarySourceProvider::new();
/// provider.register_source(LibraryIdentifier::new("Helper", Some("1.0")), helper_src.to_string());
///
/// let out = compile_with_libraries(main_src, None, &provider).unwrap();
/// assert!(out.result.is_success());
/// assert!(out.included.contains_key("H"));
/// ```
pub fn compile_with_libraries(
    source: &str,
    options: Option<CompilerOptions>,
    provider: &dyn crate::library::LibrarySourceProvider,
) -> Result<CompileOutputWithLibs, CompilationError> {
    let options = options.unwrap_or_default();

    // Resolve all include declarations by compiling dependencies before the
    // main pipeline runs.  A temporary parse is needed to extract the include
    // declarations from the main library header.
    let parser = CqlParser::new();
    let ast_for_includes = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    let model_provider = CompilationContext::new(options.clone(), None).resolve_provider();
    let mut stack = Vec::new();
    let pre_compiled_includes = resolve_includes_to_elm(
        &ast_for_includes.includes,
        provider,
        &options,
        &model_provider,
        &mut stack,
    )?;

    let output = run_compile_pipeline(
        source,
        PipelineConfig {
            context: CompilationContext::new(options.clone(), None),
            emit_mode: PipelineEmitMode::Elm,
            pre_compiled_includes,
        },
    )?;

    let library = output
        .library
        .expect("pipeline with Elm mode must return a library");
    let (errors, warnings, messages) = categorize_exceptions(&output.diagnostics, &options);

    Ok(CompileOutputWithLibs {
        result: CompilationResult {
            library,
            errors,
            warnings,
            messages,
        },
        included: output.included_libs,
    })
}

/// Validate CQL source code without producing ELM output.
///
/// Runs parse + semantic analysis via the shared pipeline (skipping ELM
/// emission).  Only validation diagnostics are returned.
///
/// # Arguments
///
/// * `source` - The CQL source code to validate.
/// * `options` - Optional compiler options.
///
/// # Returns
///
/// Returns a `ValidationResult` containing any errors or warnings.
///
/// # Example
///
/// ```
/// use rh_cql::validate;
///
/// let source = "library Test version '1.0' define X: 1 + 2";
/// let result = validate(source, None).unwrap();
/// assert!(result.is_valid());
/// ```
pub fn validate(
    source: &str,
    options: Option<CompilerOptions>,
) -> Result<ValidationResult, CompilationError> {
    let options = options.unwrap_or_default();
    let output = run_compile_pipeline(
        source,
        PipelineConfig {
            context: CompilationContext::new(options.clone(), None),
            emit_mode: PipelineEmitMode::None,
            pre_compiled_includes: HashMap::new(),
        },
    )?;

    let (errors, warnings, messages) = categorize_exceptions(&output.diagnostics, &options);

    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
        messages,
    })
}

/// The result of validating CQL source code.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the source is valid (no errors).
    pub is_valid: bool,
    /// Error-level diagnostics found during validation.
    pub errors: Vec<Diagnostic>,
    /// Warning-level diagnostics found during validation.
    pub warnings: Vec<Diagnostic>,
    /// Info-level messages.
    pub messages: Vec<Diagnostic>,
}

impl ValidationResult {
    /// Returns true if the source is valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Returns true if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the total count of all issues.
    pub fn issue_count(&self) -> usize {
        self.errors.len() + self.warnings.len() + self.messages.len()
    }
}

/// Explain the parse-level AST structure of CQL source.
///
/// Parses the source and produces a human-readable description of the AST,
/// including definition names and node types.
///
/// # Arguments
///
/// * `source` - The CQL source code to explain.
///
/// # Returns
///
/// Returns a multi-line string describing the parsed AST, or a
/// `CompilationError` if the source cannot be parsed.
///
/// # Example
///
/// ```
/// use rh_cql::explain_parse;
///
/// let source = "library Test version '1.0' define X: 1 + 2";
/// let explanation = explain_parse(source).unwrap();
/// assert!(explanation.contains("ExpressionDef"));
/// ```
pub fn explain_parse(source: &str) -> Result<String, CompilationError> {
    let parser = CqlParser::new();
    // NOTE: On parse failure the entire AST is unavailable — partial AST is not
    // yet supported because the parser does not implement error-recovery.  The
    // function therefore returns `Err(CompilationError::Parse(...))` rather than
    // a best-effort partial tree.  See the spec note in explain/mod.rs.
    // TODO: partial AST on parse error (requires parser recovery support)
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;
    Ok(crate::explain::explain_parse(&ast))
}

/// Explain the semantic analysis of CQL source.
///
/// Parses and analyzes the source, then produces a human-readable narrative
/// describing resolved types, overload selections, and implicit conversions
/// for each expression node.
///
/// # Arguments
///
/// * `source` - The CQL source code to explain.
/// * `options` - Optional compiler options. If None, default options are used.
///
/// # Returns
///
/// Returns a multi-line string describing the semantic analysis, or a
/// `CompilationError` if the source cannot be parsed.
///
/// # Example
///
/// ```
/// use rh_cql::explain_compile;
///
/// let source = "library Test version '1.0' define X: 1 + 2";
/// let explanation = explain_compile(source, None).unwrap();
/// assert!(explanation.contains("ExpressionDef"));
/// ```
pub fn explain_compile(
    source: &str,
    options: Option<CompilerOptions>,
) -> Result<String, CompilationError> {
    let options = options.unwrap_or_default();
    let output = run_compile_pipeline(
        source,
        PipelineConfig {
            context: CompilationContext::new(options, None),
            emit_mode: PipelineEmitMode::None,
            pre_compiled_includes: HashMap::new(),
        },
    )?;
    let typed_library = output
        .typed_library
        .expect("pipeline with None emit mode must return typed_library");
    Ok(crate::explain::explain_compile(&typed_library))
}

/// Categorize a flat list of [`Diagnostic`] items by severity.
fn categorize_exceptions(
    diagnostics: &[Diagnostic],
    _options: &CompilerOptions,
) -> (Vec<Diagnostic>, Vec<Diagnostic>, Vec<Diagnostic>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut messages = Vec::new();

    for d in diagnostics {
        match d.severity {
            Severity::Error => errors.push(d.clone()),
            Severity::Warning => warnings.push(d.clone()),
            Severity::Info => messages.push(d.clone()),
        }
    }

    (errors, warnings, messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::SignatureLevel;

    #[test]
    fn test_compile_simple() {
        let source = "library Test version '1.0' define X: 42";
        let result = compile(source, None).unwrap();

        assert!(result.is_success());
        assert!(result.library.identifier.is_some());

        let id = result.library.identifier.as_ref().unwrap();
        assert_eq!(id.id, Some("Test".to_string()));
        assert_eq!(id.version, Some("1.0".to_string()));
    }

    #[test]
    fn test_compile_with_usings() {
        let source = r#"
            library Test version '1.0'
            using FHIR version '4.0.1'
            context Patient
            define Name: Patient.name
        "#;
        let result = compile(source, None).unwrap();

        assert!(result.is_success());
        assert!(result.library.usings.is_some());
        assert!(result.library.contexts.is_some());
        assert!(result.library.statements.is_some());
    }

    #[test]
    fn test_compile_with_options() {
        let source = "library Test version '1.0' define X: 1 + 2";
        let options = CompilerOptions::debug().with_signature_level(SignatureLevel::All);

        let result = compile(source, Some(options)).unwrap();
        assert!(result.is_success());
    }

    #[test]
    fn test_compile_to_json() {
        let source = "library Test version '1.0' define X: 42";
        let json = compile_to_json(source, None, true).unwrap();

        assert!(json.contains("\"id\": \"Test\""));
        assert!(json.contains("\"version\": \"1.0\""));
    }

    #[test]
    fn test_compile_to_compact_json() {
        let source = "library Test version '1.0' define X: 42";
        let json = compile_to_json(source, None, false).unwrap();

        // Compact JSON shouldn't have newlines between elements
        assert!(json.contains("\"id\":\"Test\""));
    }

    #[test]
    fn test_compile_parse_error() {
        let source = "this is not valid CQL @@@@";
        let result = compile(source, None);

        assert!(result.is_err());
        if let Err(CompilationError::Parse(msg)) = result {
            assert!(!msg.is_empty());
        } else {
            panic!("Expected parse error");
        }
    }

    #[test]
    fn test_validate_valid_source() {
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = validate(source, None).unwrap();

        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_parse_error() {
        let source = "not valid cql at all!!";
        let result = validate(source, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_compilation_result_methods() {
        let source = "library Test version '1.0' define X: 42";
        let result = compile(source, None).unwrap();

        assert!(result.is_success());
        assert!(!result.has_warnings());
        assert!(!result.has_messages());
        assert_eq!(result.issue_count(), 0);

        // Test JSON conversion
        let json = result.to_json().unwrap();
        assert!(json.contains("Test"));

        let compact = result.to_compact_json().unwrap();
        assert!(compact.contains("Test"));
    }

    #[test]
    fn test_explain_parse_returns_expression_defs() {
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = explain_parse(source).unwrap();
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
    }

    #[test]
    fn test_explain_parse_returns_function_defs() {
        let source = "library Test version '1.0' define function Add(a Integer, b Integer): a + b";
        let result = explain_parse(source).unwrap();
        assert!(result.contains("FunctionDef(Add)"), "got: {result}");
    }

    #[test]
    fn test_explain_parse_error_on_invalid_source() {
        let source = "not valid cql @@@@";
        let result = explain_parse(source);
        assert!(matches!(result, Err(CompilationError::Parse(_))));
    }

    #[test]
    fn test_explain_parse_ast_header() {
        let source = "library Test version '1.0' define X: 42";
        let result = explain_parse(source).unwrap();
        assert!(result.starts_with("AST Explanation:"), "got: {result}");
    }

    #[test]
    fn test_explain_parse_walks_child_nodes() {
        // Verifies that explain_parse recurses into expression bodies, printing
        // child nodes with indentation rather than only top-level defs.
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = explain_parse(source).unwrap();
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
        assert!(
            result.contains("BinaryExpression(Add)"),
            "child BinaryExpression(Add) not found — tree walk may not be working: {result}"
        );
        assert!(
            result.contains("Literal(Integer: 1)"),
            "Literal(Integer: 1) not found: {result}"
        );
        assert!(
            result.contains("Literal(Integer: 2)"),
            "Literal(Integer: 2) not found: {result}"
        );
    }

    #[test]
    fn test_explain_compile_source_locations() {
        // Verifies that semantic events are prefixed with [line:col].
        // Even if no semantic events fire, the function must not panic.
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = explain_compile(source, None).unwrap();
        assert!(!result.is_empty());
        // If any event is emitted it MUST include a [line:col] prefix.
        if result.lines().any(|l| l.trim_start().starts_with('[')) {
            let has_location = result.lines().any(|l| {
                let t = l.trim_start();
                t.starts_with('[') && t.contains(':')
            });
            assert!(
                has_location,
                "event line missing [line:col] prefix: {result}"
            );
        }
    }

    #[test]
    fn test_explain_compile_returns_expression_defs() {
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = explain_compile(source, None).unwrap();
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_header() {
        let source = "library Test version '1.0' define X: 42";
        let result = explain_compile(source, None).unwrap();
        assert!(result.starts_with("Compile Explanation:"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_multiple_defs() {
        let source = r#"
            library Test version '1.0'
            define A: 1
            define B: 2
            define C: A + B
        "#;
        let result = explain_compile(source, None).unwrap();
        assert!(result.contains("ExpressionDef(A)"), "got: {result}");
        assert!(result.contains("ExpressionDef(B)"), "got: {result}");
        assert!(result.contains("ExpressionDef(C)"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_parse_error_propagates() {
        let source = "not valid cql @@@@";
        let result = explain_compile(source, None);
        assert!(matches!(result, Err(CompilationError::Parse(_))));
    }

    #[test]
    fn test_explain_compile_records_overload_resolution() {
        // Arithmetic uses an overloaded operator — the overload should be recorded
        // once the analyzer populates SemanticMeta.resolved_overload.  Until that
        // field is set by the analyzer for binary operations, this test merely
        // verifies the function does not panic and returns a well-formed string.
        // TODO: strengthen to assert `result.contains("overload resolved")` once
        // SemanticMeta.resolved_overload is populated by the semantic analyzer.
        let source = "library Test version '1.0' define X: 1 + 2";
        let result = explain_compile(source, None).unwrap();
        assert!(!result.is_empty());
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_with_options() {
        use crate::options::CompilerOption;
        let source = "library Test version '1.0' define X: 1 + 2";
        let opts = CompilerOptions::new().with_option(CompilerOption::EnableResultTypes);
        let result = explain_compile(source, Some(opts)).unwrap();
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_function_def() {
        let source = "library Test version '1.0' define function Double(x Integer): x * 2";
        let result = explain_compile(source, None).unwrap();
        assert!(result.contains("FunctionDef(Double)"), "got: {result}");
    }

    #[test]
    fn test_explain_compile_implicit_conversions() {
        // Integer divided by Integer with decimal promotion: the body should
        // reference a conversion wrapper (if emitted by analyzer) or at minimum
        // not panic.
        // TODO: strengthen to assert `result.contains("implicit conversions")` once
        // SemanticMeta.implicit_conversions is populated by the semantic analyzer.
        let source = "library Test version '1.0' define X: 5 / 2";
        let result = explain_compile(source, None).unwrap();
        assert!(!result.is_empty());
        assert!(result.contains("ExpressionDef(X)"), "got: {result}");
    }

    #[test]
    fn test_compile_with_terminology() {
        let source = r#"
            library Test version '1.0'
            codesystem "LOINC": 'http://loinc.org'
            valueset "BP": 'http://example.org/vs/bp'
            code "SysBP": '8480-6' from "LOINC"
        "#;
        let result = compile(source, None).unwrap();

        assert!(result.is_success());
        assert!(result.library.code_systems.is_some());
        assert!(result.library.value_sets.is_some());
        assert!(result.library.codes.is_some());
    }

    #[test]
    fn test_compile_with_parameters() {
        let source = r#"
            library Test version '1.0'
            parameter MeasurementPeriod Interval<DateTime>
            define InPeriod: Today() in MeasurementPeriod
        "#;
        let result = compile(source, None).unwrap();

        assert!(result.is_success());
        assert!(result.library.parameters.is_some());
    }

    #[test]
    fn test_compile_complex_library() {
        let source = r#"
            library ComplexLib version '2.0'
            using FHIR version '4.0.1'
            
            codesystem "SNOMED": 'http://snomed.info/sct'
            valueset "Diabetes": 'http://cts.nlm.nih.gov/vs/diabetes'
            
            parameter MeasurementPeriod Interval<DateTime> default Interval[@2024-01-01, @2024-12-31]
            
            context Patient
            
            define "Is Adult":
                AgeInYears() >= 18
            
            define "In Period":
                Today() in MeasurementPeriod
        "#;
        let result = compile(source, None).unwrap();

        assert!(result.is_success());

        // Verify all sections present
        assert!(result.library.identifier.is_some());
        assert!(result.library.usings.is_some());
        assert!(result.library.code_systems.is_some());
        assert!(result.library.value_sets.is_some());
        assert!(result.library.parameters.is_some());
        assert!(result.library.contexts.is_some());
        assert!(result.library.statements.is_some());
    }

    // ── Task 1.4 regression tests: consistent diagnostics across all pipeline
    // entry-points.  Each pair asserts that the same source produces the same
    // errors/warnings regardless of which public function is called. ─────────

    /// Source that compiles cleanly — used to verify zero-error consistency.
    const CLEAN_SOURCE: &str = "library T version '1.0' define X: 1 + 2";
    /// Source that causes a semantic error (undefined reference).
    const ERROR_SOURCE: &str = "library T version '1.0' define X: UndefinedRef + 1";

    #[test]
    fn test_consistent_diagnostics_clean_compile_vs_validate() {
        let c = compile(CLEAN_SOURCE, None).unwrap();
        let v = validate(CLEAN_SOURCE, None).unwrap();
        assert_eq!(
            c.errors.len(),
            v.errors.len(),
            "error count must match between compile and validate"
        );
        assert_eq!(
            c.warnings.len(),
            v.warnings.len(),
            "warning count must match between compile and validate"
        );
    }

    #[test]
    fn test_consistent_diagnostics_error_compile_vs_validate() {
        let c = compile(ERROR_SOURCE, None).unwrap();
        let v = validate(ERROR_SOURCE, None).unwrap();
        assert_eq!(
            c.errors.len(),
            v.errors.len(),
            "error count must match between compile and validate (error source)"
        );
    }

    #[test]
    fn test_consistent_diagnostics_compile_vs_sourcemap() {
        let c = compile(CLEAN_SOURCE, None).unwrap();
        let sm = compile_to_elm_with_sourcemap(CLEAN_SOURCE, None, None).unwrap();
        assert_eq!(
            c.errors.len(),
            sm.errors.len(),
            "error count must match between compile and compile_to_elm_with_sourcemap"
        );
        assert_eq!(
            c.warnings.len(),
            sm.warnings.len(),
            "warning count must match between compile and compile_to_elm_with_sourcemap"
        );
    }

    #[test]
    fn test_consistent_diagnostics_error_compile_vs_sourcemap() {
        let c = compile(ERROR_SOURCE, None).unwrap();
        let sm = compile_to_elm_with_sourcemap(ERROR_SOURCE, None, None).unwrap();
        assert_eq!(
            c.errors.len(),
            sm.errors.len(),
            "error count must match between compile and compile_to_elm_with_sourcemap (error source)"
        );
    }

    #[test]
    fn test_compile_with_model_none_matches_compile() {
        // compile_with_model(src, None, None) must behave identically to compile(src, None).
        let c = compile(CLEAN_SOURCE, None).unwrap();
        let m = compile_with_model(CLEAN_SOURCE, None, None).unwrap();
        assert_eq!(c.errors.len(), m.errors.len());
        assert_eq!(c.warnings.len(), m.warnings.len());
        // Both should produce a library with the same identifier
        let c_id = c.library.identifier.as_ref().and_then(|i| i.id.as_deref());
        let m_id = m.library.identifier.as_ref().and_then(|i| i.id.as_deref());
        assert_eq!(c_id, m_id);
    }

    #[test]
    fn test_sourcemap_result_has_json_helpers() {
        // Verify to_json / to_compact_json / source_map_json work on
        // SourceMapCompilationResult (not just CompilationResult).
        let sm = compile_to_elm_with_sourcemap(CLEAN_SOURCE, None, None).unwrap();
        assert!(sm.is_success());
        let json = sm.to_json().unwrap();
        assert!(
            json.contains("\"id\": \"T\""),
            "pretty JSON must contain id: {json}"
        );
        let compact = sm.to_compact_json().unwrap();
        assert!(
            compact.contains("\"id\":\"T\""),
            "compact JSON must contain id: {compact}"
        );
        let _sm_json = sm.source_map_json().unwrap(); // must not panic
    }

    #[test]
    fn test_explain_compile_consistent_with_compile_defs() {
        // explain_compile must surface the same top-level definitions as compile.
        let source = r#"library T version '1.0' define A: 1 define B: 2"#;
        let c = compile(source, None).unwrap();
        let exp = explain_compile(source, None).unwrap();

        // compile should report the same number of statement defs
        let stmt_count = c
            .library
            .statements
            .as_ref()
            .map(|s| s.defs.len())
            .unwrap_or(0);
        assert!(
            stmt_count >= 2,
            "expected at least 2 defs; got {stmt_count}"
        );
        assert!(exp.contains("ExpressionDef(A)"), "explain missing A: {exp}");
        assert!(exp.contains("ExpressionDef(B)"), "explain missing B: {exp}");
    }
}
