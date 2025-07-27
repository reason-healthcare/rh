//! Type system and value types for FHIRPath expressions
//!
//! This module contains:
//! - FhirPathValue enum representing all possible FHIRPath values
//! - Type operations (is, as, ofType for type checking and conversion)

pub mod operations;
pub mod values;

// Re-export the main types
pub use operations::TypeEvaluator;
pub use values::FhirPathValue;
