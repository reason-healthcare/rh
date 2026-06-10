use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::document_reference::DocumentReferenceAttester;
use crate::resources::document_reference::DocumentReferenceContent;
use crate::resources::document_reference::DocumentReferenceRelatesto;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DocumentReference Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DocumentReferenceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the status field.
    fn status(&self) -> DocumentReferenceStatus;
    /// Returns a reference to the docStatus field.
    fn doc_status(&self) -> Option<CompositionStatus>;
    /// Returns a reference to the modality field.
    fn modality(&self) -> &[CodeableConcept];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the context field.
    fn context(&self) -> &[Reference];
    /// Returns a reference to the event field.
    fn event(&self) -> &[CodeableReference];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableReference];
    /// Returns a reference to the facilityType field.
    fn facility_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the practiceSetting field.
    fn practice_setting(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the period field.
    fn period(&self) -> Option<Period>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<InstantType>;
    /// Returns a reference to the author field.
    fn author(&self) -> &[Reference];
    /// Returns a reference to the attester field.
    fn attester(&self) -> &[DocumentReferenceAttester];
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
}
/// DocumentReference Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::document_reference::DocumentReference;
    /// use rh_hl7_fhir_r5_core::traits::document_reference::DocumentReferenceMutators;
    ///
    /// let resource = DocumentReference::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DocumentReferenceStatus) -> Self;
    /// Sets the docStatus field and returns self for chaining.
    fn set_doc_status(self, value: CompositionStatus) -> Self;
    /// Sets the modality field and returns self for chaining.
    fn set_modality(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the modality field and returns self for chaining.
    fn add_modality(self, item: CodeableConcept) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the context field and returns self for chaining.
    fn set_context(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the context field and returns self for chaining.
    fn add_context(self, item: Reference) -> Self;
    /// Sets the event field and returns self for chaining.
    fn set_event(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the event field and returns self for chaining.
    fn add_event(self, item: CodeableReference) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the bodySite field and returns self for chaining.
    fn add_body_site(self, item: CodeableReference) -> Self;
    /// Sets the facilityType field and returns self for chaining.
    fn set_facility_type(self, value: CodeableConcept) -> Self;
    /// Sets the practiceSetting field and returns self for chaining.
    fn set_practice_setting(self, value: CodeableConcept) -> Self;
    /// Sets the period field and returns self for chaining.
    fn set_period(self, value: Period) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: Reference) -> Self;
    /// Sets the attester field and returns self for chaining.
    fn set_attester(self, value: Vec<DocumentReferenceAttester>) -> Self;
    /// Adds an item to the attester field and returns self for chaining.
    fn add_attester(self, item: DocumentReferenceAttester) -> Self;
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
}
/// DocumentReference Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 5.0.0
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
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the doc_status field is present (Some).
    fn has_doc_status(&self) -> bool;
    /// Returns true if the modality field is not empty.
    fn has_modality(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the context field is not empty.
    fn has_context(&self) -> bool;
    /// Returns true if the event field is not empty.
    fn has_event(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the facility_type field is present (Some).
    fn has_facility_type(&self) -> bool;
    /// Returns true if the practice_setting field is present (Some).
    fn has_practice_setting(&self) -> bool;
    /// Returns true if the period field is present (Some).
    fn has_period(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the attester field is not empty.
    fn has_attester(&self) -> bool;
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
}
