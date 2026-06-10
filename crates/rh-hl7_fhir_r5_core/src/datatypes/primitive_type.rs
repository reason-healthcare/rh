use crate::datatypes::data_type::DataType;
use serde::{Deserialize, Serialize};
/// PrimitiveType
///
/// PrimitiveType Type: The base type for all re-useable types defined that have a simple property.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PrimitiveType
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: PrimitiveType
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
}

impl Default for PrimitiveType {
    fn default() -> Self {
        Self {
            base: DataType::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("PrimitiveType.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PrimitiveType.extension", 0, None),
        ]
    });

impl crate::validation::ValidatableResource for PrimitiveType {
    fn resource_type(&self) -> &'static str {
        "PrimitiveType"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }
}
