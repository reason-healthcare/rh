//! # FHIRPath Parser and Evaluator
//!
//! This crate provides functionality for parsing and evaluating FHIRPath expressions
//! against FHIR resources. FHIRPath is a path-based navigation and extraction language
//! that can be used to navigate and extract elements from FHIR resources.
//!
//! ## Example
//!
//! ```rust
//! use fhirpath::{FhirPathParser, FhirPathEvaluator, EvaluationContext};
//! use serde_json::json;
//!
//! let parser = FhirPathParser::new();
//! let expression = parser.parse("Patient.name.given").unwrap();
//!
//! let evaluator = FhirPathEvaluator::new();
//! let context = EvaluationContext::new(json!({"name": [{"given": ["John"]}]}));
//! let result = evaluator.evaluate(&expression, &context).unwrap();
//! ```

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod parser;

// Re-export the main public types users need
pub use ast::{
    AdditiveOperator, EqualityOperator, Expression, FhirPathExpression, InequalityOperator,
    Invocation, Literal, MembershipOperator, MultiplicativeOperator, PolarityOperator, Term,
    TypeSpecifier,
};
pub use error::{FhirPathError, FhirPathResult};
pub use evaluator::{EvaluationContext, FhirPathEvaluator, FhirPathValue};
pub use parser::FhirPathParser;
