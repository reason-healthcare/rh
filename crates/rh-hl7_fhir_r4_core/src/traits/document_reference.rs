use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::document_reference::DocumentReferenceContent;
use crate::resources::document_reference::DocumentReferenceContext;
use crate::resources::document_reference::DocumentReferenceRelatesto;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DocumentReference Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. Provides metadata about the document so that the document can be discovered and managed. The scope of a document is any seralized object with a mime-type, so includes formal patient centric documents (CDA), cliical notes, scanned paper, and non-patient specific documents like policy text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentReferenceAccessors: DomainResourceAccessors {
    /// Returns a reference to the masterIdentifier field.
    fn master_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> DocumentReferenceStatus;
    /// Returns a reference to the docStatus field.
    fn doc_status(&self) -> Option<CompositionStatus>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<InstantType>;
    /// Returns a reference to the author field.
    fn author(&self) -> &[Reference];
    /// Returns a reference to the authenticator field.
    fn authenticator(&self) -> Option<Reference>;
    /// Returns a reference to the custodian field.
    fn custodian(&self) -> Option<Reference>;
    /// Returns a reference to the relatesTo field.
    fn relates_to(&self) -> &[DocumentReferenceRelatesto];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the securityLabel field.
    fn security_label(&self) -> &[CodeableConcept];
    /// Returns a reference to the content field.
    fn content(&self) -> &[DocumentReferenceContent];
    /// Returns a reference to the context field.
    fn context(&self) -> Option<DocumentReferenceContext>;
}
/// DocumentReference Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. Provides metadata about the document so that the document can be discovered and managed. The scope of a document is any seralized object with a mime-type, so includes formal patient centric documents (CDA), cliical notes, scanned paper, and non-patient specific documents like policy text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentReferenceMutators: DomainResourceMutators {
    /// Create a new DocumentReference with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::document_reference::DocumentReference;
    /// use hl7_fhir_r4_core::traits::document_reference::DocumentReferenceMutators;
    ///
    /// let resource = DocumentReference::new();
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
    /// Sets the docStatus field and returns self for chaining.
    fn set_doc_status(self, value: CompositionStatus) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: Reference) -> Self;
    /// Sets the authenticator field and returns self for chaining.
    fn set_authenticator(self, value: Reference) -> Self;
    /// Sets the custodian field and returns self for chaining.
    fn set_custodian(self, value: Reference) -> Self;
    /// Sets the relatesTo field and returns self for chaining.
    fn set_relates_to(self, value: Vec<DocumentReferenceRelatesto>) -> Self;
    /// Adds an item to the relatesTo field and returns self for chaining.
    fn add_relates_to(self, item: DocumentReferenceRelatesto) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the securityLabel field and returns self for chaining.
    fn set_security_label(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the securityLabel field and returns self for chaining.
    fn add_security_label(self, item: CodeableConcept) -> Self;
    /// Sets the content field and returns self for chaining.
    fn set_content(self, value: Vec<DocumentReferenceContent>) -> Self;
    /// Adds an item to the content field and returns self for chaining.
    fn add_content(self, item: DocumentReferenceContent) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: DocumentReferenceContext) -> Self;
}
/// DocumentReference Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. Provides metadata about the document so that the document can be discovered and managed. The scope of a document is any seralized object with a mime-type, so includes formal patient centric documents (CDA), cliical notes, scanned paper, and non-patient specific documents like policy text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentReferenceExistence: DomainResourceExistence {
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
    /// Returns true if the doc_status field is present (Some).
    fn has_doc_status(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the authenticator field is present (Some).
    fn has_authenticator(&self) -> bool;
    /// Returns true if the custodian field is present (Some).
    fn has_custodian(&self) -> bool;
    /// Returns true if the relates_to field is not empty.
    fn has_relates_to(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the security_label field is not empty.
    fn has_security_label(&self) -> bool;
    /// Returns true if the content field is not empty.
    fn has_content(&self) -> bool;
    /// Returns true if the context field is present (Some).
    fn has_context(&self) -> bool;
}
