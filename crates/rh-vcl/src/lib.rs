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
pub mod parser;

pub use ast::*;
pub use error::*;
pub use parser::parse_vcl;

/// Re-export commonly used types
pub type VclResult<T> = Result<T, VclError>;
