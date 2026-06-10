use crate::datatypes::data_type::DataType;
use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Range
///
/// Range Type: A set of ordered Quantities defined by a low and high limit.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Range
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Range
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Low limit
    pub low: Option<Quantity>,
    /// High limit
    pub high: Option<Quantity>,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            low: Default::default(),
            high: Default::default(),
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
    rh_foundation::Invariant::new("rng-2", rh_foundation::Severity::Error, "If present, low SHALL have a lower value than high", "low.value.empty() or high.value.empty() or low.lowBoundary().comparable(high.highBoundary()).not() or (low.lowBoundary() <= high.highBoundary())"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Range.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Range.extension", 0, None),
            rh_foundation::ElementCardinality::new("Range.low", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Range.high", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Range {
    fn resource_type(&self) -> &'static str {
        "Range"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Range")
    }
}
