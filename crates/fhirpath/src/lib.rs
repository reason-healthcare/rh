//! # FHIRPath Parser and Evaluator
//!
//! This crate provides functionality for parsing and evaluating FHIRPath expressions
//! against FHIR resources. FHIRPath is a path-based navigation and extraction language
//! that can be used to navigate and extract elements from FHIR resources.
//!
//! ## Example
//!
//! ```rust
//! use fhirpath::{FhirPathParser, FhirPathExpression};
//!
//! let parser = FhirPathParser::new();
//! let expression = parser.parse("Patient.name.given").unwrap();
//! // Use the expression to evaluate against FHIR resources
//! ```

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod parser;

pub use ast::*;
pub use error::*;
pub use evaluator::*;
pub use parser::*;
