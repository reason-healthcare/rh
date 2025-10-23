//! Error types for VCL parsing

use rh_foundation::FoundationError;
use thiserror::Error;

/// Errors that can occur during VCL parsing
#[derive(Error, Debug)]
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

    /// Foundation error (covers IO, JSON, etc.)
    #[error(transparent)]
    Foundation(#[from] FoundationError),

    /// Generic error with context
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Clone for VclError {
    fn clone(&self) -> Self {
        match self {
            Self::ParseError {
                message,
                position,
                input,
            } => Self::ParseError {
                message: message.clone(),
                position: *position,
                input: input.clone(),
            },
            Self::UnexpectedEof { position } => Self::UnexpectedEof {
                position: *position,
            },
            Self::InvalidToken { token, position } => Self::InvalidToken {
                token: token.clone(),
                position: *position,
            },
            Self::InvalidUri { uri } => Self::InvalidUri { uri: uri.clone() },
            Self::InvalidQuotedString { message, position } => Self::InvalidQuotedString {
                message: message.clone(),
                position: *position,
            },
            Self::NestingTooDeep { max_depth } => Self::NestingTooDeep {
                max_depth: *max_depth,
            },
            Self::TranslationError { message } => Self::TranslationError {
                message: message.clone(),
            },
            Self::Foundation(e) => Self::Other(anyhow::anyhow!("{e}")),
            Self::Other(e) => Self::Other(anyhow::anyhow!("{e}")),
        }
    }
}

impl PartialEq for VclError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::ParseError {
                    message: m1,
                    position: p1,
                    input: i1,
                },
                Self::ParseError {
                    message: m2,
                    position: p2,
                    input: i2,
                },
            ) => m1 == m2 && p1 == p2 && i1 == i2,
            (Self::UnexpectedEof { position: p1 }, Self::UnexpectedEof { position: p2 }) => {
                p1 == p2
            }
            (
                Self::InvalidToken {
                    token: t1,
                    position: p1,
                },
                Self::InvalidToken {
                    token: t2,
                    position: p2,
                },
            ) => t1 == t2 && p1 == p2,
            (Self::InvalidUri { uri: u1 }, Self::InvalidUri { uri: u2 }) => u1 == u2,
            (
                Self::InvalidQuotedString {
                    message: m1,
                    position: p1,
                },
                Self::InvalidQuotedString {
                    message: m2,
                    position: p2,
                },
            ) => m1 == m2 && p1 == p2,
            (Self::NestingTooDeep { max_depth: d1 }, Self::NestingTooDeep { max_depth: d2 }) => {
                d1 == d2
            }
            (Self::TranslationError { message: m1 }, Self::TranslationError { message: m2 }) => {
                m1 == m2
            }
            _ => false,
        }
    }
}

impl Eq for VclError {}

impl From<std::io::Error> for VclError {
    fn from(err: std::io::Error) -> Self {
        VclError::Foundation(FoundationError::Io(err))
    }
}

impl From<serde_json::Error> for VclError {
    fn from(err: serde_json::Error) -> Self {
        VclError::Foundation(FoundationError::Serialization(err))
    }
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
