//! Error types for VCL parsing

use thiserror::Error;

/// Errors that can occur during VCL parsing
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum VclError {
    /// Parse error with context
    #[error("Parse error at position {position}: {message}")]
    ParseError {
        message: String,
        position: usize,
        input: String,
    },

    /// Unexpected end of input
    #[error("Unexpected end of input at position {position}")]
    UnexpectedEof { position: usize },

    /// Invalid character or token
    #[error("Invalid token '{token}' at position {position}")]
    InvalidToken { token: String, position: usize },

    /// Invalid URI format
    #[error("Invalid URI format: {uri}")]
    InvalidUri { uri: String },

    /// Invalid quoted string
    #[error("Invalid quoted string at position {position}: {message}")]
    InvalidQuotedString { message: String, position: usize },

    /// Nested expression too deep
    #[error("Expression nesting too deep (max depth: {max_depth})")]
    NestingTooDeep { max_depth: usize },

    /// Translation error - expression cannot be translated to FHIR
    #[error("Translation error: {message}")]
    TranslationError { message: String },
}

impl VclError {
    /// Create a new parse error
    pub fn parse_error(
        message: impl Into<String>,
        position: usize,
        input: impl Into<String>,
    ) -> Self {
        Self::ParseError {
            message: message.into(),
            position,
            input: input.into(),
        }
    }

    /// Create a new invalid token error
    pub fn invalid_token(token: impl Into<String>, position: usize) -> Self {
        Self::InvalidToken {
            token: token.into(),
            position,
        }
    }

    /// Create a new invalid URI error
    pub fn invalid_uri(uri: impl Into<String>) -> Self {
        Self::InvalidUri { uri: uri.into() }
    }

    /// Create a new invalid quoted string error
    pub fn invalid_quoted_string(message: impl Into<String>, position: usize) -> Self {
        Self::InvalidQuotedString {
            message: message.into(),
            position,
        }
    }
}

/// Convenient type alias for Results in this crate
pub type VclResult<T> = Result<T, VclError>;
