//! RH FHIR Validator
//!
//! High-performance FHIR resource validation library providing:
//! - Structural validation via Rust's type system
//! - FHIRPath invariant evaluation
//! - Parallel batch validation
//! - Profile-based validation
//!
//! See [DESIGN.md](../DESIGN.md) for architecture details.

pub mod types;
pub mod validator;

pub use types::{
    Invariant, IssueCode, Severity, ValidationIssue, ValidationResult, ValidatorError,
};
pub use validator::{FhirValidator, JsonValidator, ValidatorConfig};

/// Result type for validator operations
pub type Result<T> = std::result::Result<T, crate::types::ValidatorError>;
