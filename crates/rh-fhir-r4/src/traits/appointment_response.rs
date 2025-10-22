use crate::bindings::participationstatus::Participationstatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// AppointmentResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AppointmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the appointment field.
    fn appointment(&self) -> Reference;
    /// Returns a reference to the start field.
    fn start(&self) -> Option<InstantType>;
    /// Returns a reference to the end field.
    fn end(&self) -> Option<InstantType>;
    /// Returns a reference to the participantType field.
    fn participant_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the actor field.
    fn actor(&self) -> Option<Reference>;
    /// Returns a reference to the participantStatus field.
    fn participant_status(&self) -> Participationstatus;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
}
/// AppointmentResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AppointmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentResponseMutators: DomainResourceMutators {
    /// Create a new AppointmentResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::appointment_response::AppointmentResponse;
    /// use hl7_fhir_r4_core::traits::appointment_response::AppointmentResponseMutators;
    ///
    /// let resource = AppointmentResponse::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the appointment field and returns self for chaining.
    fn set_appointment(self, value: Reference) -> Self;
    /// Sets the start field and returns self for chaining.
    fn set_start(self, value: String) -> Self;
    /// Sets the end field and returns self for chaining.
    fn set_end(self, value: String) -> Self;
    /// Sets the participantType field and returns self for chaining.
    fn set_participant_type(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the participantType field and returns self for chaining.
    fn add_participant_type(self, item: CodeableConcept) -> Self;
    /// Sets the actor field and returns self for chaining.
    fn set_actor(self, value: Reference) -> Self;
    /// Sets the participantStatus field and returns self for chaining.
    fn set_participant_status(self, value: Participationstatus) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
}
/// AppointmentResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AppointmentResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait AppointmentResponseExistence: DomainResourceExistence {
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
    /// Returns true if the appointment field is present (Some).
    fn has_appointment(&self) -> bool;
    /// Returns true if the start field is present (Some).
    fn has_start(&self) -> bool;
    /// Returns true if the end field is present (Some).
    fn has_end(&self) -> bool;
    /// Returns true if the participant_type field is not empty.
    fn has_participant_type(&self) -> bool;
    /// Returns true if the actor field is present (Some).
    fn has_actor(&self) -> bool;
    /// Returns true if the participant_status field is present (Some).
    fn has_participant_status(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
}
