use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::population::Population;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::resources::medicinal_product_indication::MedicinalProductIndicationOthertherapy;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductIndication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indication for the Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIndication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIndication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIndicationAccessors: DomainResourceAccessors {
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the diseaseSymptomProcedure field.
    fn disease_symptom_procedure(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the diseaseStatus field.
    fn disease_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the comorbidity field.
    fn comorbidity(&self) -> &[CodeableConcept];
    /// Returns a reference to the intendedEffect field.
    fn intended_effect(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the duration field.
    fn duration(&self) -> Option<Quantity>;
    /// Returns a reference to the otherTherapy field.
    fn other_therapy(&self) -> &[MedicinalProductIndicationOthertherapy];
    /// Returns a reference to the undesirableEffect field.
    fn undesirable_effect(&self) -> &[Reference];
    /// Returns a reference to the population field.
    fn population(&self) -> &[Population];
}
/// MedicinalProductIndication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Indication for the Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIndication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIndication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIndicationMutators: DomainResourceMutators {
    /// Create a new MedicinalProductIndication with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_indication::MedicinalProductIndication;
    /// use hl7_fhir_r4_core::traits::medicinal_product_indication::MedicinalProductIndicationMutators;
    ///
    /// let resource = MedicinalProductIndication::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the diseaseSymptomProcedure field and returns self for chaining.
    fn set_disease_symptom_procedure(self, value: CodeableConcept) -> Self;
    /// Sets the diseaseStatus field and returns self for chaining.
    fn set_disease_status(self, value: CodeableConcept) -> Self;
    /// Sets the comorbidity field and returns self for chaining.
    fn set_comorbidity(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the comorbidity field and returns self for chaining.
    fn add_comorbidity(self, item: CodeableConcept) -> Self;
    /// Sets the intendedEffect field and returns self for chaining.
    fn set_intended_effect(self, value: CodeableConcept) -> Self;
    /// Sets the duration field and returns self for chaining.
    fn set_duration(self, value: Quantity) -> Self;
    /// Sets the otherTherapy field and returns self for chaining.
    fn set_other_therapy(self, value: Vec<MedicinalProductIndicationOthertherapy>) -> Self;
    /// Adds an item to the otherTherapy field and returns self for chaining.
    fn add_other_therapy(self, item: MedicinalProductIndicationOthertherapy) -> Self;
    /// Sets the undesirableEffect field and returns self for chaining.
    fn set_undesirable_effect(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the undesirableEffect field and returns self for chaining.
    fn add_undesirable_effect(self, item: Reference) -> Self;
    /// Sets the population field and returns self for chaining.
    fn set_population(self, value: Vec<Population>) -> Self;
    /// Adds an item to the population field and returns self for chaining.
    fn add_population(self, item: Population) -> Self;
}
/// MedicinalProductIndication Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Indication for the Medicinal Product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIndication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIndication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIndicationExistence: DomainResourceExistence {
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
    /// Returns true if the disease_symptom_procedure field is present (Some).
    fn has_disease_symptom_procedure(&self) -> bool;
    /// Returns true if the disease_status field is present (Some).
    fn has_disease_status(&self) -> bool;
    /// Returns true if the comorbidity field is not empty.
    fn has_comorbidity(&self) -> bool;
    /// Returns true if the intended_effect field is present (Some).
    fn has_intended_effect(&self) -> bool;
    /// Returns true if the duration field is present (Some).
    fn has_duration(&self) -> bool;
    /// Returns true if the other_therapy field is not empty.
    fn has_other_therapy(&self) -> bool;
    /// Returns true if the undesirable_effect field is not empty.
    fn has_undesirable_effect(&self) -> bool;
    /// Returns true if the population field is not empty.
    fn has_population(&self) -> bool;
}
