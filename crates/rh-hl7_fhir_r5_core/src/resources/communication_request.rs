use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CommunicationRequest
///
/// A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CommunicationRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: CommunicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Fulfills plan or proposal
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<Reference>,
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
    ///
    /// Available values:
    /// - `281323002`
    /// - `397709008`
    /// - `105480006`
    /// - `719500002`
    /// - `445060000`
    /// - `704273008`
    /// - `704274002`
    /// - `704458005`
    /// - `704275001`
    /// - `704276000`
    /// - ... and 8 more values
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Message category
    ///
    /// Binding: example (Codes for general categories of communications such as alerts, instruction, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/communication-category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medium: Vec<CodeableConcept>,
    /// Focus of message
    pub subject: Option<Reference>,
    /// Resources that pertain to this communication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Reference>,
    /// The Encounter during which this CommunicationRequest was created
    pub encounter: Option<Reference>,
    /// Message payload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<CommunicationRequestPayload>,
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
    /// Who asks for the information to be shared
    pub requester: Option<Reference>,
    /// Who to share the information with
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<Reference>,
    /// Who should share the information
    #[serde(rename = "informationProvider")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_provider: Vec<Reference>,
    /// Why is communication needed?
    ///
    /// Binding: example (Codes for describing reasons for the occurrence of a communication.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActReason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<CodeableReference>,
    /// Comments made about communication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// CommunicationRequest nested structure for the 'payload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRequestPayload {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Message part content (Attachment)
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,
    /// Message part content (Reference)
    #[serde(rename = "contentReference")]
    pub content_reference: Reference,
    /// Message part content (CodeableConcept)
    #[serde(rename = "contentCodeableConcept")]
    pub content_codeable_concept: CodeableConcept,
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
            intent: RequestIntent::default(),
            _intent: Default::default(),
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
            information_provider: Default::default(),
            reason: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CommunicationRequestPayload {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            content_attachment: Default::default(),
            content_reference: Default::default(),
            content_codeable_concept: Default::default(),
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
    rh_foundation::ElementBinding::new("CommunicationRequest.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|5.0.0").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("CommunicationRequest.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("CommunicationRequest.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Codes indicating the relative importance of a communication request."),
    rh_foundation::ElementBinding::new("CommunicationRequest.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|5.0.0").with_description("The status of the communication request."),
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
            rh_foundation::ElementCardinality::new("CommunicationRequest.intent", 1, Some(1)),
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
            rh_foundation::ElementCardinality::new(
                "CommunicationRequest.informationProvider",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CommunicationRequest.reason", 0, None),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for CommunicationRequest {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::communication_request::CommunicationRequestAccessors for CommunicationRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_slice()
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
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn do_not_perform(&self) -> Option<BooleanType> {
        self.do_not_perform
    }
    fn medium(&self) -> &[CodeableConcept] {
        self.medium.as_slice()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn about(&self) -> &[Reference] {
        self.about.as_slice()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn payload(&self) -> &[CommunicationRequestPayload] {
        self.payload.as_slice()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn recipient(&self) -> &[Reference] {
        self.recipient.as_slice()
    }
    fn information_provider(&self) -> &[Reference] {
        self.information_provider.as_slice()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
}

impl crate::traits::communication_request::CommunicationRequestMutators for CommunicationRequest {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_replaces(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.replaces = value;
        resource
    }
    fn add_replaces(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.replaces.push(item);
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
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
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
        resource.medium = value;
        resource
    }
    fn add_medium(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.medium.push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_about(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.about = value;
        resource
    }
    fn add_about(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.about.push(item);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_payload(self, value: Vec<CommunicationRequestPayload>) -> Self {
        let mut resource = self.clone();
        resource.payload = value;
        resource
    }
    fn add_payload(self, item: CommunicationRequestPayload) -> Self {
        let mut resource = self.clone();
        resource.payload.push(item);
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
        resource.recipient = value;
        resource
    }
    fn add_recipient(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.recipient.push(item);
        resource
    }
    fn set_information_provider(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.information_provider = value;
        resource
    }
    fn add_information_provider(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.information_provider.push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
}

impl crate::traits::communication_request::CommunicationRequestExistence for CommunicationRequest {
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some() || self.occurrence_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_replaces(&self) -> bool {
        !self.replaces.is_empty()
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
    fn has_intent(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_medium(&self) -> bool {
        !self.medium.is_empty()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_about(&self) -> bool {
        !self.about.is_empty()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_payload(&self) -> bool {
        !self.payload.is_empty()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_recipient(&self) -> bool {
        !self.recipient.is_empty()
    }
    fn has_information_provider(&self) -> bool {
        !self.information_provider.is_empty()
    }
    fn has_reason(&self) -> bool {
        !self.reason.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
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
