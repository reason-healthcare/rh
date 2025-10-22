use crate::bindings::fm_status::FmStatus;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::vision_prescription::VisionPrescriptionLensspecification;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// VisionPrescription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An authorization for the provision of glasses and/or contact lenses to a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VisionPrescription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VisionPrescriptionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> FmStatus;
    /// Returns a reference to the created field.
    fn created(&self) -> DateTimeType;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the dateWritten field.
    fn date_written(&self) -> DateTimeType;
    /// Returns a reference to the prescriber field.
    fn prescriber(&self) -> Reference;
    /// Returns a reference to the lensSpecification field.
    fn lens_specification(&self) -> &[VisionPrescriptionLensspecification];
}
/// VisionPrescription Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An authorization for the provision of glasses and/or contact lenses to a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VisionPrescription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VisionPrescriptionMutators: DomainResourceMutators {
    /// Create a new VisionPrescription with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::vision_prescription::VisionPrescription;
    /// use hl7_fhir_r4_core::traits::vision_prescription::VisionPrescriptionMutators;
    ///
    /// let resource = VisionPrescription::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FmStatus) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the dateWritten field and returns self for chaining.
    fn set_date_written(self, value: String) -> Self;
    /// Sets the prescriber field and returns self for chaining.
    fn set_prescriber(self, value: Reference) -> Self;
    /// Sets the lensSpecification field and returns self for chaining.
    fn set_lens_specification(self, value: Vec<VisionPrescriptionLensspecification>) -> Self;
    /// Adds an item to the lensSpecification field and returns self for chaining.
    fn add_lens_specification(self, item: VisionPrescriptionLensspecification) -> Self;
}
/// VisionPrescription Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An authorization for the provision of glasses and/or contact lenses to a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VisionPrescription
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VisionPrescription
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VisionPrescriptionExistence: DomainResourceExistence {
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
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the date_written field is present (Some).
    fn has_date_written(&self) -> bool;
    /// Returns true if the prescriber field is present (Some).
    fn has_prescriber(&self) -> bool;
    /// Returns true if the lens_specification field is not empty.
    fn has_lens_specification(&self) -> bool;
}
