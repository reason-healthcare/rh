use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ServiceRequest
///
/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers assigned to this order
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
    /// What request fulfills
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// What request replaces
    pub replaces: Option<Vec<Reference>>,
    /// Composite Request ID
    pub requisition: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Classification of service
    ///
    /// Binding: example (Classification of the requested service.)
    ///
    /// Available values:
    /// - `108252007`: Laboratory procedure
    /// - `363679005`: Imaging
    /// - `409063005`: Counselling
    /// - `409073007`: Education
    /// - `387713003`: Surgical procedure
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// True if service/procedure should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// What is being requested/ordered
    ///
    /// Binding: example (Codes for tests or services that can be carried out by a designated individual, organization or healthcare service.  For laboratory, LOINC is  (preferred)[http://build.fhir.org/terminologies.html#preferred] and a valueset using LOINC Order codes is available [here](valueset-diagnostic-requests.html).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableConcept>,
    /// Additional order information
    ///
    /// Binding: example (Codified order entry details which are based on order context.)
    ///
    /// Available values:
    /// - `47545007`: Continuous positive airway pressure ventilation treatment (regime/therapy)
    /// - `286812008`: Pressure controlled ventilation (procedure)
    /// - `243144002`: Patient triggered inspiratory assistance (procedure)
    /// - `243150007`: Assisted controlled mandatory ventilation (procedure)
    /// - `59427005`: Synchronized intermittent mandatory ventilation (procedure)
    #[serde(rename = "orderDetail")]
    pub order_detail: Option<Vec<CodeableConcept>>,
    /// Service amount (Quantity)
    #[serde(rename = "quantityQuantity")]
    pub quantity_quantity: Option<Quantity>,
    /// Service amount (Ratio)
    #[serde(rename = "quantityRatio")]
    pub quantity_ratio: Option<Ratio>,
    /// Service amount (Range)
    #[serde(rename = "quantityRange")]
    pub quantity_range: Option<Range>,
    /// Individual or Entity the service is ordered for
    pub subject: Reference,
    /// Encounter in which the request was created
    pub encounter: Option<Reference>,
    /// When service should occur (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When service should occur (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When service should occur (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
    /// Preconditions for service (boolean)
    #[serde(rename = "asNeededBoolean")]
    pub as_needed_boolean: Option<BooleanType>,
    /// Preconditions for service (CodeableConcept)
    #[serde(rename = "asNeededCodeableConcept")]
    pub as_needed_codeable_concept: Option<CodeableConcept>,
    /// Date request signed
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/what is requesting service
    pub requester: Option<Reference>,
    /// Performer role
    ///
    /// Binding: example (Indicates specific responsibility of an individual within the care team, such as "Primary physician", "Team coordinator", "Caregiver", etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/participant-role
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// Requested performer
    pub performer: Option<Vec<Reference>>,
    /// Requested location
    ///
    /// Binding: example (A location type where services are delivered.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ServiceDeliveryLocationRoleType
    #[serde(rename = "locationCode")]
    pub location_code: Option<Vec<CodeableConcept>>,
    /// Requested location
    #[serde(rename = "locationReference")]
    pub location_reference: Option<Vec<Reference>>,
    /// Explanation/Justification for procedure or service
    ///
    /// Binding: example (Diagnosis or problem codes justifying the reason for requesting the service investigation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Explanation/Justification for service or service
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Associated insurance coverage
    pub insurance: Option<Vec<Reference>>,
    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Procedure Samples
    pub specimen: Option<Vec<Reference>>,
    /// Location on Body
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// Comments
    pub note: Option<Vec<Annotation>>,
    /// Patient or consumer-oriented instructions
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<StringType>,
    /// Extension element for the 'patientInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_patientInstruction")]
    pub _patient_instruction: Option<Element>,
    /// Request provenance
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Reference>>,
}

impl Default for ServiceRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            replaces: Default::default(),
            requisition: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            code: Default::default(),
            order_detail: Default::default(),
            quantity_quantity: Default::default(),
            quantity_ratio: Default::default(),
            quantity_range: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            as_needed_boolean: Default::default(),
            as_needed_codeable_concept: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            performer_type: Default::default(),
            performer: Default::default(),
            location_code: Default::default(),
            location_reference: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            insurance: Default::default(),
            supporting_info: Default::default(),
            specimen: Default::default(),
            body_site: Default::default(),
            note: Default::default(),
            patient_instruction: Default::default(),
            _patient_instruction: Default::default(),
            relevant_history: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ServiceRequest {
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

impl crate::traits::resource::ResourceMutators for ServiceRequest {
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

impl crate::traits::resource::ResourceExistence for ServiceRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ServiceRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for ServiceRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for ServiceRequest {
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

impl crate::traits::service_request::ServiceRequestAccessors for ServiceRequest {
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
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_deref().unwrap_or(&[])
    }
    fn requisition(&self) -> Option<Identifier> {
        self.requisition.clone()
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
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
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn order_detail(&self) -> &[CodeableConcept] {
        self.order_detail.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn performer_type(&self) -> Option<CodeableConcept> {
        self.performer_type.clone()
    }
    fn performer(&self) -> &[Reference] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location_code(&self) -> &[CodeableConcept] {
        self.location_code.as_deref().unwrap_or(&[])
    }
    fn location_reference(&self) -> &[Reference] {
        self.location_reference.as_deref().unwrap_or(&[])
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn specimen(&self) -> &[Reference] {
        self.specimen.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> &[CodeableConcept] {
        self.body_site.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn patient_instruction(&self) -> Option<StringType> {
        self.patient_instruction.clone()
    }
    fn relevant_history(&self) -> &[Reference] {
        self.relevant_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::service_request::ServiceRequestMutators for ServiceRequest {
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
    fn set_requisition(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.requisition = Some(value);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_order_detail(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.order_detail = Some(value);
        resource
    }
    fn add_order_detail(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .order_detail
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
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
    fn set_performer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.location_code = Some(value);
        resource
    }
    fn add_location_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .location_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_location_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.location_reference = Some(value);
        resource
    }
    fn add_location_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .location_reference
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_insurance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specimen(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn add_specimen(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn add_body_site(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site.get_or_insert_with(Vec::new).push(item);
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
    fn set_patient_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.patient_instruction = Some(value);
        resource
    }
    fn set_relevant_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.relevant_history = Some(value);
        resource
    }
    fn add_relevant_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .relevant_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::service_request::ServiceRequestExistence for ServiceRequest {
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
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity_quantity.is_some()
            || self.quantity_ratio.is_some()
            || self.quantity_range.is_some()
    }
    fn has_as_needed(&self) -> bool {
        self.as_needed_boolean.is_some() || self.as_needed_codeable_concept.is_some()
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
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_requisition(&self) -> bool {
        self.requisition.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_intent(&self) -> bool {
        true
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
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_order_detail(&self) -> bool {
        self.order_detail.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location_code(&self) -> bool {
        self.location_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location_reference(&self) -> bool {
        self.location_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specimen(&self) -> bool {
        self.specimen.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient_instruction(&self) -> bool {
        self.patient_instruction.is_some()
    }
    fn has_relevant_history(&self) -> bool {
        self.relevant_history
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
