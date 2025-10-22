use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::document_manifest::DocumentManifestRelated;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DocumentManifest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A collection of documents compiled for a purpose together with metadata that applies to the collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentManifest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentManifest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentManifestAccessors: DomainResourceAccessors {
    /// Returns a reference to the masterIdentifier field.
    fn master_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> DocumentReferenceStatus;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the created field.
    fn created(&self) -> Option<DateTimeType>;
    /// Returns a reference to the author field.
    fn author(&self) -> &[Reference];
    /// Returns a reference to the recipient field.
    fn recipient(&self) -> &[Reference];
    /// Returns a reference to the source field.
    fn source(&self) -> Option<StringType>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the content field.
    fn content(&self) -> &[Reference];
    /// Returns a reference to the related field.
    fn related(&self) -> &[DocumentManifestRelated];
}
/// DocumentManifest Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A collection of documents compiled for a purpose together with metadata that applies to the collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentManifest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentManifest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentManifestMutators: DomainResourceMutators {
    /// Create a new DocumentManifest with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::document_manifest::DocumentManifest;
    /// use hl7_fhir_r4_core::traits::document_manifest::DocumentManifestMutators;
    ///
    /// let resource = DocumentManifest::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the masterIdentifier field and returns self for chaining.
    fn set_master_identifier(self, value: Identifier) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DocumentReferenceStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the created field and returns self for chaining.
    fn set_created(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: Reference) -> Self;
    /// Sets the recipient field and returns self for chaining.
    fn set_recipient(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the recipient field and returns self for chaining.
    fn add_recipient(self, item: Reference) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: String) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the content field and returns self for chaining.
    fn set_content(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the content field and returns self for chaining.
    fn add_content(self, item: Reference) -> Self;
    /// Sets the related field and returns self for chaining.
    fn set_related(self, value: Vec<DocumentManifestRelated>) -> Self;
    /// Adds an item to the related field and returns self for chaining.
    fn add_related(self, item: DocumentManifestRelated) -> Self;
}
/// DocumentManifest Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A collection of documents compiled for a purpose together with metadata that applies to the collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentManifest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentManifest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentManifestExistence: DomainResourceExistence {
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
    /// Returns true if the master_identifier field is present (Some).
    fn has_master_identifier(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the created field is present (Some).
    fn has_created(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the recipient field is not empty.
    fn has_recipient(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the content field is not empty.
    fn has_content(&self) -> bool;
    /// Returns true if the related field is not empty.
    fn has_related(&self) -> bool;
}
