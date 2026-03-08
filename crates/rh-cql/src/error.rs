//! Error types for CQL operations.

use thiserror::Error;

/// CQL-specific errors.
#[derive(Debug, Error)]
pub enum CqlError {
    /// Error parsing CQL source.
    #[error("CQL parse error: {0}")]
    Parse(String),

    /// Syntax error at a specific location.
    #[error("Syntax error at line {line}, column {column}: {message}")]
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },

    /// Unexpected token during parsing.
    #[error("Unexpected token '{found}' at line {line}, column {column}, expected {expected}")]
    UnexpectedToken {
        found: String,
        expected: String,
        line: usize,
        column: usize,
    },

    /// Unexpected end of input.
    #[error("Unexpected end of input at line {line}, column {column}")]
    UnexpectedEof { line: usize, column: usize },

    /// Error translating CQL to ELM.
    #[error("CQL translation error: {0}")]
    Translation(String),

    /// Error during ELM execution.
    #[error("CQL execution error: {0}")]
    Execution(String),

    /// Library not found.
    #[error("Library not found: {0}")]
    LibraryNotFound(String),

    /// Type error during evaluation.
    #[error("Type error: {0}")]
    TypeError(String),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type for CQL operations.
pub type Result<T> = std::result::Result<T, CqlError>;
