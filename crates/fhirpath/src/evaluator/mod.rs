//! FHIRPath expression evaluator module

mod values;
mod context;
mod arithmetic;
mod comparison;
mod collection;
mod functions;
mod strings;
mod core;

// Re-export the main types
pub use values::FhirPathValue;
pub use context::EvaluationContext;
pub use core::FhirPathEvaluator;

// Re-export the evaluator modules for testing and advanced usage
pub use arithmetic::ArithmeticEvaluator;
pub use comparison::ComparisonEvaluator;
pub use collection::CollectionEvaluator;
pub use functions::FunctionRegistry;
pub use strings::StringEvaluator;
