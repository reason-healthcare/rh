//! Core evaluation engine for FHIRPath expressions
//!
//! This module contains:
//! - Main FhirPathEvaluator for expression evaluation
//! - EvaluationContext for managing evaluation state and variables

pub mod context;
pub mod evaluator;

// Re-export the main types
pub use context::{EvaluationContext, TraceLog};
pub use evaluator::FhirPathEvaluator;
