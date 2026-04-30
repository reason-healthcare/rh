use thiserror::Error;

/// Result type for publisher operations.
pub type Result<T> = std::result::Result<T, PublisherError>;

/// Errors produced by the FHIR package publisher.
#[derive(Error, Debug)]
pub enum PublisherError {
    /// A required file is missing from the source directory.
    #[error("Missing required file: {0}")]
    MissingFile(String),

    /// `ImplementationGuide.json` fields are inconsistent with `package.json`.
    #[error("ImplementationGuide sync error: {0}")]
    IgSync(String),

    /// A lifecycle hook processor returned an error.
    #[error("Hook processor '{processor}' failed: {message}")]
    HookProcessor { processor: String, message: String },

    /// A processor name declared in `packager.toml` is not registered.
    #[error("Unknown hook processor: '{0}'")]
    UnknownProcessor(String),

    /// A dependency package required by the build is not present locally.
    #[error("Missing dependency package: {0}")]
    MissingPackage(String),

    /// FHIR validation produced ERROR-severity issues.
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// CQL compilation or validation error.
    #[error("CQL error: {0}")]
    Cql(String),

    /// Tarball creation or extraction error.
    #[error("Archive error: {0}")]
    Archive(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing error.
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Catch-all for other errors.
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}
