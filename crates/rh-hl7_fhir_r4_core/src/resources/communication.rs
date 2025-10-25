use crate::bindings::event_status::EventStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Communication
///
/// An occurrence of information being transmitted; e.g. an alert that was sent to a responsible provider, a public health agency that was notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Communication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Communication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Communication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Request fulfilled by this communication
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of this action
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// Reply to
    #[serde(rename = "inResponseTo")]
    pub in_response_to: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: EventStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Codes for the reason why a communication did not happen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/communication-not-done-reason
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Message category
    ///
    /// Binding: example (Codes for general categories of communications such as alerts, instructions, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/communication-category
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// A channel of communication
    ///
    /// Binding: example (Codes for communication mediums such as phone, fax, email, in person, etc.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ParticipationMode
    pub medium: Option<Vec<CodeableConcept>>,
    /// Focus of message
    pub subject: Option<Reference>,
    /// Description of the purpose/content
    ///
    /// Binding: example (Codes describing the purpose or content of the communication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/communication-topic
    pub topic: Option<CodeableConcept>,
    /// Resources that pertain to this communication
    pub about: Option<Vec<Reference>>,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// When sent
    pub sent: Option<DateTimeType>,
    /// Extension element for the 'sent' primitive field. Contains metadata and extensions.
    pub _sent: Option<Element>,
    /// When received
    pub received: Option<DateTimeType>,
    /// Extension element for the 'received' primitive field. Contains metadata and extensions.
    pub _received: Option<Element>,
    /// Message recipient
    pub recipient: Option<Vec<Reference>>,
    /// Message sender
    pub sender: Option<Reference>,
    /// Indication for message
    ///
    /// Binding: example (Codes for describing reasons for the occurrence of a communication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why was communication done?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Message payload
    pub payload: Option<Vec<CommunicationPayload>>,
    /// Comments made about the communication
    pub note: Option<Vec<Annotation>>,
}
/// Communication nested structure for the 'payload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPayload {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Message part content (string)
    #[serde(rename = "contentString")]
    pub content_string: StringType,
    /// Message part content (Attachment)
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,
    /// Message part content (Reference)
    #[serde(rename = "contentReference")]
    pub content_reference: Reference,
}
/// media
///
/// It contains enriched media representation of the alert message, such as a voice recording.  This may be used, for example for compliance with jurisdictional accessibility requirements, literacy issues, or translations of the unstructured text content in other languages.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/communication-media
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMedia {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Communication {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            in_response_to: Default::default(),
            status: EventStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            medium: Default::default(),
            subject: Default::default(),
            topic: Default::default(),
            about: Default::default(),
            encounter: Default::default(),
            sent: Default::default(),
            _sent: Default::default(),
            received: Default::default(),
            _received: Default::default(),
            recipient: Default::default(),
            sender: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            payload: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CommunicationPayload {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_string: Default::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
        }
    }
}

impl Default for CommunicationMedia {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
                "Communication.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|4.0.1",
            )
            .with_description("Codes indicating the relative importance of a communication."),
            rh_foundation::ElementBinding::new(
                "Communication.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/event-status|4.0.1",
            )
            .with_description("The status of the communication."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Communication.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.contained", 0, None),
            rh_foundation::ElementCardinality::new("Communication.extension", 0, None),
            rh_foundation::ElementCardinality::new("Communication.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Communication.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Communication.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("Communication.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("Communication.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Communication.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Communication.inResponseTo", 0, None),
            rh_foundation::ElementCardinality::new("Communication.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.category", 0, None),
            rh_foundation::ElementCardinality::new("Communication.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.medium", 0, None),
            rh_foundation::ElementCardinality::new("Communication.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.topic", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.about", 0, None),
            rh_foundation::ElementCardinality::new("Communication.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.sent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.received", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.recipient", 0, None),
            rh_foundation::ElementCardinality::new("Communication.sender", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("Communication.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("Communication.payload", 0, None),
            rh_foundation::ElementCardinality::new("Communication.payload.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.payload.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Communication.payload.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Communication.payload.content[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Communication.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Communication {
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

impl crate::traits::resource::ResourceMutators for Communication {
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

impl crate::traits::resource::ResourceExistence for Communication {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Communication {
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

impl crate::traits::domain_resource::DomainResourceMutators for Communication {
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

impl crate::traits::domain_resource::DomainResourceExistence for Communication {
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

impl crate::traits::communication::CommunicationAccessors for Communication {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn in_response_to(&self) -> &[Reference] {
        self.in_response_to.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EventStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn medium(&self) -> &[CodeableConcept] {
        self.medium.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn topic(&self) -> Option<CodeableConcept> {
        self.topic.clone()
    }
    fn about(&self) -> &[Reference] {
        self.about.as_deref().unwrap_or(&[])
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn sent(&self) -> Option<DateTimeType> {
        self.sent.clone()
    }
    fn received(&self) -> Option<DateTimeType> {
        self.received.clone()
    }
    fn recipient(&self) -> &[Reference] {
        self.recipient.as_deref().unwrap_or(&[])
    }
    fn sender(&self) -> Option<Reference> {
        self.sender.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn payload(&self) -> &[CommunicationPayload] {
        self.payload.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::communication::CommunicationMutators for Communication {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_uri
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
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_in_response_to(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.in_response_to = Some(value);
        resource
    }
    fn add_in_response_to(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .in_response_to
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_status(self, value: EventStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
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
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_medium(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.medium = Some(value);
        resource
    }
    fn add_medium(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.medium.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_topic(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.topic = Some(value);
        resource
    }
    fn set_about(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.about = Some(value);
        resource
    }
    fn add_about(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.about.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_sent(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.sent = Some(value);
        resource
    }
    fn set_received(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.received = Some(value);
        resource
    }
    fn set_recipient(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.recipient = Some(value);
        resource
    }
    fn add_recipient(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.recipient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_sender(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.sender = Some(value);
        resource
    }
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_payload(self, value: Vec<CommunicationPayload>) -> Self {
        let mut resource = self.clone();
        resource.payload = Some(value);
        resource
    }
    fn add_payload(self, item: CommunicationPayload) -> Self {
        let mut resource = self.clone();
        resource.payload.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::communication::CommunicationExistence for Communication {
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_in_response_to(&self) -> bool {
        self.in_response_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_medium(&self) -> bool {
        self.medium.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_topic(&self) -> bool {
        self.topic.is_some()
    }
    fn has_about(&self) -> bool {
        self.about.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_sent(&self) -> bool {
        self.sent.is_some()
    }
    fn has_received(&self) -> bool {
        self.received.is_some()
    }
    fn has_recipient(&self) -> bool {
        self.recipient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_sender(&self) -> bool {
        self.sender.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_payload(&self) -> bool {
        self.payload.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Communication {
    fn resource_type(&self) -> &'static str {
        "Communication"
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
        Some("http://hl7.org/fhir/StructureDefinition/Communication")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::communication::{
    CommunicationAccessors, CommunicationExistence, CommunicationMutators,
};
