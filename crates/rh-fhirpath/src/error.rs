//! Error types for FHIRPath parsing and evaluation.
//!
//! This module provides comprehensive error types for all FHIRPath
//! operations including parsing, type checking, and evaluation.

use rh_foundation::FoundationError;
use thiserror::Error;

/// Result type for FHIRPath operations
pub type FhirPathResult<T> = Result<T, FhirPathError>;

/// Errors that can occur during FHIRPath parsing and evaluation.
///
/// These errors are domain-specific to FHIRPath and provide detailed
/// context about what went wrong during parsing or evaluation.
#[derive(Error, Debug)]
pub enum FhirPathError {
    /// Syntax error in the FHIRPath expression with location
    #[error("Syntax error in FHIRPath expression at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Type error during evaluation (e.g., wrong type for operation)
    #[error("Type error: {message}")]
    TypeError { message: String },

    /// Function not found or invalid function call
    #[error("Function error: {message}")]
    FunctionError { message: String },

    /// Invalid operation on the given data types
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    /// Resource navigation error (e.g., invalid path)
    #[error("Navigation error: {message}")]
    NavigationError { message: String },

    /// General evaluation error
    #[error("Evaluation error: {message}")]
    EvaluationError { message: String },

    /// Parsing error with context
    #[error("Parse error: {0}")]
    ParseError(#[from] anyhow::Error),

    /// Foundation error (covers IO, JSON, etc.)
    #[error(transparent)]
    Foundation(#[from] FoundationError),
}

impl FhirPathError {
    /// Create a syntax error at a specific location
    pub fn syntax_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SyntaxError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        Self::TypeError {
            message: message.into(),
        }
    }

    /// Create a function error
    pub fn function_error(message: impl Into<String>) -> Self {
        Self::FunctionError {
            message: message.into(),
        }
    }

    /// Create an invalid operation error
    pub fn invalid_operation(message: impl Into<String>) -> Self {
        Self::InvalidOperation {
            message: message.into(),
        }
    }

    /// Create a navigation error
    pub fn navigation_error(message: impl Into<String>) -> Self {
        Self::NavigationError {
            message: message.into(),
        }
    }

    /// Create an evaluation error
    pub fn evaluation_error(message: impl Into<String>) -> Self {
        Self::EvaluationError {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for FhirPathError {
    fn from(err: std::io::Error) -> Self {
        FhirPathError::Foundation(FoundationError::Io(err))
    }
}

impl From<serde_json::Error> for FhirPathError {
    fn from(err: serde_json::Error) -> Self {
        FhirPathError::Foundation(FoundationError::Serialization(err))
    }
}
