use crate::bindings::audit_event_action::AuditEventAction;
use crate::bindings::audit_event_severity::AuditEventSeverity;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AuditEvent
///
/// A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AuditEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Type/identifier of event
    ///
    /// Binding: example (Type of event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-event-type
    pub category: Option<Vec<CodeableConcept>>,
    /// Specific type of event
    ///
    /// Binding: example (Specific type of event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-event-sub-type
    pub code: CodeableConcept,
    /// Type of action performed during the event
    pub action: Option<AuditEventAction>,
    /// Extension element for the 'action' primitive field. Contains metadata and extensions.
    pub _action: Option<Element>,
    /// emergency | alert | critical | error | warning | notice | informational | debug
    pub severity: Option<AuditEventSeverity>,
    /// Extension element for the 'severity' primitive field. Contains metadata and extensions.
    pub _severity: Option<Element>,
    /// When the activity occurred (Period)
    #[serde(rename = "occurredPeriod")]
    pub occurred_period: Option<Period>,
    /// When the activity occurred (dateTime)
    #[serde(rename = "occurredDateTime")]
    pub occurred_date_time: Option<DateTimeType>,
    /// Time when the event was recorded
    pub recorded: InstantType,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Whether the event succeeded or failed
    pub outcome: Option<AuditEventOutcome>,
    /// Authorization related to the event
    ///
    /// Binding: example (The authorized purposeOfUse for the activity.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub authorization: Option<Vec<CodeableConcept>>,
    /// Workflow authorization within which this event occurred
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// The patient is the subject of the data used/created/updated/deleted during the activity
    pub patient: Option<Reference>,
    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<Reference>,
    /// Actor involved in the event
    pub agent: Vec<AuditEventAgent>,
    /// Audit Event Reporter
    pub source: AuditEventSource,
    /// Data or objects used
    pub entity: Option<Vec<AuditEventEntity>>,
}
/// AuditEvent nested structure for the 'entity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventEntity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Additional Information about the entity
    pub detail: Option<Vec<AuditEventEntityDetail>>,
    /// Specific instance of resource
    pub what: Option<Reference>,
    /// What role the entity played
    ///
    /// Binding: example (DICOM Audit Event Entity Role)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/object-role
    pub role: Option<CodeableConcept>,
    /// Security labels on the entity
    ///
    /// Binding: example (Example Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-label-examples
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<CodeableConcept>>,
    /// Query parameters
    pub query: Option<Base64BinaryType>,
    /// Extension element for the 'query' primitive field. Contains metadata and extensions.
    pub _query: Option<Element>,
    /// Entity is attributed to this agent
    pub agent: Option<Vec<StringType>>,
}
/// AuditEvent nested structure for the 'outcome' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventOutcome {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether the event succeeded or failed
    ///
    /// Binding: preferred (DICOM Audit Event Outcome)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-event-outcome
    pub code: Coding,
    /// Additional outcome detail
    ///
    /// Binding: example (A code that provides details as the exact issue.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-event-outcome-detail
    pub detail: Option<Vec<CodeableConcept>>,
}
/// AuditEvent nested structure for the 'source' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Logical source location within the enterprise
    pub site: Option<Reference>,
    /// The identity of source detecting the event
    pub observer: Reference,
    /// The type of source where event originated
    ///
    /// Binding: preferred (Code specifying the type of system that detected and recorded the event. Use of these codes is not required but is encouraged to maintain translation with DICOM AuditMessage schema.)
    ///
    /// Available values:
    /// - `1`
    /// - `2`
    /// - `3`
    /// - `4`
    /// - `5`
    /// - `6`
    /// - `7`
    /// - `8`
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
}
/// AuditEvent nested structure for the 'agent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventAgent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How agent participated
    ///
    /// Binding: preferred (The Participation type of the agent to the event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participation-role-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Agent role in the event
    ///
    /// Binding: example (What security role enabled the agent to participate in the event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-role-type
    pub role: Option<Vec<CodeableConcept>>,
    /// Identifier of who
    pub who: Reference,
    /// Whether user is initiator
    pub requestor: Option<BooleanType>,
    /// Extension element for the 'requestor' primitive field. Contains metadata and extensions.
    pub _requestor: Option<Element>,
    /// The agent location when the event occurred
    pub location: Option<Reference>,
    /// Policy that authorized the agent participation in the event
    pub policy: Option<Vec<StringType>>,
    /// Extension element for the 'policy' primitive field. Contains metadata and extensions.
    pub _policy: Option<Element>,
    /// This agent network location for the activity (Reference)
    #[serde(rename = "networkReference")]
    pub network_reference: Option<Reference>,
    /// This agent network location for the activity (uri)
    #[serde(rename = "networkUri")]
    pub network_uri: Option<StringType>,
    /// This agent network location for the activity (string)
    #[serde(rename = "networkString")]
    pub network_string: Option<StringType>,
    /// Allowable authorization for this agent
    ///
    /// Binding: example (The reason the activity took place.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    pub authorization: Option<Vec<CodeableConcept>>,
}
/// AuditEventEntity nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventEntityDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of the property
    ///
    /// Binding: example (Additional detail about an entity used in an event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-event-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Property value (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Property value (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Property value (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Property value (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Property value (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Property value (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Property value (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// Property value (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Property value (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Property value (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
    /// Property value (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
}

impl Default for AuditEvent {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            category: Default::default(),
            code: CodeableConcept::default(),
            action: Default::default(),
            _action: Default::default(),
            severity: Default::default(),
            _severity: Default::default(),
            occurred_period: Default::default(),
            occurred_date_time: Default::default(),
            recorded: InstantType::default(),
            _recorded: Default::default(),
            outcome: Default::default(),
            authorization: Default::default(),
            based_on: Default::default(),
            patient: Default::default(),
            encounter: Default::default(),
            agent: Vec::new(),
            source: AuditEventSource::default(),
            entity: Default::default(),
        }
    }
}

impl Default for AuditEventEntity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            what: Default::default(),
            role: Default::default(),
            security_label: Default::default(),
            query: Default::default(),
            _query: Default::default(),
            agent: Default::default(),
        }
    }
}

impl Default for AuditEventOutcome {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Coding::default(),
            detail: Default::default(),
        }
    }
}

impl Default for AuditEventSource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            site: Default::default(),
            observer: Reference::default(),
            type_: Default::default(),
        }
    }
}

impl Default for AuditEventAgent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            role: Default::default(),
            who: Reference::default(),
            requestor: Default::default(),
            _requestor: Default::default(),
            location: Default::default(),
            policy: Default::default(),
            _policy: Default::default(),
            network_reference: Default::default(),
            network_uri: Default::default(),
            network_string: Default::default(),
            authorization: Default::default(),
        }
    }
}

impl Default for AuditEventEntityDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_time: Default::default(),
            value_date_time: Default::default(),
            value_period: Default::default(),
            value_base64_binary: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
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
        vec![
    rh_foundation::ElementBinding::new("AuditEvent.action", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/audit-event-action|5.0.0").with_description("DICOM Audit Event Action"),
    rh_foundation::ElementBinding::new("AuditEvent.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("AuditEvent.severity", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/audit-event-severity|5.0.0").with_description("This is in the SysLog header, PRI. http://tools.ietf.org/html/rfc5424#appendix-A.3"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("AuditEvent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.contained", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.category", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.action", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.severity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.occurred[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.recorded", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome.detail", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.authorization", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.patient", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent", 1, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.role", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.who", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.requestor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.policy", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.authorization", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.source", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.source.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.source.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.source.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.source.site", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.source.observer", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.source.type", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.what", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.securityLabel", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.query", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.detail", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.detail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.detail.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "AuditEvent.entity.detail.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.detail.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.detail.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.agent", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for AuditEvent {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for AuditEvent {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for AuditEvent {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for AuditEvent {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for AuditEvent {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for AuditEvent {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::audit_event::AuditEventAccessors for AuditEvent {
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn action(&self) -> Option<AuditEventAction> {
        self.action.clone()
    }
    fn severity(&self) -> Option<AuditEventSeverity> {
        self.severity.clone()
    }
    fn recorded(&self) -> InstantType {
        self.recorded.clone()
    }
    fn outcome(&self) -> Option<AuditEventOutcome> {
        self.outcome.clone()
    }
    fn authorization(&self) -> &[CodeableConcept] {
        self.authorization.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn agent(&self) -> &[AuditEventAgent] {
        &self.agent
    }
    fn source(&self) -> AuditEventSource {
        self.source.clone()
    }
    fn entity(&self) -> &[AuditEventEntity] {
        self.entity.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::audit_event::AuditEventMutators for AuditEvent {
    fn new() -> Self {
        Self::default()
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_action(self, value: AuditEventAction) -> Self {
        let mut resource = self.clone();
        resource.action = Some(value);
        resource
    }
    fn set_severity(self, value: AuditEventSeverity) -> Self {
        let mut resource = self.clone();
        resource.severity = Some(value);
        resource
    }
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = value;
        resource
    }
    fn set_outcome(self, value: AuditEventOutcome) -> Self {
        let mut resource = self.clone();
        resource.outcome = Some(value);
        resource
    }
    fn set_authorization(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.authorization = Some(value);
        resource
    }
    fn add_authorization(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .authorization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_agent(self, value: Vec<AuditEventAgent>) -> Self {
        let mut resource = self.clone();
        resource.agent = value;
        resource
    }
    fn add_agent(self, item: AuditEventAgent) -> Self {
        let mut resource = self.clone();
        resource.agent.push(item);
        resource
    }
    fn set_source(self, value: AuditEventSource) -> Self {
        let mut resource = self.clone();
        resource.source = value;
        resource
    }
    fn set_entity(self, value: Vec<AuditEventEntity>) -> Self {
        let mut resource = self.clone();
        resource.entity = Some(value);
        resource
    }
    fn add_entity(self, item: AuditEventEntity) -> Self {
        let mut resource = self.clone();
        resource.entity.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::audit_event::AuditEventExistence for AuditEvent {
    fn has_occurred(&self) -> bool {
        self.occurred_period.is_some() || self.occurred_date_time.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_action(&self) -> bool {
        self.action.is_some()
    }
    fn has_severity(&self) -> bool {
        self.severity.is_some()
    }
    fn has_recorded(&self) -> bool {
        true
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_authorization(&self) -> bool {
        self.authorization.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_agent(&self) -> bool {
        !self.agent.is_empty()
    }
    fn has_source(&self) -> bool {
        true
    }
    fn has_entity(&self) -> bool {
        self.entity.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for AuditEvent {
    fn resource_type(&self) -> &'static str {
        "AuditEvent"
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
        Some("http://hl7.org/fhir/StructureDefinition/AuditEvent")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::audit_event::{
    AuditEventAccessors, AuditEventExistence, AuditEventMutators,
};
