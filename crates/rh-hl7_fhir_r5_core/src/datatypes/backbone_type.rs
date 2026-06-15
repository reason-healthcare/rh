use crate::datatypes::data_type::DataType;
use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// BackboneType
///
/// BackboneType Type: Base definition for the few data types that are allowed to carry modifier extensions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BackboneType
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: BackboneType
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackboneType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<Extension>,
}

impl Default for BackboneType {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            modifier_extension: Default::default(),
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
            rh_foundation::ElementCardinality::new("BackboneType.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("BackboneType.extension", 0, None),
            rh_foundation::ElementCardinality::new("BackboneType.modifierExtension", 0, None),
        ]
    });

impl crate::validation::ValidatableResource for BackboneType {
    fn resource_type(&self) -> &'static str {
        "BackboneType"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }
}
