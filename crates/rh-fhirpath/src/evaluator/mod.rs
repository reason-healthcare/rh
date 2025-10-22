//! FHIRPath expression evaluator module
//!
//! This module is organized into several logical groups:
//! - `core`: Core evaluation engine and context management
//! - `operations`: All operation evaluators (arithmetic, comparison, collection, etc.)
//! - `types`: Type system and value types
//! - `functions`: Built-in function registry and implementations
//! - `metadata`: FHIR type metadata integration

pub mod core;
pub mod functions;
pub mod metadata;
pub mod operations;
pub mod types;

// Re-export the main types for public API
pub use core::{EvaluationContext, FhirPathEvaluator};
pub use functions::FunctionRegistry;
pub use types::FhirPathValue;

// Re-export the operation evaluators for testing and advanced usage
pub use operations::{
    ArithmeticEvaluator, CollectionEvaluator, ComparisonEvaluator, DateTimeEvaluator,
    MathEvaluator, StringEvaluator, UnitEvaluator,
};
pub use types::operations::TypeEvaluator;
