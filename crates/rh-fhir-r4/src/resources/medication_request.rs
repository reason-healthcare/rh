use crate::bindings::medicationrequest_intent::MedicationrequestIntent;
use crate::bindings::medicationrequest_status::MedicationrequestStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationRequest
///
/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External ids for this request
    pub identifier: Option<Vec<Identifier>>,
    /// active | on-hold | cancelled | completed | entered-in-error | stopped | draft | unknown
    pub status: MedicationrequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Identifies the reasons for a given status.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-status-reason
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: MedicationrequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Type of medication usage
    ///
    /// Binding: example (A coded concept identifying the category of medication request.  For example, where the medication is to be consumed or administered, or the type of medication treatment.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-category
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
    /// Reported rather than primary record (boolean)
    #[serde(rename = "reportedBoolean")]
    pub reported_boolean: Option<BooleanType>,
    /// Reported rather than primary record (Reference)
    #[serde(rename = "reportedReference")]
    pub reported_reference: Option<Reference>,
    /// Medication to be taken (CodeableConcept)
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,
    /// Medication to be taken (Reference)
    #[serde(rename = "medicationReference")]
    pub medication_reference: Reference,
    /// Who or group medication request is for
    pub subject: Reference,
    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<Reference>,
    /// Information to support ordering of the medication
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// When request was initially authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/What requested the Request
    pub requester: Option<Reference>,
    /// Intended performer of administration
    pub performer: Option<Reference>,
    /// Desired kind of performer of the medication administration
    ///
    /// Binding: example (Identifies the type of individual that is desired to administer the medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/performer-role
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// Person who entered the request
    pub recorder: Option<Reference>,
    /// Reason or indication for ordering or not ordering the medication
    ///
    /// Binding: example (A coded concept indicating why the medication was ordered.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Condition or observation that supports why the prescription is being written
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
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
    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// Overall pattern of medication administration
    ///
    /// Binding: example (Identifies the overall pattern of medication administratio.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-course-of-therapy
    #[serde(rename = "courseOfTherapyType")]
    pub course_of_therapy_type: Option<CodeableConcept>,
    /// Associated insurance coverage
    pub insurance: Option<Vec<Reference>>,
    /// Information about the prescription
    pub note: Option<Vec<Annotation>>,
    /// How the medication should be taken
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// Medication supply authorization
    #[serde(rename = "dispenseRequest")]
    pub dispense_request: Option<MedicationRequestDispenserequest>,
    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,
    /// An order/prescription that is being replaced
    #[serde(rename = "priorPrescription")]
    pub prior_prescription: Option<Reference>,
    /// Clinical Issue with action
    #[serde(rename = "detectedIssue")]
    pub detected_issue: Option<Vec<Reference>>,
    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Reference>>,
}
/// MedicationRequest nested structure for the 'dispenseRequest' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestDispenserequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// First fill details
    #[serde(rename = "initialFill")]
    pub initial_fill: Option<MedicationRequestDispenserequestInitialfill>,
    /// Minimum period of time between dispenses
    #[serde(rename = "dispenseInterval")]
    pub dispense_interval: Option<Duration>,
    /// Time period supply is authorized for
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
    /// Number of refills authorized
    #[serde(rename = "numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfRepeatsAllowed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub _number_of_repeats_allowed: Option<Element>,
    /// Amount of medication to supply per dispense
    pub quantity: Option<Quantity>,
    /// Number of days supply per dispense
    #[serde(rename = "expectedSupplyDuration")]
    pub expected_supply_duration: Option<Duration>,
    /// Intended dispenser
    pub performer: Option<Reference>,
}
/// MedicationRequestDispenserequest nested structure for the 'initialFill' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestDispenserequestInitialfill {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// First fill quantity
    pub quantity: Option<Quantity>,
    /// First fill duration
    pub duration: Option<Duration>,
}
/// MedicationRequest nested structure for the 'substitution' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestSubstitution {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether substitution is allowed or not (boolean)
    #[serde(rename = "allowedBoolean")]
    pub allowed_boolean: BooleanType,
    /// Whether substitution is allowed or not (CodeableConcept)
    #[serde(rename = "allowedCodeableConcept")]
    pub allowed_codeable_concept: CodeableConcept,
    /// Why should (not) substitution be made
    ///
    /// Binding: example (A coded concept describing the reason that a different medication should (or should not) be substituted from what was prescribed.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-SubstanceAdminSubstitutionReason
    pub reason: Option<CodeableConcept>,
}

impl Default for MedicationRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: MedicationrequestStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            intent: MedicationrequestIntent::default(),
            _intent: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            reported_boolean: Default::default(),
            reported_reference: Default::default(),
            medication_codeable_concept: Default::default(),
            medication_reference: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            performer: Default::default(),
            performer_type: Default::default(),
            recorder: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            group_identifier: Default::default(),
            course_of_therapy_type: Default::default(),
            insurance: Default::default(),
            note: Default::default(),
            dosage_instruction: Default::default(),
            dispense_request: Default::default(),
            substitution: Default::default(),
            prior_prescription: Default::default(),
            detected_issue: Default::default(),
            event_history: Default::default(),
        }
    }
}

impl Default for MedicationRequestDispenserequest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            initial_fill: Default::default(),
            dispense_interval: Default::default(),
            validity_period: Default::default(),
            number_of_repeats_allowed: Default::default(),
            _number_of_repeats_allowed: Default::default(),
            quantity: Default::default(),
            expected_supply_duration: Default::default(),
            performer: Default::default(),
        }
    }
}

impl Default for MedicationRequestDispenserequestInitialfill {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            duration: Default::default(),
        }
    }
}

impl Default for MedicationRequestSubstitution {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            allowed_boolean: Default::default(),
            allowed_codeable_concept: Default::default(),
            reason: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationRequest {
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

impl crate::traits::resource::ResourceMutators for MedicationRequest {
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

impl crate::traits::resource::ResourceExistence for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationRequest {
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

impl crate::traits::medication_request::MedicationRequestAccessors for MedicationRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MedicationrequestStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn intent(&self) -> MedicationrequestIntent {
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
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn performer(&self) -> Option<Reference> {
        self.performer.clone()
    }
    fn performer_type(&self) -> Option<CodeableConcept> {
        self.performer_type.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
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
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn course_of_therapy_type(&self) -> Option<CodeableConcept> {
        self.course_of_therapy_type.clone()
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn dosage_instruction(&self) -> &[Dosage] {
        self.dosage_instruction.as_deref().unwrap_or(&[])
    }
    fn dispense_request(&self) -> Option<MedicationRequestDispenserequest> {
        self.dispense_request.clone()
    }
    fn substitution(&self) -> Option<MedicationRequestSubstitution> {
        self.substitution.clone()
    }
    fn prior_prescription(&self) -> Option<Reference> {
        self.prior_prescription.clone()
    }
    fn detected_issue(&self) -> &[Reference] {
        self.detected_issue.as_deref().unwrap_or(&[])
    }
    fn event_history(&self) -> &[Reference] {
        self.event_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medication_request::MedicationRequestMutators for MedicationRequest {
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
    fn set_status(self, value: MedicationrequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_intent(self, value: MedicationrequestIntent) -> Self {
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
    fn set_performer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn set_performer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
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
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_course_of_therapy_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.course_of_therapy_type = Some(value);
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
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self {
        let mut resource = self.clone();
        resource.dosage_instruction = Some(value);
        resource
    }
    fn add_dosage_instruction(self, item: Dosage) -> Self {
        let mut resource = self.clone();
        resource
            .dosage_instruction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_dispense_request(self, value: MedicationRequestDispenserequest) -> Self {
        let mut resource = self.clone();
        resource.dispense_request = Some(value);
        resource
    }
    fn set_substitution(self, value: MedicationRequestSubstitution) -> Self {
        let mut resource = self.clone();
        resource.substitution = Some(value);
        resource
    }
    fn set_prior_prescription(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.prior_prescription = Some(value);
        resource
    }
    fn set_detected_issue(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.detected_issue = Some(value);
        resource
    }
    fn add_detected_issue(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .detected_issue
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_event_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.event_history = Some(value);
        resource
    }
    fn add_event_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .event_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medication_request::MedicationRequestExistence for MedicationRequest {
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
    fn has_reported(&self) -> bool {
        self.reported_boolean.is_some() || self.reported_reference.is_some()
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.is_some()
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_course_of_therapy_type(&self) -> bool {
        self.course_of_therapy_type.is_some()
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_dosage_instruction(&self) -> bool {
        self.dosage_instruction
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_dispense_request(&self) -> bool {
        self.dispense_request.is_some()
    }
    fn has_substitution(&self) -> bool {
        self.substitution.is_some()
    }
    fn has_prior_prescription(&self) -> bool {
        self.prior_prescription.is_some()
    }
    fn has_detected_issue(&self) -> bool {
        self.detected_issue.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_event_history(&self) -> bool {
        self.event_history.as_ref().is_some_and(|v| !v.is_empty())
    }
}
