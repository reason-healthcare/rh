//! Clinical Practice Guidelines (CPG) support for FHIR resources.

pub mod error;
pub mod resolver;

pub mod apply;

pub mod context;
pub mod expression;
pub mod fhir_to_cql;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
