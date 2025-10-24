use crate::bindings::participationstatus::Participationstatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AppointmentResponse
///
/// A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AppointmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentResponse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// Appointment this response relates to
    pub appointment: Reference,
    /// Time from appointment, or requested new start time
    pub start: Option<InstantType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// Time from appointment, or requested new end time
    pub end: Option<InstantType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Role of participant in the appointment
    ///
    /// Binding: extensible (Role of participant in encounter.)
    ///
    /// Available values:
    /// - `SPRF`
    /// - `PPRF`
    /// - `PART`
    #[serde(rename = "participantType")]
    pub participant_type: Option<Vec<CodeableConcept>>,
    /// Person, Location, HealthcareService, or Device
    pub actor: Option<Reference>,
    /// accepted | declined | tentative | needs-action
    #[serde(rename = "participantStatus")]
    pub participant_status: Participationstatus,
    /// Extension element for the 'participantStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_participantStatus")]
    pub _participant_status: Option<Element>,
    /// Additional comments
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}

impl Default for AppointmentResponse {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            appointment: Reference::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            participant_type: Default::default(),
            actor: Default::default(),
            participant_status: Participationstatus::default(),
            _participant_status: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
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
    rh_foundation::Invariant::new("apr-1", rh_foundation::Severity::Error, "Either the participantType or actor must be specified", "participantType.exists() or actor.exists()").with_xpath("(exists(f:participantType) or exists(f:actor))"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for AppointmentResponse {
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

impl crate::traits::resource::ResourceMutators for AppointmentResponse {
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

impl crate::traits::resource::ResourceExistence for AppointmentResponse {
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

impl crate::traits::domain_resource::DomainResourceAccessors for AppointmentResponse {
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

impl crate::traits::domain_resource::DomainResourceMutators for AppointmentResponse {
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

impl crate::traits::domain_resource::DomainResourceExistence for AppointmentResponse {
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

impl crate::traits::appointment_response::AppointmentResponseAccessors for AppointmentResponse {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn appointment(&self) -> Reference {
        self.appointment.clone()
    }
    fn start(&self) -> Option<InstantType> {
        self.start.clone()
    }
    fn end(&self) -> Option<InstantType> {
        self.end.clone()
    }
    fn participant_type(&self) -> &[CodeableConcept] {
        self.participant_type.as_deref().unwrap_or(&[])
    }
    fn actor(&self) -> Option<Reference> {
        self.actor.clone()
    }
    fn participant_status(&self) -> Participationstatus {
        self.participant_status.clone()
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
}

impl crate::traits::appointment_response::AppointmentResponseMutators for AppointmentResponse {
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
    fn set_appointment(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.appointment = value;
        resource
    }
    fn set_start(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.start = Some(value);
        resource
    }
    fn set_end(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.end = Some(value);
        resource
    }
    fn set_participant_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.participant_type = Some(value);
        resource
    }
    fn add_participant_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .participant_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_actor(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.actor = Some(value);
        resource
    }
    fn set_participant_status(self, value: Participationstatus) -> Self {
        let mut resource = self.clone();
        resource.participant_status = value;
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
}

impl crate::traits::appointment_response::AppointmentResponseExistence for AppointmentResponse {
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
    fn has_appointment(&self) -> bool {
        true
    }
    fn has_start(&self) -> bool {
        self.start.is_some()
    }
    fn has_end(&self) -> bool {
        self.end.is_some()
    }
    fn has_participant_type(&self) -> bool {
        self.participant_type
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_actor(&self) -> bool {
        self.actor.is_some()
    }
    fn has_participant_status(&self) -> bool {
        true
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
}

impl crate::validation::ValidatableResource for AppointmentResponse {
    fn resource_type(&self) -> &'static str {
        "AppointmentResponse"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/AppointmentResponse")
    }
}
