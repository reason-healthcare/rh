use crate::bindings::medicationrequest_intent::MedicationrequestIntent;
use crate::bindings::medicationrequest_status::MedicationrequestStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::medication_request::MedicationRequestDispenserequest;
use crate::resources::medication_request::MedicationRequestSubstitution;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicationRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationRequestAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> MedicationrequestStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the intent field.
    fn intent(&self) -> MedicationrequestIntent;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<Reference>;
    /// Returns a reference to the performerType field.
    fn performer_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the groupIdentifier field.
    fn group_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the courseOfTherapyType field.
    fn course_of_therapy_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the dosageInstruction field.
    fn dosage_instruction(&self) -> &[Dosage];
    /// Returns a reference to the dispenseRequest field.
    fn dispense_request(&self) -> Option<MedicationRequestDispenserequest>;
    /// Returns a reference to the substitution field.
    fn substitution(&self) -> Option<MedicationRequestSubstitution>;
    /// Returns a reference to the priorPrescription field.
    fn prior_prescription(&self) -> Option<Reference>;
    /// Returns a reference to the detectedIssue field.
    fn detected_issue(&self) -> &[Reference];
    /// Returns a reference to the eventHistory field.
    fn event_history(&self) -> &[Reference];
}
/// MedicationRequest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationRequestMutators: DomainResourceMutators {
    /// Create a new MedicationRequest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medication_request::MedicationRequest;
    /// use hl7_fhir_r4_core::traits::medication_request::MedicationRequestMutators;
    ///
    /// let resource = MedicationRequest::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationrequestStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: MedicationrequestIntent) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the doNotPerform field and returns self for chaining.
    fn set_do_not_perform(self, value: bool) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the supportingInformation field and returns self for chaining.
    fn set_supporting_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInformation field and returns self for chaining.
    fn add_supporting_information(self, item: Reference) -> Self;
    /// Sets the authoredOn field and returns self for chaining.
    fn set_authored_on(self, value: String) -> Self;
    /// Sets the requester field and returns self for chaining.
    fn set_requester(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Reference) -> Self;
    /// Sets the performerType field and returns self for chaining.
    fn set_performer_type(self, value: CodeableConcept) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesCanonical field and returns self for chaining.
    fn add_instantiates_canonical(self, item: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesUri field and returns self for chaining.
    fn add_instantiates_uri(self, item: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the groupIdentifier field and returns self for chaining.
    fn set_group_identifier(self, value: Identifier) -> Self;
    /// Sets the courseOfTherapyType field and returns self for chaining.
    fn set_course_of_therapy_type(self, value: CodeableConcept) -> Self;
    /// Sets the insurance field and returns self for chaining.
    fn set_insurance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the insurance field and returns self for chaining.
    fn add_insurance(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the dosageInstruction field and returns self for chaining.
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self;
    /// Adds an item to the dosageInstruction field and returns self for chaining.
    fn add_dosage_instruction(self, item: Dosage) -> Self;
    /// Sets the dispenseRequest field and returns self for chaining.
    fn set_dispense_request(self, value: MedicationRequestDispenserequest) -> Self;
    /// Sets the substitution field and returns self for chaining.
    fn set_substitution(self, value: MedicationRequestSubstitution) -> Self;
    /// Sets the priorPrescription field and returns self for chaining.
    fn set_prior_prescription(self, value: Reference) -> Self;
    /// Sets the detectedIssue field and returns self for chaining.
    fn set_detected_issue(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the detectedIssue field and returns self for chaining.
    fn add_detected_issue(self, item: Reference) -> Self;
    /// Sets the eventHistory field and returns self for chaining.
    fn set_event_history(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the eventHistory field and returns self for chaining.
    fn add_event_history(self, item: Reference) -> Self;
}
/// MedicationRequest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationRequestExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the reported field is present (Some).
    fn has_reported(&self) -> bool;
    /// Returns true if the medication field is present (Some).
    fn has_medication(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the performer_type field is present (Some).
    fn has_performer_type(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the instantiates_canonical field is not empty.
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is not empty.
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the group_identifier field is present (Some).
    fn has_group_identifier(&self) -> bool;
    /// Returns true if the course_of_therapy_type field is present (Some).
    fn has_course_of_therapy_type(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the dosage_instruction field is not empty.
    fn has_dosage_instruction(&self) -> bool;
    /// Returns true if the dispense_request field is present (Some).
    fn has_dispense_request(&self) -> bool;
    /// Returns true if the substitution field is present (Some).
    fn has_substitution(&self) -> bool;
    /// Returns true if the prior_prescription field is present (Some).
    fn has_prior_prescription(&self) -> bool;
    /// Returns true if the detected_issue field is not empty.
    fn has_detected_issue(&self) -> bool;
    /// Returns true if the event_history field is not empty.
    fn has_event_history(&self) -> bool;
}
