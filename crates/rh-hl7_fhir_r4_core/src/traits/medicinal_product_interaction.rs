use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::medicinal_product_interaction::MedicinalProductInteractionInteractant;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductInteraction Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The interactions of the medicinal product with other medicinal products, or other forms of interactions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductInteraction
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductInteraction
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductInteractionAccessors: DomainResourceAccessors {
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the interactant field.
    fn interactant(&self) -> &[MedicinalProductInteractionInteractant];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the effect field.
    fn effect(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the incidence field.
    fn incidence(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the management field.
    fn management(&self) -> Option<CodeableConcept>;
}
/// MedicinalProductInteraction Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The interactions of the medicinal product with other medicinal products, or other forms of interactions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductInteraction
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductInteraction
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductInteractionMutators: DomainResourceMutators {
    /// Create a new MedicinalProductInteraction with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_interaction::MedicinalProductInteraction;
    /// use hl7_fhir_r4_core::traits::medicinal_product_interaction::MedicinalProductInteractionMutators;
    ///
    /// let resource = MedicinalProductInteraction::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the interactant field and returns self for chaining.
    fn set_interactant(self, value: Vec<MedicinalProductInteractionInteractant>) -> Self;
    /// Adds an item to the interactant field and returns self for chaining.
    fn add_interactant(self, item: MedicinalProductInteractionInteractant) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the effect field and returns self for chaining.
    fn set_effect(self, value: CodeableConcept) -> Self;
    /// Sets the incidence field and returns self for chaining.
    fn set_incidence(self, value: CodeableConcept) -> Self;
    /// Sets the management field and returns self for chaining.
    fn set_management(self, value: CodeableConcept) -> Self;
}
/// MedicinalProductInteraction Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The interactions of the medicinal product with other medicinal products, or other forms of interactions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductInteraction
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductInteraction
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductInteractionExistence: DomainResourceExistence {
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
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the interactant field is not empty.
    fn has_interactant(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the effect field is present (Some).
    fn has_effect(&self) -> bool;
    /// Returns true if the incidence field is present (Some).
    fn has_incidence(&self) -> bool;
    /// Returns true if the management field is present (Some).
    fn has_management(&self) -> bool;
}
