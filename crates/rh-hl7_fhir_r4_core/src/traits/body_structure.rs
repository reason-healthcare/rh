use crate::datatypes::attachment::Attachment;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// BodyStructure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BodyStructure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BodyStructureAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the morphology field.
    fn morphology(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the location field.
    fn location(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the locationQualifier field.
    fn location_qualifier(&self) -> &[CodeableConcept];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the image field.
    fn image(&self) -> &[Attachment];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
}
/// BodyStructure Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BodyStructure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BodyStructureMutators: DomainResourceMutators {
    /// Create a new BodyStructure with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::body_structure::BodyStructure;
    /// use hl7_fhir_r4_core::traits::body_structure::BodyStructureMutators;
    ///
    /// let resource = BodyStructure::new();
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
    /// Sets the morphology field and returns self for chaining.
    fn set_morphology(self, value: CodeableConcept) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: CodeableConcept) -> Self;
    /// Sets the locationQualifier field and returns self for chaining.
    fn set_location_qualifier(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the locationQualifier field and returns self for chaining.
    fn add_location_qualifier(self, item: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the image field and returns self for chaining.
    fn set_image(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the image field and returns self for chaining.
    fn add_image(self, item: Attachment) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
}
/// BodyStructure Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BodyStructure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BodyStructureExistence: DomainResourceExistence {
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
    /// Returns true if the morphology field is present (Some).
    fn has_morphology(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the location_qualifier field is not empty.
    fn has_location_qualifier(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the image field is not empty.
    fn has_image(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
}
