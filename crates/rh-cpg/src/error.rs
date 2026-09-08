use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CpgError {
    #[error("invalid resource: {0}")]
    InvalidResource(String),
    #[error("canonical not found: {0}")]
    CanonicalNotFound(String),
    #[error("reference not found: {0}")]
    ReferenceNotFound(String),
    #[error("expression error: {0}")]
    ExpressionError(String),
    #[error("unsupported expression language: {0}")]
    UnsupportedExpressionLanguage(String),
    #[error("evaluation error: {0}")]
    EvaluationError(String),
    #[error(transparent)]
    Json(#[from] JsonError),
    #[error("CQL evaluation error: {0}")]
    CqlEval(String),
}

pub type CpgResult<T> = Result<T, CpgError>;
