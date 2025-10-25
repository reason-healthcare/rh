use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CommunicationRequest
///
/// A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CommunicationRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CommunicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan or proposal
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Request(s) replaced by this request
    pub replaces: Option<Vec<Reference>>,
    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Codes identifying the reason for the current state of a request.)
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// Message category
    ///
    /// Binding: example (Codes for general categories of communications such as alerts, instruction, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/communication-category
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// True if request is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// A channel of communication
    ///
    /// Binding: example (Codes for communication mediums such as phone, fax, email, in person, etc.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ParticipationMode
    pub medium: Option<Vec<CodeableConcept>>,
    /// Focus of message
    pub subject: Option<Reference>,
    /// Resources that pertain to this communication request
    pub about: Option<Vec<Reference>>,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// Message payload
    pub payload: Option<Vec<CommunicationRequestPayload>>,
    /// When scheduled (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When scheduled (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When request transitioned to being actionable
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/what is requesting service
    pub requester: Option<Reference>,
    /// Message recipient
    pub recipient: Option<Vec<Reference>>,
    /// Message sender
    pub sender: Option<Reference>,
    /// Why is communication needed?
    ///
    /// Binding: example (Codes for describing reasons for the occurrence of a communication.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActReason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why is communication needed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Comments made about communication request
    pub note: Option<Vec<Annotation>>,
}
/// CommunicationRequest nested structure for the 'payload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRequestPayload {
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

impl Default for CommunicationRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            replaces: Default::default(),
            group_identifier: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            medium: Default::default(),
            subject: Default::default(),
            about: Default::default(),
            encounter: Default::default(),
            payload: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            recipient: Default::default(),
            sender: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CommunicationRequestPayload {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_string: Default::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
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
                "CommunicationRequest.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|4.0.1",
            )
            .with_description(
                "Codes indicating the relative importance of a communication request.",
            ),
            rh_foundation::ElementBinding::new(
                "CommunicationRequest.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-status|4.0.1",
            )
            .with_description("The status of the communication request."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("CommunicationRequest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CommunicationRequest.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.contained", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CommunicationRequest.identifier", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.replaces", 0, None),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.groupIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CommunicationRequest.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.category", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.doNotPerform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.medium", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.about", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.payload", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.payload.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.payload.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.payload.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.payload.content[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.occurrence[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CommunicationRequest.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.recipient", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.sender", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CommunicationRequest.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("CommunicationRequest.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CommunicationRequest {
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

impl crate::traits::resource::ResourceMutators for CommunicationRequest {
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

impl crate::traits::resource::ResourceExistence for CommunicationRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CommunicationRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for CommunicationRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for CommunicationRequest {
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

impl crate::traits::communication_request::CommunicationRequestAccessors for CommunicationRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_deref().unwrap_or(&[])
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn status(&self) -> RequestStatus {
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
    fn do_not_perform(&self) -> Option<BooleanType> {
        self.do_not_perform
    }
    fn medium(&self) -> &[CodeableConcept] {
        self.medium.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn about(&self) -> &[Reference] {
        self.about.as_deref().unwrap_or(&[])
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn payload(&self) -> &[CommunicationRequestPayload] {
        self.payload.as_deref().unwrap_or(&[])
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
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
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::communication_request::CommunicationRequestMutators for CommunicationRequest {
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
    fn set_replaces(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.replaces = Some(value);
        resource
    }
    fn add_replaces(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.replaces.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
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
    fn set_do_not_perform(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.do_not_perform = Some(value);
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
    fn set_payload(self, value: Vec<CommunicationRequestPayload>) -> Self {
        let mut resource = self.clone();
        resource.payload = Some(value);
        resource
    }
    fn add_payload(self, item: CommunicationRequestPayload) -> Self {
        let mut resource = self.clone();
        resource.payload.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_authored_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored_on = Some(value);
        resource
    }
    fn set_requester(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requester = Some(value);
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

impl crate::traits::communication_request::CommunicationRequestExistence for CommunicationRequest {
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
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some() || self.occurrence_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
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
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_medium(&self) -> bool {
        self.medium.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_about(&self) -> bool {
        self.about.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_payload(&self) -> bool {
        self.payload.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
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
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CommunicationRequest {
    fn resource_type(&self) -> &'static str {
        "CommunicationRequest"
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
        Some("http://hl7.org/fhir/StructureDefinition/CommunicationRequest")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::communication_request::{
    CommunicationRequestAccessors, CommunicationRequestExistence, CommunicationRequestMutators,
};
