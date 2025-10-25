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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("trd-1", rh_foundation::Severity::Error, "Either timing, or a data requirement, but not both", "data.empty() or timing.empty()").with_xpath("not(exists(f:data)) or not(exists(*[starts-with(local-name(.), 'timing')]))"),
    rh_foundation::Invariant::new("trd-2", rh_foundation::Severity::Error, "A condition only if there is a data requirement", "condition.exists() implies data.exists()").with_xpath("not(exists(f:condition)) or exists(f:data)"),
    rh_foundation::Invariant::new("trd-3", rh_foundation::Severity::Error, "A named event requires a name, a periodic event requires timing, and a data event requires data", "(type = 'named-event' implies name.exists()) and (type = 'periodic' implies timing.exists()) and (type.startsWith('data-') implies data.exists())").with_xpath("((not(f:type/@value = 'named-event')) or name.exists()) and (not(f:type/@value = 'periodic') or timing.exists()) and (not(starts-with(f:type/@value, 'data-')) or data.exists())"),
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
            "http://hl7.org/fhir/ValueSet/trigger-type|4.0.1",
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
