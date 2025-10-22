use crate::bindings::flag_status::FlagStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Flag Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Prospective warnings of potential issues when providing care to the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Flag
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Flag
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FlagAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FlagStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Reference;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
}
/// Flag Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Prospective warnings of potential issues when providing care to the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Flag
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Flag
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FlagMutators: DomainResourceMutators {
    /// Create a new Flag with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::flag::Flag;
    /// use hl7_fhir_r4_core::traits::flag::FlagMutators;
    ///
    /// let resource = Flag::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FlagStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
}
/// Flag Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Prospective warnings of potential issues when providing care to the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Flag
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Flag
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FlagExistence: DomainResourceExistence {
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
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
}
