//! Error types and utilities for the workspace.
//!
//! This module provides the foundation error type that can be extended
//! by domain-specific error types in other crates.

use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Foundation error type providing common error variants.
///
/// This enum covers the most common error cases across the workspace.
/// Domain-specific crates can extend this by wrapping it in their own
/// error types.
///
/// # Example
/// ```
/// use rh_foundation::{FoundationError, ErrorContext};
///
/// fn example() -> rh_foundation::Result<()> {
///     std::fs::read_to_string("config.json")
///         .context("Failed to read config file")?;
///     Ok(())
/// }
/// ```
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

    /// HTTP request error (available with `http` feature)
    #[error("HTTP error: {0}")]
    Http(String),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParse(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
}

/// Result type alias using FoundationError
pub type Result<T> = std::result::Result<T, FoundationError>;

/// Trait for adding context to errors.
///
/// This trait allows chaining contextual information to errors,
/// making it easier to track the source and cause of errors.
///
/// # Example
/// ```
/// use rh_foundation::ErrorContext;
///
/// fn read_config() -> rh_foundation::Result<String> {
///     std::fs::read_to_string("config.json")
///         .context("Failed to read configuration file")?;
///     Ok("config".to_string())
/// }
/// ```
pub trait ErrorContext<T, E> {
    /// Add context to an error
    fn context(self, context: impl fmt::Display) -> Result<T>;

    /// Add context with a function (lazy evaluation)
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, context: impl fmt::Display) -> Result<T> {
        self.map_err(|e| FoundationError::Other(anyhow::Error::new(e).context(context.to_string())))
    }

    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| FoundationError::Other(anyhow::Error::new(e).context(f())))
    }
}

/// Error with additional metadata for better debugging and error tracking.
///
/// This structure wraps any error and allows attaching key-value metadata
/// that can be useful for logging, debugging, or displaying detailed error
/// information to users.
#[derive(Debug)]
pub struct ErrorWithMetadata {
    /// The underlying error
    pub error: Box<dyn std::error::Error + Send + Sync>,
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

impl ErrorWithMetadata {
    /// Create a new error with metadata
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            error: Box::new(error),
            metadata: HashMap::new(),
        }
    }

    /// Add a metadata entry
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

impl fmt::Display for ErrorWithMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        if !self.metadata.is_empty() {
            write!(f, " [")?;
            let mut first = true;
            for (key, value) in &self.metadata {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{key}={value}")?;
                first = false;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorWithMetadata {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_with_metadata() {
        let err = ErrorWithMetadata::new(FoundationError::InvalidInput("test".to_string()))
            .with_metadata("file", "config.json")
            .with_metadata("line", "42");

        assert_eq!(err.get_metadata("file"), Some("config.json"));
        assert_eq!(err.get_metadata("line"), Some("42"));
        assert_eq!(err.get_metadata("missing"), None);

        let display = format!("{err}");
        assert!(display.contains("Invalid input"));
        assert!(display.contains("file=config.json"));
        assert!(display.contains("line=42"));
    }

    #[test]
    fn test_error_context() {
        let result: std::result::Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));

        let with_context = result.context("Reading configuration");
        assert!(with_context.is_err());

        let err_msg = format!("{}", with_context.unwrap_err());
        assert!(err_msg.contains("Reading configuration"));
    }
}
