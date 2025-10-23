//! Validation error types.
//!
//! This module provides the unified error type for the validator crate,
//! extending FoundationError with domain-specific validation errors.

use rh_foundation::FoundationError;
use thiserror::Error;

/// Errors that can occur during FHIR validation operations.
///
/// This error type extends FoundationError with domain-specific
/// error variants for validation, package management, and code generation.
#[derive(Error, Debug)]
pub enum ValidatorError {
    /// JSON syntax error with location
    #[error("JSON syntax error: {message} at line {line}, column {column}")]
    JsonSyntax {
        message: String,
        line: usize,
        column: usize,
    },

    /// Schema validation error
    #[error("Schema validation error: {message}")]
    Schema { message: String },

    /// FHIR resource validation error
    #[error("FHIR resource validation failed: {message}")]
    ResourceValidation { message: String },

    /// Invalid FHIR version
    #[error("Invalid FHIR version: {version}")]
    InvalidVersion { version: String },

    /// Package not found
    #[error("Package not found: {package}")]
    PackageNotFound { package: String },

    /// Package download failed
    #[error("Failed to download package: {message}")]
    PackageDownload { message: String },

    /// Code generation failed
    #[error("Failed to generate Rust types: {message}")]
    CodegenFailed { message: String },

    /// Code generation error
    #[error(transparent)]
    Codegen(#[from] rh_codegen::CodegenError),

    /// Package loader error
    #[error(transparent)]
    Loader(#[from] rh_loader::LoaderError),

    /// Foundation error (covers IO, JSON, etc.)
    #[error(transparent)]
    Foundation(#[from] FoundationError),

    /// Generic error with context
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<std::io::Error> for ValidatorError {
    fn from(err: std::io::Error) -> Self {
        ValidatorError::Foundation(FoundationError::Io(err))
    }
}

impl From<serde_json::Error> for ValidatorError {
    fn from(err: serde_json::Error) -> Self {
        ValidatorError::Foundation(FoundationError::Serialization(err))
    }
}

impl Clone for ValidatorError {
    fn clone(&self) -> Self {
        match self {
            Self::JsonSyntax {
                message,
                line,
                column,
            } => Self::JsonSyntax {
                message: message.clone(),
                line: *line,
                column: *column,
            },
            Self::Schema { message } => Self::Schema {
                message: message.clone(),
            },
            Self::ResourceValidation { message } => Self::ResourceValidation {
                message: message.clone(),
            },
            Self::InvalidVersion { version } => Self::InvalidVersion {
                version: version.clone(),
            },
            Self::PackageNotFound { package } => Self::PackageNotFound {
                package: package.clone(),
            },
            Self::PackageDownload { message } => Self::PackageDownload {
                message: message.clone(),
            },
            Self::CodegenFailed { message } => Self::CodegenFailed {
                message: message.clone(),
            },
            Self::Codegen(e) => Self::CodegenFailed {
                message: e.to_string(),
            },
            Self::Loader(e) => Self::PackageDownload {
                message: e.to_string(),
            },
            Self::Foundation(e) => Self::Other(anyhow::anyhow!("{e}")),
            Self::Other(e) => Self::Other(anyhow::anyhow!("{e}")),
        }
    }
}

impl PartialEq for ValidatorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::JsonSyntax {
                    message: m1,
                    line: l1,
                    column: c1,
                },
                Self::JsonSyntax {
                    message: m2,
                    line: l2,
                    column: c2,
                },
            ) => m1 == m2 && l1 == l2 && c1 == c2,
            (Self::Schema { message: m1 }, Self::Schema { message: m2 }) => m1 == m2,
            (
                Self::ResourceValidation { message: m1 },
                Self::ResourceValidation { message: m2 },
            ) => m1 == m2,
            (Self::InvalidVersion { version: v1 }, Self::InvalidVersion { version: v2 }) => {
                v1 == v2
            }
            (Self::PackageNotFound { package: p1 }, Self::PackageNotFound { package: p2 }) => {
                p1 == p2
            }
            (Self::PackageDownload { message: m1 }, Self::PackageDownload { message: m2 }) => {
                m1 == m2
            }
            (Self::CodegenFailed { message: m1 }, Self::CodegenFailed { message: m2 }) => m1 == m2,
            _ => false,
        }
    }
}

/// Result type for validator operations
pub type Result<T> = std::result::Result<T, ValidatorError>;
