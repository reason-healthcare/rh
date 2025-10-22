use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::population::Population;
use crate::datatypes::reference::Reference;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductUndesirableEffect Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describe the undesirable effects of the medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductUndesirableEffect
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductUndesirableEffect
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductUndesirableEffectAccessors: DomainResourceAccessors {
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the symptomConditionEffect field.
    fn symptom_condition_effect(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the classification field.
    fn classification(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the frequencyOfOccurrence field.
    fn frequency_of_occurrence(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the population field.
    fn population(&self) -> &[Population];
}
/// MedicinalProductUndesirableEffect Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describe the undesirable effects of the medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductUndesirableEffect
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductUndesirableEffect
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductUndesirableEffectMutators: DomainResourceMutators {
    /// Create a new MedicinalProductUndesirableEffect with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_undesirable_effect::MedicinalProductUndesirableEffect;
    /// use hl7_fhir_r4_core::traits::medicinal_product_undesirable_effect::MedicinalProductUndesirableEffectMutators;
    ///
    /// let resource = MedicinalProductUndesirableEffect::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the symptomConditionEffect field and returns self for chaining.
    fn set_symptom_condition_effect(self, value: CodeableConcept) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: CodeableConcept) -> Self;
    /// Sets the frequencyOfOccurrence field and returns self for chaining.
    fn set_frequency_of_occurrence(self, value: CodeableConcept) -> Self;
    /// Sets the population field and returns self for chaining.
    fn set_population(self, value: Vec<Population>) -> Self;
    /// Adds an item to the population field and returns self for chaining.
    fn add_population(self, item: Population) -> Self;
}
/// MedicinalProductUndesirableEffect Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describe the undesirable effects of the medicinal product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductUndesirableEffect
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductUndesirableEffect
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductUndesirableEffectExistence: DomainResourceExistence {
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
    /// Returns true if the symptom_condition_effect field is present (Some).
    fn has_symptom_condition_effect(&self) -> bool;
    /// Returns true if the classification field is present (Some).
    fn has_classification(&self) -> bool;
    /// Returns true if the frequency_of_occurrence field is present (Some).
    fn has_frequency_of_occurrence(&self) -> bool;
    /// Returns true if the population field is not empty.
    fn has_population(&self) -> bool;
}
