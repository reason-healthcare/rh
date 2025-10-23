//! Error types and utilities for the workspace.
//!
//! This module provides the foundation error type that can be extended
//! by domain-specific error types in other crates.

use std::fmt;
use thiserror::Error;

/// Foundation error type providing common error variants.
///
/// This enum covers the most common error cases across the workspace.
/// Domain-specific crates can extend this by wrapping it in their own
/// error types.
#[derive(Error, Debug)]
pub enum FoundationError {
    /// Configuration error with a descriptive message
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Generic error with context
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),

    /// Invalid input with descriptive message
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Parsing error
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Result type alias using FoundationError
pub type Result<T> = std::result::Result<T, FoundationError>;

/// Trait for adding context to errors
pub trait ErrorContext<T, E> {
    /// Add context to an error
    fn context(self, context: impl fmt::Display) -> Result<T>;
}

impl<T, E> ErrorContext<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, context: impl fmt::Display) -> Result<T> {
        self.map_err(|e| FoundationError::Other(anyhow::Error::new(e).context(context.to_string())))
    }
}
