use crate::bindings::days_of_week::DaysOfWeek;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use serde::{Deserialize, Serialize};
/// Availability
///
/// Availability Type: Availability data for an {item}.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Availability
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Availability
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Times the {item} is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<Element>>,
    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailableTime")]
    pub not_available_time: Option<Vec<Element>>,
}
/// Availability nested structure for the 'availableTime' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityAvailabletime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Option<Vec<DaysOfWeek>>,
    /// Extension element for the 'daysOfWeek' primitive field. Contains metadata and extensions.
    #[serde(rename = "_daysOfWeek")]
    pub _days_of_week: Option<Element>,
    /// Always available? i.e. 24 hour service
    #[serde(rename = "allDay")]
    pub all_day: Option<BooleanType>,
    /// Extension element for the 'allDay' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allDay")]
    pub _all_day: Option<Element>,
    /// Opening time of day (ignored if allDay = true)
    #[serde(rename = "availableStartTime")]
    pub available_start_time: Option<TimeType>,
    /// Extension element for the 'availableStartTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availableStartTime")]
    pub _available_start_time: Option<Element>,
    /// Closing time of day (ignored if allDay = true)
    #[serde(rename = "availableEndTime")]
    pub available_end_time: Option<TimeType>,
    /// Extension element for the 'availableEndTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availableEndTime")]
    pub _available_end_time: Option<Element>,
}
/// Availability nested structure for the 'notAvailableTime' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityNotavailabletime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reason presented to the user explaining why time not available
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Service not available during this period
    pub during: Option<Period>,
}

impl Default for Availability {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            available_time: Default::default(),
            not_available_time: Default::default(),
        }
    }
}

impl Default for AvailabilityAvailabletime {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            days_of_week: Default::default(),
            _days_of_week: Default::default(),
            all_day: Default::default(),
            _all_day: Default::default(),
            available_start_time: Default::default(),
            _available_start_time: Default::default(),
            available_end_time: Default::default(),
            _available_end_time: Default::default(),
        }
    }
}

impl Default for AvailabilityNotavailabletime {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            during: Default::default(),
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
    rh_foundation::Invariant::new("av-1", rh_foundation::Severity::Error, "Cannot include start/end times when selecting all day availability.", "allDay.exists().not() or (allDay implies availableStartTime.exists().not() and availableEndTime.exists().not())"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Availability.availableTime.daysOfWeek",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/days-of-week|5.0.0",
        )
        .with_description("The purpose for which an extended contact detail should be used.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Availability.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Availability.extension", 0, None),
            rh_foundation::ElementCardinality::new("Availability.availableTime", 0, None),
            rh_foundation::ElementCardinality::new("Availability.availableTime.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Availability.availableTime.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Availability.availableTime.daysOfWeek",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Availability.availableTime.allDay", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Availability.availableTime.availableStartTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Availability.availableTime.availableEndTime",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Availability.notAvailableTime", 0, None),
            rh_foundation::ElementCardinality::new("Availability.notAvailableTime.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Availability.notAvailableTime.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Availability.notAvailableTime.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Availability.notAvailableTime.during",
                0,
                Some(1),
            ),
        ]
    });

impl crate::validation::ValidatableResource for Availability {
    fn resource_type(&self) -> &'static str {
        "Availability"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Availability")
    }
}
