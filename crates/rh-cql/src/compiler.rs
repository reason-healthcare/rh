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
    let options = options.unwrap_or_default();

    // Parse the CQL source
    let parser = CqlParser::new();
    let ast = parser
        .parse(source)
        .map_err(|e| CompilationError::Parse(e.to_string()))?;

    // Create the builder and translate
    let mut builder = LibraryBuilder::new();
    builder.set_options(options.clone());

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

/// Validate CQL source code without producing ELM output.
///
/// This function parses and performs semantic analysis on the CQL source
/// but only returns validation results, not the full ELM library.
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
    let mut options = options.unwrap_or_default();
    options.verify_only = true;

    let result = compile(source, Some(options))?;

    Ok(ValidationResult {
        is_valid: result.errors.is_empty(),
        errors: result.errors,
        warnings: result.warnings,
        messages: result.messages,
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

/// Categorize exceptions by severity.
fn categorize_exceptions(
    exceptions: &[crate::builder::BuilderError],
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
        let exception = CqlCompilerException::semantic(err.to_string());

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
