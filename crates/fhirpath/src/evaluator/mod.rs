//! FHIRPath expression evaluator module

mod arithmetic;
mod collection;
mod comparison;
mod context;
mod core;
mod functions;
mod strings;
mod values;

// Re-export the main types
pub use context::EvaluationContext;
pub use core::FhirPathEvaluator;
pub use values::FhirPathValue;

// Re-export the evaluator modules for testing and advanced usage
pub use arithmetic::ArithmeticEvaluator;
pub use collection::CollectionEvaluator;
pub use comparison::ComparisonEvaluator;
pub use functions::FunctionRegistry;
pub use strings::StringEvaluator;
