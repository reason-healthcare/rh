use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date::DateType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Basic Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Basic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Basic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BasicAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateType>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
}
/// Basic Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Basic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Basic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BasicMutators: DomainResourceMutators {
    /// Create a new Basic with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::basic::Basic;
    /// use hl7_fhir_r4_core::traits::basic::BasicMutators;
    ///
    /// let resource = Basic::new();
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
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
}
/// Basic Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Basic
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Basic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BasicExistence: DomainResourceExistence {
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
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
}
