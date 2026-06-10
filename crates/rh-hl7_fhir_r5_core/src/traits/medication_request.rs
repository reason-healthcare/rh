use crate::bindings::medicationrequest_intent::MedicationrequestIntent;
use crate::bindings::medicationrequest_status::MedicationrequestStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationRequestAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the priorPrescription field.
    fn prior_prescription(&self) -> Option<Reference>;
    /// Returns a reference to the groupIdentifier field.
    fn group_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> MedicationrequestStatus;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the statusChanged field.
    fn status_changed(&self) -> Option<DateTimeType>;
    /// Returns a reference to the intent field.
    fn intent(&self) -> MedicationrequestIntent;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the medication field.
    fn medication(&self) -> CodeableReference;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the informationSource field.
    fn information_source(&self) -> &[Reference];
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the authoredOn field.
    fn authored_on(&self) -> Option<DateTimeType>;
    /// Returns a reference to the requester field.
    fn requester(&self) -> Option<Reference>;
    /// Returns a reference to the reported field.
    fn reported(&self) -> Option<BooleanType>;
    /// Returns a reference to the performerType field.
    fn performer_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> &[Reference];
    /// Returns a reference to the device field.
    fn device(&self) -> &[CodeableReference];
    /// Returns a reference to the recorder field.
    fn recorder(&self) -> Option<Reference>;
    /// Returns a reference to the reason field.
    fn reason(&self) -> &[CodeableReference];
    /// Returns a reference to the courseOfTherapyType field.
    fn course_of_therapy_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the insurance field.
    fn insurance(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the renderedDosageInstruction field.
    fn rendered_dosage_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the effectiveDosePeriod field.
    fn effective_dose_period(&self) -> Option<Period>;
    /// Returns a reference to the dosageInstruction field.
    fn dosage_instruction(&self) -> &[Dosage];
    /// Returns a reference to the dispenseRequest field.
    fn dispense_request(&self) -> Option<MedicationRequestDispenserequest>;
    /// Returns a reference to the substitution field.
    fn substitution(&self) -> Option<MedicationRequestSubstitution>;
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
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::medication_request::MedicationRequest;
    /// use rh_hl7_fhir_r5_core::traits::medication_request::MedicationRequestMutators;
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
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the priorPrescription field and returns self for chaining.
    fn set_prior_prescription(self, value: Reference) -> Self;
    /// Sets the groupIdentifier field and returns self for chaining.
    fn set_group_identifier(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationrequestStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: CodeableConcept) -> Self;
    /// Sets the statusChanged field and returns self for chaining.
    fn set_status_changed(self, value: String) -> Self;
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
    /// Sets the medication field and returns self for chaining.
    fn set_medication(self, value: CodeableReference) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the informationSource field and returns self for chaining.
    fn set_information_source(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the informationSource field and returns self for chaining.
    fn add_information_source(self, item: Reference) -> Self;
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
    /// Sets the reported field and returns self for chaining.
    fn set_reported(self, value: bool) -> Self;
    /// Sets the performerType field and returns self for chaining.
    fn set_performer_type(self, value: CodeableConcept) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the performer field and returns self for chaining.
    fn add_performer(self, item: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the device field and returns self for chaining.
    fn add_device(self, item: CodeableReference) -> Self;
    /// Sets the recorder field and returns self for chaining.
    fn set_recorder(self, value: Reference) -> Self;
    /// Sets the reason field and returns self for chaining.
    fn set_reason(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the reason field and returns self for chaining.
    fn add_reason(self, item: CodeableReference) -> Self;
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
    /// Sets the renderedDosageInstruction field and returns self for chaining.
    fn set_rendered_dosage_instruction(self, value: String) -> Self;
    /// Sets the effectiveDosePeriod field and returns self for chaining.
    fn set_effective_dose_period(self, value: Period) -> Self;
    /// Sets the dosageInstruction field and returns self for chaining.
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self;
    /// Adds an item to the dosageInstruction field and returns self for chaining.
    fn add_dosage_instruction(self, item: Dosage) -> Self;
    /// Sets the dispenseRequest field and returns self for chaining.
    fn set_dispense_request(self, value: MedicationRequestDispenserequest) -> Self;
    /// Sets the substitution field and returns self for chaining.
    fn set_substitution(self, value: MedicationRequestSubstitution) -> Self;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationRequestExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the prior_prescription field is present (Some).
    fn has_prior_prescription(&self) -> bool;
    /// Returns true if the group_identifier field is present (Some).
    fn has_group_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is present (Some).
    fn has_status_reason(&self) -> bool;
    /// Returns true if the status_changed field is present (Some).
    fn has_status_changed(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the medication field is present (Some).
    fn has_medication(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the information_source field is not empty.
    fn has_information_source(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
    /// Returns true if the authored_on field is present (Some).
    fn has_authored_on(&self) -> bool;
    /// Returns true if the requester field is present (Some).
    fn has_requester(&self) -> bool;
    /// Returns true if the reported field is present (Some).
    fn has_reported(&self) -> bool;
    /// Returns true if the performer_type field is present (Some).
    fn has_performer_type(&self) -> bool;
    /// Returns true if the performer field is not empty.
    fn has_performer(&self) -> bool;
    /// Returns true if the device field is not empty.
    fn has_device(&self) -> bool;
    /// Returns true if the recorder field is present (Some).
    fn has_recorder(&self) -> bool;
    /// Returns true if the reason field is not empty.
    fn has_reason(&self) -> bool;
    /// Returns true if the course_of_therapy_type field is present (Some).
    fn has_course_of_therapy_type(&self) -> bool;
    /// Returns true if the insurance field is not empty.
    fn has_insurance(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the rendered_dosage_instruction field is present (Some).
    fn has_rendered_dosage_instruction(&self) -> bool;
    /// Returns true if the effective_dose_period field is present (Some).
    fn has_effective_dose_period(&self) -> bool;
    /// Returns true if the dosage_instruction field is not empty.
    fn has_dosage_instruction(&self) -> bool;
    /// Returns true if the dispense_request field is present (Some).
    fn has_dispense_request(&self) -> bool;
    /// Returns true if the substitution field is present (Some).
    fn has_substitution(&self) -> bool;
    /// Returns true if the event_history field is not empty.
    fn has_event_history(&self) -> bool;
}
