//! # rh-cql
//!
//! CQL (Clinical Quality Language) capabilities for the RH ecosystem.
//!
//! This crate provides:
//! - CQL-to-ELM translation
//! - CQL/ELM execution against FHIR data
//!
//! ## Status
//!
//! 🚧 Under development - API is not yet stable.

pub mod elm;
pub mod error;

pub use error::{CqlError, Result};
