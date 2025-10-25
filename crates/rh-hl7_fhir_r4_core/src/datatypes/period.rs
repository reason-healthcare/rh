use crate::datatypes::element::Element;
use crate::primitives::date_time::DateTimeType;
use serde::{Deserialize, Serialize};
/// Period
///
/// Base StructureDefinition for Period Type: A time period defined by a start and end date and optionally time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Period
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Period
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
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
            base: Element::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("per-1", rh_foundation::Severity::Error, "If present, start SHALL have a lower value than end", "start.hasValue().not() or end.hasValue().not() or (start <= end)").with_xpath("not(exists(f:start/@value)) or not(exists(f:end/@value)) or (xs:dateTime(f:start/@value) <= xs:dateTime(f:end/@value))"),
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
