use crate::datatypes::data_type::DataType;
use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// RatioRange
///
/// RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RatioRange
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: RatioRange
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatioRange {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Low Numerator limit
    #[serde(rename = "lowNumerator")]
    pub low_numerator: Option<Quantity>,
    /// High Numerator limit
    #[serde(rename = "highNumerator")]
    pub high_numerator: Option<Quantity>,
    /// Denominator value
    pub denominator: Option<Quantity>,
}

impl Default for RatioRange {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            low_numerator: Default::default(),
            high_numerator: Default::default(),
            denominator: Default::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("ratrng-1", rh_foundation::Severity::Error, "One of lowNumerator or highNumerator and denominator SHALL be present, or all are absent. If all are absent, there SHALL be some extension present", "((lowNumerator.exists() or highNumerator.exists()) and denominator.exists()) or (lowNumerator.empty() and highNumerator.empty() and denominator.empty() and extension.exists())"),
    rh_foundation::Invariant::new("ratrng-2", rh_foundation::Severity::Error, "If present, lowNumerator SHALL have a lower value than highNumerator", "lowNumerator.hasValue().not() or highNumerator.hasValue().not()  or (lowNumerator.lowBoundary() <= highNumerator.highBoundary())"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RatioRange.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RatioRange.extension", 0, None),
            rh_foundation::ElementCardinality::new("RatioRange.lowNumerator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RatioRange.highNumerator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RatioRange.denominator", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for RatioRange {
    fn resource_type(&self) -> &'static str {
        "RatioRange"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/RatioRange")
    }
}
