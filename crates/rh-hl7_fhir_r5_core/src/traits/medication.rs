use crate::bindings::medication_status::MedicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::resources::medication::MedicationBatch;
use crate::resources::medication::MedicationIngredient;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Medication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Medication
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Medication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<MedicationStatus>;
    /// Returns a reference to the marketingAuthorizationHolder field.
    fn marketing_authorization_holder(&self) -> Option<Reference>;
    /// Returns a reference to the doseForm field.
    fn dose_form(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the totalVolume field.
    fn total_volume(&self) -> Option<Quantity>;
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[MedicationIngredient];
    /// Returns a reference to the batch field.
    fn batch(&self) -> Option<MedicationBatch>;
    /// Returns a reference to the definition field.
    fn definition(&self) -> Option<Reference>;
}
/// Medication Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Medication
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Medication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationMutators: DomainResourceMutators {
    /// Create a new Medication with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::medication::Medication;
    /// use rh_hl7_fhir_r5_core::traits::medication::MedicationMutators;
    ///
    /// let resource = Medication::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: MedicationStatus) -> Self;
    /// Sets the marketingAuthorizationHolder field and returns self for chaining.
    fn set_marketing_authorization_holder(self, value: Reference) -> Self;
    /// Sets the doseForm field and returns self for chaining.
    fn set_dose_form(self, value: CodeableConcept) -> Self;
    /// Sets the totalVolume field and returns self for chaining.
    fn set_total_volume(self, value: Quantity) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<MedicationIngredient>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: MedicationIngredient) -> Self;
    /// Sets the batch field and returns self for chaining.
    fn set_batch(self, value: MedicationBatch) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: Reference) -> Self;
}
/// Medication Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Medication
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Medication
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicationExistence: DomainResourceExistence {
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the marketing_authorization_holder field is present (Some).
    fn has_marketing_authorization_holder(&self) -> bool;
    /// Returns true if the dose_form field is present (Some).
    fn has_dose_form(&self) -> bool;
    /// Returns true if the total_volume field is present (Some).
    fn has_total_volume(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the batch field is present (Some).
    fn has_batch(&self) -> bool;
    /// Returns true if the definition field is present (Some).
    fn has_definition(&self) -> bool;
}
