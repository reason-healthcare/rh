use crate::bindings::appointmentstatus::Appointmentstatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::appointment::AppointmentParticipant;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Appointment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Appointment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Appointment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Appointmentstatus;
    /// Returns a reference to the cancelationReason field.
    fn cancelation_reason(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the serviceCategory field.
    fn service_category(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialty field.
    fn specialty(&self) -> &[CodeableConcept];
    /// Returns a reference to the appointmentType field.
    fn appointment_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the supportingInformation field.
    fn supporting_information(&self) -> &[Reference];
    /// Returns a reference to the start field.
    fn start(&self) -> Option<InstantType>;
    /// Returns a reference to the end field.
    fn end(&self) -> Option<InstantType>;
    /// Returns a reference to the minutesDuration field.
    fn minutes_duration(&self) -> Option<PositiveIntType>;
    /// Returns a reference to the slot field.
    fn slot(&self) -> &[Reference];
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateTimeType>;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the patientInstruction field.
    fn patient_instruction(&self) -> Option<StringType>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[AppointmentParticipant];
    /// Returns a reference to the requestedPeriod field.
    fn requested_period(&self) -> &[Period];
}
/// Appointment Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Appointment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Appointment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentMutators: DomainResourceMutators {
    /// Create a new Appointment with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::appointment::Appointment;
    /// use hl7_fhir_r4_core::traits::appointment::AppointmentMutators;
    ///
    /// let resource = Appointment::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: Appointmentstatus) -> Self;
    /// Sets the cancelationReason field and returns self for chaining.
    fn set_cancelation_reason(self, value: CodeableConcept) -> Self;
    /// Sets the serviceCategory field and returns self for chaining.
    fn set_service_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the serviceCategory field and returns self for chaining.
    fn add_service_category(self, item: CodeableConcept) -> Self;
    /// Sets the serviceType field and returns self for chaining.
    fn set_service_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the serviceType field and returns self for chaining.
    fn add_service_type(self, item: CodeableConcept) -> Self;
    /// Sets the specialty field and returns self for chaining.
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the specialty field and returns self for chaining.
    fn add_specialty(self, item: CodeableConcept) -> Self;
    /// Sets the appointmentType field and returns self for chaining.
    fn set_appointment_type(self, value: CodeableConcept) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: i32) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the supportingInformation field and returns self for chaining.
    fn set_supporting_information(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supportingInformation field and returns self for chaining.
    fn add_supporting_information(self, item: Reference) -> Self;
    /// Sets the start field and returns self for chaining.
    fn set_start(self, value: String) -> Self;
    /// Sets the end field and returns self for chaining.
    fn set_end(self, value: String) -> Self;
    /// Sets the minutesDuration field and returns self for chaining.
    fn set_minutes_duration(self, value: i32) -> Self;
    /// Sets the slot field and returns self for chaining.
    fn set_slot(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the slot field and returns self for chaining.
    fn add_slot(self, item: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the patientInstruction field and returns self for chaining.
    fn set_patient_instruction(self, value: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<AppointmentParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: AppointmentParticipant) -> Self;
    /// Sets the requestedPeriod field and returns self for chaining.
    fn set_requested_period(self, value: Vec<Period>) -> Self;
    /// Adds an item to the requestedPeriod field and returns self for chaining.
    fn add_requested_period(self, item: Period) -> Self;
}
/// Appointment Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Appointment
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Appointment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentExistence: DomainResourceExistence {
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
    /// Returns true if the cancelation_reason field is present (Some).
    fn has_cancelation_reason(&self) -> bool;
    /// Returns true if the service_category field is not empty.
    fn has_service_category(&self) -> bool;
    /// Returns true if the service_type field is not empty.
    fn has_service_type(&self) -> bool;
    /// Returns true if the specialty field is not empty.
    fn has_specialty(&self) -> bool;
    /// Returns true if the appointment_type field is present (Some).
    fn has_appointment_type(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the supporting_information field is not empty.
    fn has_supporting_information(&self) -> bool;
    /// Returns true if the start field is present (Some).
    fn has_start(&self) -> bool;
    /// Returns true if the end field is present (Some).
    fn has_end(&self) -> bool;
    /// Returns true if the minutes_duration field is present (Some).
    fn has_minutes_duration(&self) -> bool;
    /// Returns true if the slot field is not empty.
    fn has_slot(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the patient_instruction field is present (Some).
    fn has_patient_instruction(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the requested_period field is not empty.
    fn has_requested_period(&self) -> bool;
}
