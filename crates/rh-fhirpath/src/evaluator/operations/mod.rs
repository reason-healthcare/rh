//! Operation evaluators for FHIRPath expressions
//!
//! This module contains evaluators for different types of operations:
//! - Arithmetic operations (addition, subtraction, multiplication, etc.)
//! - Comparison operations (equality, inequality, less than, etc.)
//! - Collection operations (union, intersect, exclude, etc.)
//! - Mathematical functions (abs, sqrt, ceiling, floor, etc.)
//! - String operations (length, upper, lower, substring, etc.)
//! - Date/time operations (now, today, component extraction, etc.)
//! - Unit operations (quantity calculations and conversions)

pub mod arithmetic;
pub mod collection;
pub mod comparison;
pub mod datetime;
pub mod math;
pub mod strings;
pub mod units;

// Re-export the main evaluator types
pub use arithmetic::ArithmeticEvaluator;
pub use collection::CollectionEvaluator;
pub use comparison::ComparisonEvaluator;
pub use datetime::DateTimeEvaluator;
pub use math::MathEvaluator;
pub use strings::StringEvaluator;
pub use units::UnitEvaluator;
