//! # rh-cql
//!
//! CQL (Clinical Quality Language) capabilities for the RH ecosystem.
//!
//! This crate provides:
//! - CQL-to-ELM translation
//! - CQL/ELM execution against FHIR data
//! - ModelInfo type definitions for data model resolution
//!
//! ## Status
//!
//! 🚧 Under development - API is not yet stable.
//!
//! ## Modules
//!
//! - [`elm`]: ELM (Expression Logical Model) type definitions
//! - [`modelinfo`]: ModelInfo type definitions for CQL data model resolution
//! - [`provider`]: Model information providers (in-memory, WASM-compatible)
//! - [`datatype`]: Internal DataType system for type checking

pub mod datatype;
pub mod elm;
pub mod error;
pub mod modelinfo;
pub mod provider;

pub use datatype::{DataType, SystemType, TupleElement};
pub use error::{CqlError, Result};
pub use provider::{
    fhir_r4_model_info, fhir_r4_provider, MemoryModelInfoProvider, ModelInfoProvider,
};
