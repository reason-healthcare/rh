use thiserror::Error;

use crate::FoundationError;

/// Errors that can occur during package loading operations.
///
/// This error type extends FoundationError with domain-specific
/// error variants for package loading and management.
#[derive(Error, Debug)]
pub enum LoaderError {
    /// Package not found in registry.
    #[error("Package not found: {package}@{version}")]
    PackageNotFound { package: String, version: String },

    /// Invalid package manifest structure or content.
    #[error("Invalid package manifest: {message}")]
    InvalidManifest { message: String },

    /// Archive extraction failed.
    #[error("Archive extraction failed: {message}")]
    ArchiveError { message: String },

    /// Package already exists at target location.
    #[error(
        "Package already exists: {package}@{version} at {path}. Use --overwrite to replace it."
    )]
    PackageExists {
        package: String,
        version: String,
        path: String,
    },

    /// Foundation error (covers IO, JSON, HTTP, URL parsing, Authentication).
    #[error(transparent)]
    Foundation(#[from] FoundationError),

    /// URL parsing error.
    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),
}

impl From<std::io::Error> for LoaderError {
    fn from(err: std::io::Error) -> Self {
        Self::Foundation(FoundationError::Io(err))
    }
}

impl From<serde_json::Error> for LoaderError {
    fn from(err: serde_json::Error) -> Self {
        Self::Foundation(FoundationError::Serialization(err))
    }
}

/// Result type for loader operations.
pub type LoaderResult<T> = std::result::Result<T, LoaderError>;
