//! FHIRPath expression evaluator
//!
//! This module provides the core evaluation logic for FHIRPath expressions.
//! The functionality has been refactored into focused sub-modules for maintainability:
//!
//! - `values`: FhirPathValue enum and basic value operations
//! - `context`: EvaluationContext management
//! - `arithmetic`: Arithmetic operations (+, -, *, /, etc.)
//! - `comparison`: Comparison and equality operations
//! - `collection`: Collection operations and union logic
//! - `functions`: Built-in function registry and evaluation
//! - `core`: Main FhirPathEvaluator coordination logic

mod arithmetic;
mod collection;
mod comparison;
mod context;
mod core;
mod functions;
mod values;

// Re-export the main types for backward compatibility
pub use context::EvaluationContext;
pub use core::FhirPathEvaluator;
pub use values::FhirPathValue;

// Re-export the evaluator modules for testing and advanced usage
pub use arithmetic::ArithmeticEvaluator;
pub use collection::CollectionEvaluator;
pub use comparison::ComparisonEvaluator;
pub use functions::FunctionRegistry;
