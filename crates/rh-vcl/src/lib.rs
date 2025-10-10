//! # rh-vcl - ValueSet Compose Language Parser
//!
//! This crate provides a parser for the ValueSet Compose Language (VCL),
//! a domain-specific language for expressing ValueSet CLDs in a compact syntax.
//!
//! VCL is inspired by SNOMED CT's Expression Constraint Language (ECL) and enables
//! a new family of implicit ValueSet URIs that are usable across all code systems.
//!
//! ## Example
//!
//! ```rust,ignore
//! use rh_vcl::{parse_vcl, VclExpression};
//!
//! let vcl_str = "(http://hl7.org/fhir/sid/icd-10){Z51.1*}";
//! let expr = parse_vcl(vcl_str)?;
//! ```

pub mod ast;
pub mod error;
pub mod fhir;
pub mod parser;
pub mod translator;

pub use ast::{Code, VclExpression, SimpleExpression, Filter, Operation};
pub use error::VclError;
pub use fhir::{ValueSetCompose, ValueSetInclude, ValueSetFilter};
pub use parser::parse_vcl;
pub use translator::{VclTranslator, translate_vcl_to_fhir, translate_vcl_string_to_fhir};

/// Re-export commonly used types
pub type VclResult<T> = Result<T, VclError>;
