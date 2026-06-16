use crate::bindings::days_of_week::DaysOfWeek;
use crate::bindings::event_timing::EventTiming;
use crate::bindings::units_of_time::UnitsOfTime;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::backbone_type::BackboneType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::time::TimeType;
use crate::primitives::unsigned_int::UnsignedIntType;
use serde::{Deserialize, Serialize};
/// Timing
///
/// Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Timing
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Timing
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneType,
    /// When the event occurs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<DateTimeType>,
    /// Extension element for the 'event' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _event: Vec<Element>,
    /// When the event is to occur
    pub repeat: Option<Element>,
    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    ///
    /// Binding: preferred (Code for a known / defined timing pattern.)
    ///
    /// Available values:
    /// - `BID`: BID
    /// - `TID`: TID
    /// - `QID`: QID
    /// - `AM`: AM
    /// - `PM`: PM
    /// - `QD`: QD
    /// - `QOD`: QOD
    /// - `Q1H`: every hour
    /// - `Q2H`: every 2 hours
    /// - `Q3H`: every 3 hours
    /// - ... and 6 more values
    pub code: Option<CodeableConcept>,
}
/// Timing nested structure for the 'repeat' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingRepeat {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Length/Range of lengths, or (Start and/or end) limits (Duration)
    #[serde(rename = "boundsDuration")]
    pub bounds_duration: Option<Duration>,
    /// Length/Range of lengths, or (Start and/or end) limits (Range)
    #[serde(rename = "boundsRange")]
    pub bounds_range: Option<Range>,
    /// Length/Range of lengths, or (Start and/or end) limits (Period)
    #[serde(rename = "boundsPeriod")]
    pub bounds_period: Option<Period>,
    /// Number of times to repeat
    pub count: Option<PositiveIntType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
    /// Maximum number of times to repeat
    #[serde(rename = "countMax")]
    pub count_max: Option<PositiveIntType>,
    /// Extension element for the 'countMax' primitive field. Contains metadata and extensions.
    #[serde(rename = "_countMax")]
    pub _count_max: Option<Element>,
    /// How long when it happens
    pub duration: Option<DecimalType>,
    /// Extension element for the 'duration' primitive field. Contains metadata and extensions.
    pub _duration: Option<Element>,
    /// How long when it happens (Max)
    #[serde(rename = "durationMax")]
    pub duration_max: Option<DecimalType>,
    /// Extension element for the 'durationMax' primitive field. Contains metadata and extensions.
    #[serde(rename = "_durationMax")]
    pub _duration_max: Option<Element>,
    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    #[serde(rename = "durationUnit")]
    pub duration_unit: Option<UnitsOfTime>,
    /// Extension element for the 'durationUnit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_durationUnit")]
    pub _duration_unit: Option<Element>,
    /// Indicates the number of repetitions that should occur within a period. I.e. Event occurs frequency times per period
    pub frequency: Option<PositiveIntType>,
    /// Extension element for the 'frequency' primitive field. Contains metadata and extensions.
    pub _frequency: Option<Element>,
    /// Event occurs up to frequencyMax times per period
    #[serde(rename = "frequencyMax")]
    pub frequency_max: Option<PositiveIntType>,
    /// Extension element for the 'frequencyMax' primitive field. Contains metadata and extensions.
    #[serde(rename = "_frequencyMax")]
    pub _frequency_max: Option<Element>,
    /// The duration to which the frequency applies. I.e. Event occurs frequency times per period
    pub period: Option<DecimalType>,
    /// Extension element for the 'period' primitive field. Contains metadata and extensions.
    pub _period: Option<Element>,
    /// Upper limit of period (3-4 hours)
    #[serde(rename = "periodMax")]
    pub period_max: Option<DecimalType>,
    /// Extension element for the 'periodMax' primitive field. Contains metadata and extensions.
    #[serde(rename = "_periodMax")]
    pub _period_max: Option<Element>,
    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    #[serde(rename = "periodUnit")]
    pub period_unit: Option<UnitsOfTime>,
    /// Extension element for the 'periodUnit' primitive field. Contains metadata and extensions.
    #[serde(rename = "_periodUnit")]
    pub _period_unit: Option<Element>,
    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(rename = "dayOfWeek")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_week: Vec<DaysOfWeek>,
    /// Extension element for the 'dayOfWeek' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dayOfWeek")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _day_of_week: Vec<Element>,
    /// Time of day for action
    #[serde(rename = "timeOfDay")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_of_day: Vec<TimeType>,
    /// Extension element for the 'timeOfDay' primitive field. Contains metadata and extensions.
    #[serde(rename = "_timeOfDay")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _time_of_day: Vec<Element>,
    /// Code for time period of occurrence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub when: Vec<EventTiming>,
    /// Extension element for the 'when' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _when: Vec<Element>,
    /// Minutes from event (before or after)
    pub offset: Option<UnsignedIntType>,
    /// Extension element for the 'offset' primitive field. Contains metadata and extensions.
    pub _offset: Option<Element>,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            base: BackboneType::default(),
            event: Default::default(),
            _event: Default::default(),
            repeat: Default::default(),
            code: Default::default(),
        }
    }
}

impl Default for TimingRepeat {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            bounds_duration: Default::default(),
            bounds_range: Default::default(),
            bounds_period: Default::default(),
            count: Default::default(),
            _count: Default::default(),
            count_max: Default::default(),
            _count_max: Default::default(),
            duration: Default::default(),
            _duration: Default::default(),
            duration_max: Default::default(),
            _duration_max: Default::default(),
            duration_unit: Default::default(),
            _duration_unit: Default::default(),
            frequency: Default::default(),
            _frequency: Default::default(),
            frequency_max: Default::default(),
            _frequency_max: Default::default(),
            period: Default::default(),
            _period: Default::default(),
            period_max: Default::default(),
            _period_max: Default::default(),
            period_unit: Default::default(),
            _period_unit: Default::default(),
            day_of_week: Default::default(),
            _day_of_week: Default::default(),
            time_of_day: Default::default(),
            _time_of_day: Default::default(),
            when: Default::default(),
            _when: Default::default(),
            offset: Default::default(),
            _offset: Default::default(),
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
    rh_foundation::Invariant::new("tim-1", rh_foundation::Severity::Error, "if there's a duration, there needs to be duration units", "duration.empty() or durationUnit.exists()"),
    rh_foundation::Invariant::new("tim-10", rh_foundation::Severity::Error, "If there's a timeOfDay, there cannot be a when, or vice versa", "timeOfDay.empty() or when.empty()"),
    rh_foundation::Invariant::new("tim-2", rh_foundation::Severity::Error, "if there's a period, there needs to be period units", "period.empty() or periodUnit.exists()"),
    rh_foundation::Invariant::new("tim-4", rh_foundation::Severity::Error, "duration SHALL be a non-negative value", "duration.exists() implies duration >= 0"),
    rh_foundation::Invariant::new("tim-5", rh_foundation::Severity::Error, "period SHALL be a non-negative value", "period.exists() implies period >= 0"),
    rh_foundation::Invariant::new("tim-6", rh_foundation::Severity::Error, "If there's a periodMax, there must be a period", "periodMax.empty() or period.exists()"),
    rh_foundation::Invariant::new("tim-7", rh_foundation::Severity::Error, "If there's a durationMax, there must be a duration", "durationMax.empty() or duration.exists()"),
    rh_foundation::Invariant::new("tim-8", rh_foundation::Severity::Error, "If there's a countMax, there must be a count", "countMax.empty() or count.exists()"),
    rh_foundation::Invariant::new("tim-9", rh_foundation::Severity::Error, "If there's an offset, there must be a when (and not C, CM, CD, CV)", "offset.empty() or (when.exists() and when.select($this in ('C' | 'CM' | 'CD' | 'CV')).allFalse())"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "Timing.repeat.dayOfWeek",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/days-of-week|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Timing.repeat.durationUnit",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/units-of-time|5.0.0",
            )
            .with_description("A unit of time (units from UCUM)."),
            rh_foundation::ElementBinding::new(
                "Timing.repeat.periodUnit",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/units-of-time|5.0.0",
            )
            .with_description("A unit of time (units from UCUM)."),
            rh_foundation::ElementBinding::new(
                "Timing.repeat.when",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/event-timing|5.0.0",
            )
            .with_description("Real-world event relating to the schedule."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Timing.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.extension", 0, None),
            rh_foundation::ElementCardinality::new("Timing.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Timing.event", 0, None),
            rh_foundation::ElementCardinality::new("Timing.repeat", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.extension", 0, None),
            rh_foundation::ElementCardinality::new("Timing.repeat.bounds[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.count", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.countMax", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.duration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.durationMax", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.durationUnit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.frequency", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.frequencyMax", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.periodMax", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.periodUnit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.repeat.dayOfWeek", 0, None),
            rh_foundation::ElementCardinality::new("Timing.repeat.timeOfDay", 0, None),
            rh_foundation::ElementCardinality::new("Timing.repeat.when", 0, None),
            rh_foundation::ElementCardinality::new("Timing.repeat.offset", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Timing.code", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Timing {
    fn resource_type(&self) -> &'static str {
        "Timing"
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
        Some("http://hl7.org/fhir/StructureDefinition/Timing")
    }
}
