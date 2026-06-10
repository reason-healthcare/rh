//! Prelude module - commonly used traits for convenience
//!
//! This module re-exports the most commonly used traits for working with
//! FHIR resources. Import this module to avoid having to import individual
//! traits from the `traits` module.
//!
//! # Example
//!
//! ```ignore
//! use hl7_fhir_r4_core::prelude::*;
//! use hl7_fhir_r4_core::resources::patient::Patient;
//!
//! // All mutator traits are now in scope
//! let patient = <Patient as PatientMutators>::new()
//!     .set_id("example".to_string())
//!     .set_active(true);
//! ```

// Resource mutator traits - for building resources with method chaining
pub use crate::traits::domain_resource::DomainResourceMutators;
pub use crate::traits::resource::ResourceMutators;

// Note: Individual resource mutator traits (PatientMutators, ObservationMutators, etc.)
// are re-exported from their respective resource modules for convenience.
// For example: use hl7_fhir_r4_core::resources::patient::PatientMutators;

// Validation trait
pub use crate::validation::ValidatableResource;
