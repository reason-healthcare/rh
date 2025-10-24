//! Validation support for FHIR resources
//!
//! This module provides traits for validating FHIR resources using invariants.

/// Trait for FHIR types that can be validated using invariants
///
/// This trait provides access to the invariants (constraints) defined
/// in the FHIR specification for this resource or datatype.
pub trait ValidatableResource {
    /// Returns the FHIR resource type name
    fn resource_type(&self) -> &'static str;

    /// Returns the invariants (constraints) for this resource/datatype
    ///
    /// These are the FHIRPath expressions that must evaluate to true
    /// for the resource to be considered valid.
    fn invariants() -> &'static [rh_foundation::Invariant];

    /// Returns the profile URL if this is a profile, None otherwise
    fn profile_url() -> Option<&'static str> {
        None
    }
}
