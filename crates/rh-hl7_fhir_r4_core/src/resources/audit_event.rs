use crate::bindings::audit_event_action::AuditEventAction;
use crate::bindings::audit_event_outcome::AuditEventOutcome;
use crate::bindings::network_type::NetworkType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AuditEvent
///
/// A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
/// - Version: 4.0.1
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
    /// Binding: extensible (Type of event.)
    ///
    /// Available values:
    /// - `110100`
    /// - `110101`
    /// - `110102`
    /// - `110103`
    /// - `110104`
    /// - `110105`
    /// - `110106`
    /// - `110107`
    /// - `110108`
    /// - `110109`
    /// - ... and 5 more values
    #[serde(rename = "type")]
    pub type_: Coding,
    /// More specific type/id for the event
    ///
    /// Binding: extensible (Sub-type of event.)
    ///
    /// Available values:
    /// - `110120`
    /// - `110121`
    /// - `110122`
    /// - `110123`
    /// - `110124`
    /// - `110125`
    /// - `110126`
    /// - `110127`
    /// - `110128`
    /// - `110129`
    /// - ... and 13 more values
    pub subtype: Option<Vec<Coding>>,
    /// Type of action performed during the event
    pub action: Option<AuditEventAction>,
    /// Extension element for the 'action' primitive field. Contains metadata and extensions.
    pub _action: Option<Element>,
    /// When the activity occurred
    pub period: Option<Period>,
    /// Time when the event was recorded
    pub recorded: InstantType,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
    /// Whether the event succeeded or failed
    pub outcome: Option<AuditEventOutcome>,
    /// Extension element for the 'outcome' primitive field. Contains metadata and extensions.
    pub _outcome: Option<Element>,
    /// Description of the event outcome
    #[serde(rename = "outcomeDesc")]
    pub outcome_desc: Option<StringType>,
    /// Extension element for the 'outcomeDesc' primitive field. Contains metadata and extensions.
    #[serde(rename = "_outcomeDesc")]
    pub _outcome_desc: Option<Element>,
    /// The purposeOfUse of the event
    ///
    /// Binding: extensible (The reason the activity took place.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    #[serde(rename = "purposeOfEvent")]
    pub purpose_of_event: Option<Vec<CodeableConcept>>,
    /// Actor involved in the event
    pub agent: Vec<AuditEventAgent>,
    /// Audit Event Reporter
    pub source: AuditEventSource,
    /// Data or objects used
    pub entity: Option<Vec<AuditEventEntity>>,
}
/// AuditEventEntity nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventEntityDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of the property
    #[serde(rename = "type")]
    pub type_: StringType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Property value (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Property value (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
}
/// AuditEvent nested structure for the 'source' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Logical source location within the enterprise
    pub site: Option<StringType>,
    /// Extension element for the 'site' primitive field. Contains metadata and extensions.
    pub _site: Option<Element>,
    /// The identity of source detecting the event
    pub observer: Reference,
    /// The type of source where event originated
    ///
    /// Binding: extensible (Code specifying the type of system that detected and recorded the event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-source-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<Coding>>,
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
    /// Type of entity involved
    ///
    /// Binding: extensible (Code for the entity type involved in the audit event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/audit-entity-type
    #[serde(rename = "type")]
    pub type_: Option<Coding>,
    /// What role the entity played
    ///
    /// Binding: extensible (Code representing the role the entity played in the audit event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/object-role
    pub role: Option<Coding>,
    /// Life-cycle stage for the entity
    ///
    /// Binding: extensible (Identifier for the data life-cycle stage for the entity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/object-lifecycle-events
    pub lifecycle: Option<Coding>,
    /// Security labels on the entity
    ///
    /// Binding: extensible (Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-labels
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<Coding>>,
    /// Descriptor for entity
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Descriptive text
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Query parameters
    pub query: Option<Base64BinaryType>,
    /// Extension element for the 'query' primitive field. Contains metadata and extensions.
    pub _query: Option<Element>,
}
/// AuditEventAgent nested structure for the 'network' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventAgentNetwork {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifier for the network access point of the user device
    pub address: Option<StringType>,
    /// Extension element for the 'address' primitive field. Contains metadata and extensions.
    pub _address: Option<Element>,
    /// The type of network access point
    #[serde(rename = "type")]
    pub type_: Option<NetworkType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// AuditEvent nested structure for the 'agent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventAgent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Logical network location for application activity
    pub network: Option<AuditEventAgentNetwork>,
    /// How agent participated
    ///
    /// Binding: extensible (The Participation type of the agent to the event.)
    ///
    /// Available values:
    /// - `AMENDER`
    /// - `COAUTH`
    /// - `CONT`
    /// - `EVTWIT`
    /// - `PRIMAUTH`
    /// - `REVIEWER`
    /// - `SOURCE`
    /// - `TRANS`
    /// - `VALID`
    /// - `VERF`
    /// - ... and 53 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Agent role in the event
    ///
    /// Binding: example (What security role enabled the agent to participate in the event.)
    ///
    /// Available values:
    /// - `AMENDER`
    /// - `COAUTH`
    /// - `CONT`
    /// - `EVTWIT`
    /// - `PRIMAUTH`
    /// - `REVIEWER`
    /// - `SOURCE`
    /// - `TRANS`
    /// - `VALID`
    /// - `VERF`
    /// - ... and 53 more values
    pub role: Option<Vec<CodeableConcept>>,
    /// Identifier of who
    pub who: Option<Reference>,
    /// Alternative User identity
    #[serde(rename = "altId")]
    pub alt_id: Option<StringType>,
    /// Extension element for the 'altId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_altId")]
    pub _alt_id: Option<Element>,
    /// Human friendly name for the agent
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Whether user is initiator
    pub requestor: BooleanType,
    /// Extension element for the 'requestor' primitive field. Contains metadata and extensions.
    pub _requestor: Option<Element>,
    /// Where
    pub location: Option<Reference>,
    /// Policy that authorized event
    pub policy: Option<Vec<StringType>>,
    /// Extension element for the 'policy' primitive field. Contains metadata and extensions.
    pub _policy: Option<Element>,
    /// Type of media
    ///
    /// Binding: extensible (Used when the event is about exporting/importing onto media.)
    ///
    /// Available values:
    /// - `110030`
    /// - `110031`
    /// - `110032`
    /// - `110033`
    /// - `110034`
    /// - `110035`
    /// - `110036`
    /// - `110037`
    /// - `110010`
    /// - `110038`
    pub media: Option<Coding>,
    /// Reason given for this user
    ///
    /// Binding: extensible (The reason the activity took place.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-PurposeOfUse
    #[serde(rename = "purposeOfUse")]
    pub purpose_of_use: Option<Vec<CodeableConcept>>,
}

impl Default for AuditEvent {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            type_: Default::default(),
            subtype: Default::default(),
            action: Default::default(),
            _action: Default::default(),
            period: Default::default(),
            recorded: InstantType::default(),
            _recorded: Default::default(),
            outcome: Default::default(),
            _outcome: Default::default(),
            outcome_desc: Default::default(),
            _outcome_desc: Default::default(),
            purpose_of_event: Default::default(),
            agent: Vec::new(),
            source: AuditEventSource::default(),
            entity: Default::default(),
        }
    }
}

impl Default for AuditEventEntityDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            value_string: Default::default(),
            value_base64_binary: Default::default(),
        }
    }
}

impl Default for AuditEventSource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            site: Default::default(),
            _site: Default::default(),
            observer: Reference::default(),
            type_: Default::default(),
        }
    }
}

impl Default for AuditEventEntity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            what: Default::default(),
            type_: Default::default(),
            role: Default::default(),
            lifecycle: Default::default(),
            security_label: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            query: Default::default(),
            _query: Default::default(),
        }
    }
}

impl Default for AuditEventAgentNetwork {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            address: Default::default(),
            _address: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for AuditEventAgent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            network: Default::default(),
            type_: Default::default(),
            role: Default::default(),
            who: Default::default(),
            alt_id: Default::default(),
            _alt_id: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            requestor: BooleanType::default(),
            _requestor: Default::default(),
            location: Default::default(),
            policy: Default::default(),
            _policy: Default::default(),
            media: Default::default(),
            purpose_of_use: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("sev-1", rh_foundation::Severity::Error, "Either a name or a query (NOT both)", "name.empty() or query.empty()").with_xpath("not(exists(f:name)) or not(exists(f:query))"),
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
                "AuditEvent.action",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/audit-event-action|4.0.1",
            )
            .with_description(
                "Indicator for type of action performed during the event that generated the event.",
            ),
            rh_foundation::ElementBinding::new(
                "AuditEvent.agent.network.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/network-type|4.0.1",
            )
            .with_description("The type of network access point of this agent in the audit event."),
            rh_foundation::ElementBinding::new(
                "AuditEvent.outcome",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/audit-event-outcome|4.0.1",
            )
            .with_description("Indicates whether the event succeeded or failed."),
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
            rh_foundation::ElementCardinality::new("AuditEvent.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.subtype", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.action", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.recorded", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.outcomeDesc", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.purposeOfEvent", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent", 1, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.extension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.role", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.who", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.altId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.requestor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.policy", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.media", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "AuditEvent.agent.network.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network.address", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.network.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.agent.purposeOfUse", 0, None),
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
            rh_foundation::ElementCardinality::new("AuditEvent.entity.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.lifecycle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.securityLabel", 0, None),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AuditEvent.entity.description", 0, Some(1)),
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
    fn type_(&self) -> Coding {
        self.type_.clone()
    }
    fn subtype(&self) -> &[Coding] {
        self.subtype.as_deref().unwrap_or(&[])
    }
    fn action(&self) -> Option<AuditEventAction> {
        self.action.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn recorded(&self) -> InstantType {
        self.recorded.clone()
    }
    fn outcome(&self) -> Option<AuditEventOutcome> {
        self.outcome.clone()
    }
    fn outcome_desc(&self) -> Option<StringType> {
        self.outcome_desc.clone()
    }
    fn purpose_of_event(&self) -> &[CodeableConcept] {
        self.purpose_of_event.as_deref().unwrap_or(&[])
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
    fn set_type_(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_subtype(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.subtype = Some(value);
        resource
    }
    fn add_subtype(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource.subtype.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_action(self, value: AuditEventAction) -> Self {
        let mut resource = self.clone();
        resource.action = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
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
    fn set_outcome_desc(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.outcome_desc = Some(value);
        resource
    }
    fn set_purpose_of_event(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.purpose_of_event = Some(value);
        resource
    }
    fn add_purpose_of_event(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .purpose_of_event
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn has_type_(&self) -> bool {
        true
    }
    fn has_subtype(&self) -> bool {
        self.subtype.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_action(&self) -> bool {
        self.action.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_recorded(&self) -> bool {
        true
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_outcome_desc(&self) -> bool {
        self.outcome_desc.is_some()
    }
    fn has_purpose_of_event(&self) -> bool {
        self.purpose_of_event
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
