//! Public CQL compiler API.
//!
//! This module provides the main entry point for compiling CQL source code to ELM.
//! It integrates the parser, preprocessor, builder, and output generation into
//! a single, easy-to-use API.
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

use crate::builder::LibraryBuilder;
use crate::elm;
use crate::options::CompilerOptions;
use crate::output::{library_to_compact_json, library_to_json_with_options};
use crate::parser::CqlParser;
use crate::reporting::{CqlCompilerException, Severity};
use std::sync::Arc;

/// The result of compiling CQL source code.
///
/// Contains the translated ELM library along with any errors or warnings
/// that occurred during compilation.
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// The translated ELM library.
    pub library: elm::Library,
    /// Errors that occurred during compilation.
    pub errors: Vec<CqlCompilerException>,
    /// Warnings that occurred during compilation.
    pub warnings: Vec<CqlCompilerException>,
    /// Informational messages from compilation.
    pub messages: Vec<CqlCompilerException>,
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
}

/// Compile CQL source code to ELM.
///
/// This is the main entry point for the CQL compiler. It parses the CQL source,
/// performs semantic analysis, and produces an ELM library.
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
    // Use FHIR R4 model provider - prefer package with full ModelInfo,
    // fall back to built-in minimal info if package not available
    let provider = crate::provider::fhir_r4_provider_from_package();
    compile_with_model(source, options, Some(&provider))
}

/// Compile CQL source code to ELM with a custom model provider.
///
/// This allows specifying a custom model provider for type resolution.
/// For most FHIR-based CQL, use `compile()` which provides FHIR R4 by default.
///
/// # Arguments
///
/// * `source` - The CQL source code to compile.
/// * `options` - Optional compiler options. If None, default options are used.
/// * `model_provider` - Optional model provider for type resolution.
///
/// # Returns
///
/// Returns a `CompilationResult` containing the ELM library and any
/// errors or warnings.
pub fn compile_with_model(
    source: &str,
    options: Option<CompilerOptions>,
    model_provider: Option<&dyn crate::provider::ModelInfoProvider>,
) -> Result<CompilationResult, CompilationError> {
    let options = options.unwrap_or_default();

    // Parse the CQL source
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    // Create the builder and translate
    let mut builder = LibraryBuilder::new();
    builder.set_options(options.clone());

    // Set model provider if provided
    if let Some(provider) = model_provider {
        builder.set_model_provider(provider);

        // Load conversion registry from ModelInfo if available
        // The CQL file typically includes FHIR model via 'using FHIR version ...'
        // We load conversions from the FHIR ModelInfo which contains
        // FHIRHelpers conversion definitions.
        if let Some(model_info) = provider.get_model("FHIR", None) {
            let registry = crate::conversion::ConversionRegistry::from_model_info(&model_info);
            builder.set_conversion_registry(registry);
        }
    }

    let library = builder.build(&ast);

    // Collect errors/warnings from builder
    let (errors, warnings, messages) = categorize_exceptions(builder.errors(), &options);

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
            return Err(CompilationError::Semantic(err.message().to_string()));
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
    /// Errors that occurred during compilation.
    pub errors: Vec<crate::reporting::CqlCompilerException>,
    /// Warnings that occurred during compilation.
    pub warnings: Vec<crate::reporting::CqlCompilerException>,
    /// Informational messages from compilation.
    pub messages: Vec<crate::reporting::CqlCompilerException>,
}

impl SourceMapCompilationResult {
    /// Returns true if compilation completed without errors.
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
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
/// Uses the multi-stage pipeline: parse → semantic analysis (`SemanticAnalyzer`)
/// → ELM emission (`ElmEmitter`).  The emitter records a [`SourceMap`] as a
/// side-channel during emission, correlating [`SourceSpan`] on each
/// [`TypedNode`] to the ELM node ids it produces.
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
    use std::sync::Arc;

    let options = options.unwrap_or_default();

    // Parse
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    // Semantic analysis
    let provider: Arc<dyn crate::provider::ModelInfoProvider> =
        Arc::new(crate::provider::fhir_r4_provider_from_package());
    let analyzer =
        crate::semantics::analyzer::SemanticAnalyzer::new(Arc::clone(&provider), options.clone());
    let (typed_library, diagnostics) = analyzer.analyze(ast);

    // ELM emission — builds source map as a side-channel
    let mut emitter = crate::emit::ElmEmitter::new(options.clone());
    let library = emitter.emit(typed_library);
    let mut source_map = emitter.take_source_map();

    // Populate doc_id in the source map now that we know the library identifier
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
    let uri = library_uri.unwrap_or("");
    let doc_id = crate::sourcemap::generate_doc_id(lib_id, lib_version, uri);

    // Register the source document
    source_map
        .source_documents
        .push(crate::sourcemap::SourceDocument {
            doc_id: doc_id.clone(),
            uri: uri.to_string(),
            checksum: None,
            line_index: None,
        });

    // Back-fill the doc_id into every mapping that was recorded with an empty one
    for mapping in &mut source_map.mappings {
        if mapping.doc_id.is_empty() {
            mapping.doc_id = doc_id.clone();
        }
    }

    let (errors, warnings, messages) = categorize_exceptions(&diagnostics, &options);

    Ok(SourceMapCompilationResult {
        library,
        source_map,
        errors,
        warnings,
        messages,
    })
}

/// Validate CQL source code without producing ELM output.
///
/// This function parses and performs semantic analysis on the CQL source
/// using `SemanticAnalyzer` directly, skipping the ELM emit step.
/// Only validation results are returned.
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

    // Parse the CQL source
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    // Use SemanticAnalyzer directly — parse + analyze only, no ELM emit
    let provider: Arc<dyn crate::provider::ModelInfoProvider> =
        Arc::new(crate::provider::fhir_r4_provider_from_package());
    let analyzer = crate::semantics::analyzer::SemanticAnalyzer::new(provider, options.clone());
    let (_, diagnostics) = analyzer.analyze(ast);

    let (errors, warnings, messages) = categorize_exceptions(&diagnostics, &options);

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
    /// Errors found during validation.
    pub errors: Vec<CqlCompilerException>,
    /// Warnings found during validation.
    pub warnings: Vec<CqlCompilerException>,
    /// Informational messages.
    pub messages: Vec<CqlCompilerException>,
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
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;
    let provider: Arc<dyn crate::provider::ModelInfoProvider> =
        Arc::new(crate::provider::fhir_r4_provider_from_package());
    let analyzer = crate::semantics::analyzer::SemanticAnalyzer::new(provider, options);
    let (typed_library, _diagnostics) = analyzer.analyze(ast);
    Ok(crate::explain::explain_compile(&typed_library))
}

/// Categorize exceptions by severity.
fn categorize_exceptions(
    exceptions: &[crate::reporting::CqlCompilerException],
    _options: &CompilerOptions,
) -> (
    Vec<CqlCompilerException>,
    Vec<CqlCompilerException>,
    Vec<CqlCompilerException>,
) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut messages = Vec::new();

    for err in exceptions {
        let exception = err.clone();

        match exception.severity() {
            Severity::Error => errors.push(exception),
            Severity::Warning => warnings.push(exception),
            Severity::Info => messages.push(exception),
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
            assert!(has_location, "event line missing [line:col] prefix: {result}");
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
}
