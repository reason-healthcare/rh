use crate::bindings::sequence_type::SequenceType;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::molecular_sequence::MolecularSequenceRelative;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MolecularSequence Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Representation of a molecular sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MolecularSequenceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<SequenceType>;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the focus field.
    fn focus(&self) -> &[Reference];
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<Reference>;
    /// Returns a reference to the literal field.
    fn literal(&self) -> Option<StringType>;
    /// Returns a reference to the formatted field.
    fn formatted(&self) -> &[Attachment];
    /// Returns a reference to the relative field.
    fn relative(&self) -> &[MolecularSequenceRelative];
}
/// MolecularSequence Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Representation of a molecular sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MolecularSequenceMutators: DomainResourceMutators {
    /// Create a new MolecularSequence with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::molecular_sequence::MolecularSequence;
    /// use rh_hl7_fhir_r5_core::traits::molecular_sequence::MolecularSequenceMutators;
    ///
    /// let resource = MolecularSequence::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: SequenceType) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the focus field and returns self for chaining.
    fn set_focus(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the focus field and returns self for chaining.
    fn add_focus(self, item: Reference) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Reference) -> Self;
    /// Sets the literal field and returns self for chaining.
    fn set_literal(self, value: String) -> Self;
    /// Sets the formatted field and returns self for chaining.
    fn set_formatted(self, value: Vec<Attachment>) -> Self;
    /// Adds an item to the formatted field and returns self for chaining.
    fn add_formatted(self, item: Attachment) -> Self;
    /// Sets the relative field and returns self for chaining.
    fn set_relative(self, value: Vec<MolecularSequenceRelative>) -> Self;
    /// Adds an item to the relative field and returns self for chaining.
    fn add_relative(self, item: MolecularSequenceRelative) -> Self;
}
/// MolecularSequence Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Representation of a molecular sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MolecularSequenceExistence: DomainResourceExistence {
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
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the focus field is not empty.
    fn has_focus(&self) -> bool;
    /// Returns true if the specimen field is present (Some).
    fn has_specimen(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the literal field is present (Some).
    fn has_literal(&self) -> bool;
    /// Returns true if the formatted field is not empty.
    fn has_formatted(&self) -> bool;
    /// Returns true if the relative field is not empty.
    fn has_relative(&self) -> bool;
}
