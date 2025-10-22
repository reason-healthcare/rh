use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::population::Population;
use crate::datatypes::reference::Reference;
use crate::resources::medicinal_product_contraindication::MedicinalProductContraindicationOthertherapy;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductContraindication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The clinical particulars - indications, contraindications etc. of a medicinal product, including for regulatory purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductContraindication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductContraindication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductContraindicationAccessors: DomainResourceAccessors {
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[Reference];
    /// Returns a reference to the disease field.
    fn disease(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the diseaseStatus field.
    fn disease_status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the comorbidity field.
    fn comorbidity(&self) -> &[CodeableConcept];
    /// Returns a reference to the therapeuticIndication field.
    fn therapeutic_indication(&self) -> &[Reference];
    /// Returns a reference to the otherTherapy field.
    fn other_therapy(&self) -> &[MedicinalProductContraindicationOthertherapy];
    /// Returns a reference to the population field.
    fn population(&self) -> &[Population];
}
/// MedicinalProductContraindication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The clinical particulars - indications, contraindications etc. of a medicinal product, including for regulatory purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductContraindication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductContraindication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductContraindicationMutators: DomainResourceMutators {
    /// Create a new MedicinalProductContraindication with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_contraindication::MedicinalProductContraindication;
    /// use hl7_fhir_r4_core::traits::medicinal_product_contraindication::MedicinalProductContraindicationMutators;
    ///
    /// let resource = MedicinalProductContraindication::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: Reference) -> Self;
    /// Sets the disease field and returns self for chaining.
    fn set_disease(self, value: CodeableConcept) -> Self;
    /// Sets the diseaseStatus field and returns self for chaining.
    fn set_disease_status(self, value: CodeableConcept) -> Self;
    /// Sets the comorbidity field and returns self for chaining.
    fn set_comorbidity(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the comorbidity field and returns self for chaining.
    fn add_comorbidity(self, item: CodeableConcept) -> Self;
    /// Sets the therapeuticIndication field and returns self for chaining.
    fn set_therapeutic_indication(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the therapeuticIndication field and returns self for chaining.
    fn add_therapeutic_indication(self, item: Reference) -> Self;
    /// Sets the otherTherapy field and returns self for chaining.
    fn set_other_therapy(self, value: Vec<MedicinalProductContraindicationOthertherapy>) -> Self;
    /// Adds an item to the otherTherapy field and returns self for chaining.
    fn add_other_therapy(self, item: MedicinalProductContraindicationOthertherapy) -> Self;
    /// Sets the population field and returns self for chaining.
    fn set_population(self, value: Vec<Population>) -> Self;
    /// Adds an item to the population field and returns self for chaining.
    fn add_population(self, item: Population) -> Self;
}
/// MedicinalProductContraindication Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The clinical particulars - indications, contraindications etc. of a medicinal product, including for regulatory purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductContraindication
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductContraindication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductContraindicationExistence: DomainResourceExistence {
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
    /// Returns true if the disease field is present (Some).
    fn has_disease(&self) -> bool;
    /// Returns true if the disease_status field is present (Some).
    fn has_disease_status(&self) -> bool;
    /// Returns true if the comorbidity field is not empty.
    fn has_comorbidity(&self) -> bool;
    /// Returns true if the therapeutic_indication field is not empty.
    fn has_therapeutic_indication(&self) -> bool;
    /// Returns true if the other_therapy field is not empty.
    fn has_other_therapy(&self) -> bool;
    /// Returns true if the population field is not empty.
    fn has_population(&self) -> bool;
}
