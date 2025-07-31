//! Error types for FHIRPath parsing and evaluation

use thiserror::Error;

/// Result type for FHIRPath operations
pub type FhirPathResult<T> = Result<T, FhirPathError>;

/// Errors that can occur during FHIRPath parsing and evaluation
#[derive(Error, Debug)]
pub enum FhirPathError {
    /// Syntax error in the FHIRPath expression
    #[error("Syntax error in FHIRPath expression at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Type error during evaluation
    #[error("Type error: {message}")]
    TypeError { message: String },

    /// Function not found or invalid function call
    #[error("Function error: {message}")]
    FunctionError { message: String },

    /// Invalid operation on the given data types
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    /// Resource navigation error
    #[error("Navigation error: {message}")]
    NavigationError { message: String },

    /// General evaluation error
    #[error("Evaluation error: {message}")]
    EvaluationError { message: String },

    /// IO or parsing error
    #[error("Parse error: {0}")]
    ParseError(#[from] anyhow::Error),
}
