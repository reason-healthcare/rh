use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("Base structure definition not found: {0}")]
    BaseNotFound(String),

    #[error("Invalid structure definition: {0}")]
    InvalidStructureDefinition(String),

    #[error("Differential merge error: {0}")]
    MergeError(String),

    #[error("Element path error: {0}")]
    PathError(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type SnapshotResult<T> = Result<T, SnapshotError>;
