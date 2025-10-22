use crate::bindings::specimen_status::SpecimenStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::specimen::SpecimenCollection;
use crate::resources::specimen::SpecimenContainer;
use crate::resources::specimen::SpecimenProcessing;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Specimen Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A sample to be used for analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Specimen
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Specimen
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the accessionIdentifier field.
    fn accession_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<SpecimenStatus>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the receivedTime field.
    fn received_time(&self) -> Option<DateTimeType>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> &[Reference];
    /// Returns a reference to the request field.
    fn request(&self) -> &[Reference];
    /// Returns a reference to the collection field.
    fn collection(&self) -> Option<SpecimenCollection>;
    /// Returns a reference to the processing field.
    fn processing(&self) -> &[SpecimenProcessing];
    /// Returns a reference to the container field.
    fn container(&self) -> &[SpecimenContainer];
    /// Returns a reference to the condition field.
    fn condition(&self) -> &[CodeableConcept];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// Specimen Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A sample to be used for analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Specimen
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Specimen
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenMutators: DomainResourceMutators {
    /// Create a new Specimen with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::specimen::Specimen;
    /// use hl7_fhir_r4_core::traits::specimen::SpecimenMutators;
    ///
    /// let resource = Specimen::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the accessionIdentifier field and returns self for chaining.
    fn set_accession_identifier(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SpecimenStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the receivedTime field and returns self for chaining.
    fn set_received_time(self, value: String) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the parent field and returns self for chaining.
    fn add_parent(self, item: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the request field and returns self for chaining.
    fn add_request(self, item: Reference) -> Self;
    /// Sets the collection field and returns self for chaining.
    fn set_collection(self, value: SpecimenCollection) -> Self;
    /// Sets the processing field and returns self for chaining.
    fn set_processing(self, value: Vec<SpecimenProcessing>) -> Self;
    /// Adds an item to the processing field and returns self for chaining.
    fn add_processing(self, item: SpecimenProcessing) -> Self;
    /// Sets the container field and returns self for chaining.
    fn set_container(self, value: Vec<SpecimenContainer>) -> Self;
    /// Adds an item to the container field and returns self for chaining.
    fn add_container(self, item: SpecimenContainer) -> Self;
    /// Sets the condition field and returns self for chaining.
    fn set_condition(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the condition field and returns self for chaining.
    fn add_condition(self, item: CodeableConcept) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// Specimen Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A sample to be used for analysis.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Specimen
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Specimen
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SpecimenExistence: DomainResourceExistence {
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
    /// Returns true if the accession_identifier field is present (Some).
    fn has_accession_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the received_time field is present (Some).
    fn has_received_time(&self) -> bool;
    /// Returns true if the parent field is not empty.
    fn has_parent(&self) -> bool;
    /// Returns true if the request field is not empty.
    fn has_request(&self) -> bool;
    /// Returns true if the collection field is present (Some).
    fn has_collection(&self) -> bool;
    /// Returns true if the processing field is not empty.
    fn has_processing(&self) -> bool;
    /// Returns true if the container field is not empty.
    fn has_container(&self) -> bool;
    /// Returns true if the condition field is not empty.
    fn has_condition(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
