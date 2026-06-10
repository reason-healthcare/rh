use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::primitives::date_time::DateTimeType;
use serde::{Deserialize, Serialize};
/// Period
///
/// Period Type: A time period defined by a start and end date and optionally time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Period
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Period
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Starting time with inclusive boundary
    pub start: Option<DateTimeType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// End time with inclusive boundary, if not ongoing
    pub end: Option<DateTimeType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
}

impl Default for Period {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
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
    rh_foundation::Invariant::new("per-1", rh_foundation::Severity::Error, "If present, start SHALL have a lower or equal value than end", "start.hasValue().not() or end.hasValue().not() or (start.lowBoundary() <= end.highBoundary())"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Period.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Period.extension", 0, None),
            rh_foundation::ElementCardinality::new("Period.start", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Period.end", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Period {
    fn resource_type(&self) -> &'static str {
        "Period"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Period")
    }
}
