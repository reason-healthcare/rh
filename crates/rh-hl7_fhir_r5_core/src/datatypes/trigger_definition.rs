use crate::bindings::trigger_type::TriggerType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::data_type::DataType;
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
/// TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: TriggerDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    #[serde(rename = "type")]
    pub type_: TriggerType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Name or URI that identifies the event
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Coded definition of the event
    pub code: Option<CodeableConcept>,
    /// What event
    #[serde(rename = "subscriptionTopic")]
    pub subscription_topic: Option<StringType>,
    /// Extension element for the 'subscriptionTopic' primitive field. Contains metadata and extensions.
    #[serde(rename = "_subscriptionTopic")]
    pub _subscription_topic: Option<Element>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<DataRequirement>,
    /// Whether the event triggers (boolean expression)
    pub condition: Option<Expression>,
}

impl Default for TriggerDefinition {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            type_: Default::default(),
            _type: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            code: Default::default(),
            subscription_topic: Default::default(),
            _subscription_topic: Default::default(),
            timing_timing: Default::default(),
            timing_reference: Default::default(),
            timing_date: Default::default(),
            timing_date_time: Default::default(),
            data: Default::default(),
            condition: Default::default(),
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
    rh_foundation::Invariant::new("trd-1", rh_foundation::Severity::Error, "Either timing, or a data requirement, but not both", "data.empty() or timing.empty()"),
    rh_foundation::Invariant::new("trd-2", rh_foundation::Severity::Error, "A condition only if there is a data requirement", "condition.exists() implies data.exists()"),
    rh_foundation::Invariant::new("trd-3", rh_foundation::Severity::Error, "A named event requires a name, a periodic event requires timing, and a data event requires data", "(type = 'named-event' implies name.exists()) and (type = 'periodic' implies timing.exists()) and (type.startsWith('data-') implies data.exists())"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "TriggerDefinition.type",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/trigger-type|5.0.0",
        )
        .with_description("The type of trigger.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("TriggerDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TriggerDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("TriggerDefinition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TriggerDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TriggerDefinition.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TriggerDefinition.subscriptionTopic",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TriggerDefinition.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TriggerDefinition.data", 0, None),
            rh_foundation::ElementCardinality::new("TriggerDefinition.condition", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for TriggerDefinition {
    fn resource_type(&self) -> &'static str {
        "TriggerDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/TriggerDefinition")
    }
}
