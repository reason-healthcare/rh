//! RH FHIR Validator
//!
//! High-performance FHIR resource validation library providing:
//! - Structural validation via Rust's type system
//! - FHIRPath invariant evaluation
//! - Parallel batch validation
//! - Profile-based validation
//!
//! See [DESIGN.md](../DESIGN.md) for architecture details.

pub mod dispatch;
pub mod types;
pub mod validator;
pub mod valuesets;

pub use dispatch::{extract_resource_type, suggest_resource_type};
pub use types::{
    Invariant, IssueCode, Severity, ValidationIssue, ValidationResult, ValidatorError,
};
pub use validator::{FhirValidator, JsonValidator, ValidatorConfig};
pub use valuesets::{ValueSetExpansion, ValueSetRegistry};

/// Result type for validator operations
pub type Result<T> = std::result::Result<T, crate::types::ValidatorError>;
