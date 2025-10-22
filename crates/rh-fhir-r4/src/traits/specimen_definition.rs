use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use crate::resources::specimen_definition::SpecimenDefinitionTypetested;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SpecimenDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the typeCollected field.
    fn type_collected(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the patientPreparation field.
    fn patient_preparation(&self) -> &[CodeableConcept];
    /// Returns a reference to the timeAspect field.
    fn time_aspect(&self) -> Option<StringType>;
    /// Returns a reference to the collection field.
    fn collection(&self) -> &[CodeableConcept];
    /// Returns a reference to the typeTested field.
    fn type_tested(&self) -> &[SpecimenDefinitionTypetested];
}
/// SpecimenDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionMutators: DomainResourceMutators {
    /// Create a new SpecimenDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::specimen_definition::SpecimenDefinition;
    /// use hl7_fhir_r4_core::traits::specimen_definition::SpecimenDefinitionMutators;
    ///
    /// let resource = SpecimenDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the typeCollected field and returns self for chaining.
    fn set_type_collected(self, value: CodeableConcept) -> Self;
    /// Sets the patientPreparation field and returns self for chaining.
    fn set_patient_preparation(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the patientPreparation field and returns self for chaining.
    fn add_patient_preparation(self, item: CodeableConcept) -> Self;
    /// Sets the timeAspect field and returns self for chaining.
    fn set_time_aspect(self, value: String) -> Self;
    /// Sets the collection field and returns self for chaining.
    fn set_collection(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the collection field and returns self for chaining.
    fn add_collection(self, item: CodeableConcept) -> Self;
    /// Sets the typeTested field and returns self for chaining.
    fn set_type_tested(self, value: Vec<SpecimenDefinitionTypetested>) -> Self;
    /// Adds an item to the typeTested field and returns self for chaining.
    fn add_type_tested(self, item: SpecimenDefinitionTypetested) -> Self;
}
/// SpecimenDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A kind of specimen with associated set of requirements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SpecimenDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_collected field is present (Some).
    fn has_type_collected(&self) -> bool;
    /// Returns true if the patient_preparation field is not empty.
    fn has_patient_preparation(&self) -> bool;
    /// Returns true if the time_aspect field is present (Some).
    fn has_time_aspect(&self) -> bool;
    /// Returns true if the collection field is not empty.
    fn has_collection(&self) -> bool;
    /// Returns true if the type_tested field is not empty.
    fn has_type_tested(&self) -> bool;
}
