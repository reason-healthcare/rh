use serde::{Deserialize, Serialize};
/// Base
///
/// Base Type: Base definition for all types defined in FHIR type system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Base
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {}

impl Default for Base {
    fn default() -> Self {
        Self {}
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::Invariant::new(
            "ele-1",
            rh_foundation::Severity::Error,
            "All FHIR elements must have a @value or children",
            "hasValue() or (children().count() > id.count())",
        )]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(Vec::new);

impl crate::validation::ValidatableResource for Base {
    fn resource_type(&self) -> &'static str {
        "Base"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }
}
