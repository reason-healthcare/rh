use crate::bindings::appointmentstatus::Appointmentstatus;
use crate::bindings::participantrequired::Participantrequired;
use crate::bindings::participationstatus::Participationstatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Appointment
///
/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Appointment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Appointment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    pub status: Appointmentstatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The coded reason for the appointment being cancelled
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/appointment-cancellation-reason
    #[serde(rename = "cancelationReason")]
    pub cancelation_reason: Option<CodeableConcept>,
    /// A broad categorization of the service that is to be performed during this appointment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-category
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// The specific service that is to be performed during this appointment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableConcept>>,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    ///
    /// Binding: preferred (No description)
    ///
    /// Available values:
    /// - `408467006`: Adult mental illness
    /// - `394577000`: Anesthetics
    /// - `394578005`: Audiological medicine
    /// - `421661004`: Blood banking and transfusion medicine
    /// - `408462000`: Burns care
    /// - `394579002`: Cardiology
    /// - `394804000`: Clinical cytogenetics and molecular genetics
    /// - `394580004`: Clinical genetics
    /// - `394803006`: Clinical hematology
    /// - `408480009`: Clinical immunology
    /// - ... and 107 more values
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    ///
    /// Binding: preferred (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0276
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<CodeableConcept>,
    /// Coded reason this appointment is scheduled
    ///
    /// Binding: preferred (The Reason for the appointment to take place.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Reason the appointment is to take place (resource)
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<UnsignedIntType>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Additional information to support the appointment
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// When appointment is to take place
    pub start: Option<InstantType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// When appointment is to conclude
    pub end: Option<InstantType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Can be less than start/end (e.g. estimate)
    #[serde(rename = "minutesDuration")]
    pub minutes_duration: Option<PositiveIntType>,
    /// Extension element for the 'minutesDuration' primitive field. Contains metadata and extensions.
    #[serde(rename = "_minutesDuration")]
    pub _minutes_duration: Option<Element>,
    /// The slots that this appointment is filling
    pub slot: Option<Vec<Reference>>,
    /// The date that this appointment was initially created
    pub created: Option<DateTimeType>,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Additional comments
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Detailed information and instructions for the patient
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<StringType>,
    /// Extension element for the 'patientInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_patientInstruction")]
    pub _patient_instruction: Option<Element>,
    /// The service request this appointment is allocated to assess
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Participants involved in appointment
    pub participant: Vec<AppointmentParticipant>,
    /// Potential date/time interval(s) requested to allocate the appointment within
    #[serde(rename = "requestedPeriod")]
    pub requested_period: Option<Vec<Period>>,
}
/// Appointment nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Role of participant in the appointment
    ///
    /// Binding: extensible (Role of participant in encounter.)
    ///
    /// Available values:
    /// - `SPRF`
    /// - `PPRF`
    /// - `PART`
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Person, Location/HealthcareService or Device
    pub actor: Option<Reference>,
    /// required | optional | information-only
    pub required: Option<Participantrequired>,
    /// Extension element for the 'required' primitive field. Contains metadata and extensions.
    pub _required: Option<Element>,
    /// accepted | declined | tentative | needs-action
    pub status: Participationstatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Participation period of the actor
    pub period: Option<Period>,
}

impl Default for Appointment {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Appointmentstatus::default(),
            _status: Default::default(),
            cancelation_reason: Default::default(),
            service_category: Default::default(),
            service_type: Default::default(),
            specialty: Default::default(),
            appointment_type: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            supporting_information: Default::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            minutes_duration: Default::default(),
            _minutes_duration: Default::default(),
            slot: Default::default(),
            created: Default::default(),
            _created: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            patient_instruction: Default::default(),
            _patient_instruction: Default::default(),
            based_on: Default::default(),
            participant: Vec::new(),
            requested_period: Default::default(),
        }
    }
}

impl Default for AppointmentParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            actor: Default::default(),
            required: Default::default(),
            _required: Default::default(),
            status: Participationstatus::default(),
            _status: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("app-1", rh_foundation::Severity::Error, "Either the type or actor on the participant SHALL be specified", "type.exists() or actor.exists()").with_xpath("(exists(f:type) or exists(f:actor))"),
    rh_foundation::Invariant::new("app-2", rh_foundation::Severity::Error, "Either start and end are specified, or neither", "start.exists() = end.exists()").with_xpath("((exists(f:start) and exists(f:end)) or (not(exists(f:start)) and not(exists(f:end))))"),
    rh_foundation::Invariant::new("app-3", rh_foundation::Severity::Error, "Only proposed or cancelled appointments can be missing start/end dates", "(start.exists() and end.exists()) or (status in ('proposed' | 'cancelled' | 'waitlist'))").with_xpath("((exists(f:start) and exists(f:end)) or (f:status/@value='proposed') or (f:status/@value='cancelled') or (f:status/@value='waitlist'))"),
    rh_foundation::Invariant::new("app-4", rh_foundation::Severity::Error, "Cancelation reason is only used for appointments that have been cancelled, or no-show", "Appointment.cancelationReason.exists() implies (Appointment.status='no-show' or Appointment.status='cancelled')").with_xpath("not(exists(f:cancellationReason)) or f:status/@value=('no-show', 'cancelled')"),
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
                "Appointment.participant.required",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/participantrequired|4.0.1",
            )
            .with_description("Is the Participant required to attend the appointment."),
            rh_foundation::ElementBinding::new(
                "Appointment.participant.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/participationstatus|4.0.1",
            )
            .with_description("The Participation status of an appointment."),
            rh_foundation::ElementBinding::new(
                "Appointment.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/appointmentstatus|4.0.1",
            )
            .with_description("The free/busy status of an appointment."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Appointment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.contained", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.extension", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.cancelationReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.serviceCategory", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.serviceType", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.specialty", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.appointmentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.reasonReference", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.supportingInformation", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.start", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.end", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.minutesDuration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.slot", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.created", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.patientInstruction", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.participant", 1, None),
            rh_foundation::ElementCardinality::new("Appointment.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Appointment.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Appointment.participant.type", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.participant.actor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.required", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.requestedPeriod", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Appointment {
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

impl crate::traits::resource::ResourceMutators for Appointment {
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

impl crate::traits::resource::ResourceExistence for Appointment {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Appointment {
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

impl crate::traits::domain_resource::DomainResourceMutators for Appointment {
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

impl crate::traits::domain_resource::DomainResourceExistence for Appointment {
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

impl crate::traits::appointment::AppointmentAccessors for Appointment {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Appointmentstatus {
        self.status.clone()
    }
    fn cancelation_reason(&self) -> Option<CodeableConcept> {
        self.cancelation_reason.clone()
    }
    fn service_category(&self) -> &[CodeableConcept] {
        self.service_category.as_deref().unwrap_or(&[])
    }
    fn service_type(&self) -> &[CodeableConcept] {
        self.service_type.as_deref().unwrap_or(&[])
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_deref().unwrap_or(&[])
    }
    fn appointment_type(&self) -> Option<CodeableConcept> {
        self.appointment_type.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn priority(&self) -> Option<UnsignedIntType> {
        self.priority
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn start(&self) -> Option<InstantType> {
        self.start.clone()
    }
    fn end(&self) -> Option<InstantType> {
        self.end.clone()
    }
    fn minutes_duration(&self) -> Option<PositiveIntType> {
        self.minutes_duration
    }
    fn slot(&self) -> &[Reference] {
        self.slot.as_deref().unwrap_or(&[])
    }
    fn created(&self) -> Option<DateTimeType> {
        self.created.clone()
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn patient_instruction(&self) -> Option<StringType> {
        self.patient_instruction.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn participant(&self) -> &[AppointmentParticipant] {
        &self.participant
    }
    fn requested_period(&self) -> &[Period] {
        self.requested_period.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::appointment::AppointmentMutators for Appointment {
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
    fn set_status(self, value: Appointmentstatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_cancelation_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.cancelation_reason = Some(value);
        resource
    }
    fn set_service_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_category = Some(value);
        resource
    }
    fn add_service_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_category
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_service_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_type = Some(value);
        resource
    }
    fn add_service_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.specialty = Some(value);
        resource
    }
    fn add_specialty(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.specialty.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_appointment_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.appointment_type = Some(value);
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
    fn set_priority(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = Some(value);
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_information
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_minutes_duration(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.minutes_duration = Some(value);
        resource
    }
    fn set_slot(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.slot = Some(value);
        resource
    }
    fn add_slot(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.slot.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = Some(value);
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_patient_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.patient_instruction = Some(value);
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
    fn set_participant(self, value: Vec<AppointmentParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = value;
        resource
    }
    fn add_participant(self, item: AppointmentParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.push(item);
        resource
    }
    fn set_requested_period(self, value: Vec<Period>) -> Self {
        let mut resource = self.clone();
        resource.requested_period = Some(value);
        resource
    }
    fn add_requested_period(self, item: Period) -> Self {
        let mut resource = self.clone();
        resource
            .requested_period
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::appointment::AppointmentExistence for Appointment {
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
    fn has_status(&self) -> bool {
        true
    }
    fn has_cancelation_reason(&self) -> bool {
        self.cancelation_reason.is_some()
    }
    fn has_service_category(&self) -> bool {
        self.service_category
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_service_type(&self) -> bool {
        self.service_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specialty(&self) -> bool {
        self.specialty.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_appointment_type(&self) -> bool {
        self.appointment_type.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_start(&self) -> bool {
        self.start.is_some()
    }
    fn has_end(&self) -> bool {
        self.end.is_some()
    }
    fn has_minutes_duration(&self) -> bool {
        self.minutes_duration.is_some()
    }
    fn has_slot(&self) -> bool {
        self.slot.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_created(&self) -> bool {
        self.created.is_some()
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_patient_instruction(&self) -> bool {
        self.patient_instruction.is_some()
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_participant(&self) -> bool {
        !self.participant.is_empty()
    }
    fn has_requested_period(&self) -> bool {
        self.requested_period
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Appointment {
    fn resource_type(&self) -> &'static str {
        "Appointment"
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
        Some("http://hl7.org/fhir/StructureDefinition/Appointment")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::appointment::{
    AppointmentAccessors, AppointmentExistence, AppointmentMutators,
};
