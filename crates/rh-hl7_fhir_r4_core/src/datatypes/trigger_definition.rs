use crate::bindings::trigger_type::TriggerType;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// TriggerDefinition
///
/// Base StructureDefinition for TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: TriggerDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    #[serde(rename = "type")]
    pub type_: TriggerType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Name or URI that identifies the event
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Timing of the event (Timing)
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,
    /// Timing of the event (Reference)
    #[serde(rename = "timingReference")]
    pub timing_reference: Option<Reference>,
    /// Timing of the event (date)
    #[serde(rename = "timingDate")]
    pub timing_date: Option<DateType>,
    /// Timing of the event (dateTime)
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<DateTimeType>,
    /// Triggering data of the event (multiple = 'and')
    pub data: Option<Vec<DataRequirement>>,
    /// Whether the event triggers (boolean expression)
    pub condition: Option<Expression>,
}

impl Default for TriggerDefinition {
    fn default() -> Self {
        Self {
            base: Element::default(),
            type_: Default::default(),
            _type: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            timing_timing: Default::default(),
            timing_reference: Default::default(),
            timing_date: Default::default(),
            timing_date_time: Default::default(),
            data: Default::default(),
            condition: Default::default(),
        }
    }
}
