use crate::bindings::days_of_week::DaysOfWeek;
use crate::bindings::event_timing::EventTiming;
use crate::bindings::units_of_time::UnitsOfTime;
use crate::datatypes::backbone_element::BackboneElement;
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
/// Base StructureDefinition for Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Timing
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Timing
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// When the event occurs
    pub event: Option<Vec<DateTimeType>>,
    /// Extension element for the 'event' primitive field. Contains metadata and extensions.
    pub _event: Option<Element>,
    /// When the event is to occur
    pub repeat: Option<Element>,
    /// BID | TID | QID | AM | PM | QD | QOD | +
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
    /// Event occurs frequency times per period
    pub frequency: Option<PositiveIntType>,
    /// Extension element for the 'frequency' primitive field. Contains metadata and extensions.
    pub _frequency: Option<Element>,
    /// Event occurs up to frequencyMax times per period
    #[serde(rename = "frequencyMax")]
    pub frequency_max: Option<PositiveIntType>,
    /// Extension element for the 'frequencyMax' primitive field. Contains metadata and extensions.
    #[serde(rename = "_frequencyMax")]
    pub _frequency_max: Option<Element>,
    /// Event occurs frequency times per period
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
    pub day_of_week: Option<Vec<DaysOfWeek>>,
    /// Extension element for the 'dayOfWeek' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dayOfWeek")]
    pub _day_of_week: Option<Element>,
    /// Time of day for action
    #[serde(rename = "timeOfDay")]
    pub time_of_day: Option<Vec<TimeType>>,
    /// Extension element for the 'timeOfDay' primitive field. Contains metadata and extensions.
    #[serde(rename = "_timeOfDay")]
    pub _time_of_day: Option<Element>,
    /// Code for time period of occurrence
    pub when: Option<Vec<EventTiming>>,
    /// Extension element for the 'when' primitive field. Contains metadata and extensions.
    pub _when: Option<Element>,
    /// Minutes from event (before or after)
    pub offset: Option<UnsignedIntType>,
    /// Extension element for the 'offset' primitive field. Contains metadata and extensions.
    pub _offset: Option<Element>,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
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
