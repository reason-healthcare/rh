use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Schedule Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A container for slots of time that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Schedule
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Schedule
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ScheduleAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the serviceCategory field.
    fn service_category(&self) -> &[CodeableConcept];
    /// Returns a reference to the serviceType field.
    fn service_type(&self) -> &[CodeableConcept];
    /// Returns a reference to the specialty field.
    fn specialty(&self) -> &[CodeableConcept];
    /// Returns a reference to the actor field.
    fn actor(&self) -> &[Reference];
    /// Returns a reference to the planningHorizon field.
    fn planning_horizon(&self) -> Option<Period>;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
}
/// Schedule Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A container for slots of time that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Schedule
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Schedule
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ScheduleMutators: DomainResourceMutators {
    /// Create a new Schedule with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::schedule::Schedule;
    /// use hl7_fhir_r4_core::traits::schedule::ScheduleMutators;
    ///
    /// let resource = Schedule::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the active field and returns self for chaining.
    fn set_active(self, value: bool) -> Self;
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
    /// Sets the actor field and returns self for chaining.
    fn set_actor(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the actor field and returns self for chaining.
    fn add_actor(self, item: Reference) -> Self;
    /// Sets the planningHorizon field and returns self for chaining.
    fn set_planning_horizon(self, value: Period) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
}
/// Schedule Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A container for slots of time that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Schedule
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Schedule
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ScheduleExistence: DomainResourceExistence {
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
    /// Returns true if the active field is present (Some).
    fn has_active(&self) -> bool;
    /// Returns true if the service_category field is not empty.
    fn has_service_category(&self) -> bool;
    /// Returns true if the service_type field is not empty.
    fn has_service_type(&self) -> bool;
    /// Returns true if the specialty field is not empty.
    fn has_specialty(&self) -> bool;
    /// Returns true if the actor field is not empty.
    fn has_actor(&self) -> bool;
    /// Returns true if the planning_horizon field is present (Some).
    fn has_planning_horizon(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
}
