use crate::bindings::slotstatus::Slotstatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Slot Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A slot of time on a schedule that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Slot
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Slot
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SlotAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the serviceCategory field.
    fn service_category(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialty field.
    fn specialty(&self) -> &[CodeableConcept];
    /// Returns a reference to the appointmentType field.
    fn appointment_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the schedule field.
    fn schedule(&self) -> Reference;
    /// Returns a reference to the status field.
    fn status(&self) -> Slotstatus;
    /// Returns a reference to the start field.
    fn start(&self) -> InstantType;
    /// Returns a reference to the end field.
    fn end(&self) -> InstantType;
    /// Returns a reference to the overbooked field.
    fn overbooked(&self) -> Option<BooleanType>;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
}
/// Slot Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A slot of time on a schedule that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Slot
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Slot
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SlotMutators: DomainResourceMutators {
    /// Create a new Slot with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::slot::Slot;
    /// use hl7_fhir_r4_core::traits::slot::SlotMutators;
    ///
    /// let resource = Slot::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
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
    /// Sets the schedule field and returns self for chaining.
    fn set_schedule(self, value: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: Slotstatus) -> Self;
    /// Sets the start field and returns self for chaining.
    fn set_start(self, value: String) -> Self;
    /// Sets the end field and returns self for chaining.
    fn set_end(self, value: String) -> Self;
    /// Sets the overbooked field and returns self for chaining.
    fn set_overbooked(self, value: bool) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
}
/// Slot Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A slot of time on a schedule that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Slot
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Slot
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SlotExistence: DomainResourceExistence {
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
    /// Returns true if the service_category field is not empty.
    fn has_service_category(&self) -> bool;
    /// Returns true if the service_type field is not empty.
    fn has_service_type(&self) -> bool;
    /// Returns true if the specialty field is not empty.
    fn has_specialty(&self) -> bool;
    /// Returns true if the appointment_type field is present (Some).
    fn has_appointment_type(&self) -> bool;
    /// Returns true if the schedule field is present (Some).
    fn has_schedule(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the start field is present (Some).
    fn has_start(&self) -> bool;
    /// Returns true if the end field is present (Some).
    fn has_end(&self) -> bool;
    /// Returns true if the overbooked field is present (Some).
    fn has_overbooked(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
}
