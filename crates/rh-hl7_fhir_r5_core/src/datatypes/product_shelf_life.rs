use crate::datatypes::backbone_type::BackboneType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ProductShelfLife
///
/// ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ProductShelfLife
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: ProductShelfLife
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductShelfLife {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneType,
    /// This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used (Duration)
    #[serde(rename = "periodDuration")]
    pub period_duration: Option<Duration>,
    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used (string)
    #[serde(rename = "periodString")]
    pub period_string: Option<StringType>,
    /// Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "specialPrecautionsForStorage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_precautions_for_storage: Vec<CodeableConcept>,
}

impl Default for ProductShelfLife {
    fn default() -> Self {
        Self {
            base: BackboneType::default(),
            type_: Default::default(),
            period_duration: Default::default(),
            period_string: Default::default(),
            special_precautions_for_storage: Default::default(),
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
            rh_foundation::ElementCardinality::new("ProductShelfLife.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProductShelfLife.extension", 0, None),
            rh_foundation::ElementCardinality::new("ProductShelfLife.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ProductShelfLife.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProductShelfLife.period[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ProductShelfLife.specialPrecautionsForStorage",
                0,
                None,
            ),
        ]
    });

impl crate::validation::ValidatableResource for ProductShelfLife {
    fn resource_type(&self) -> &'static str {
        "ProductShelfLife"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ProductShelfLife")
    }
}
