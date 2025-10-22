use crate::bindings::sequence_type::SequenceType;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::molecular_sequence::MolecularSequenceQuality;
use crate::resources::molecular_sequence::MolecularSequenceReferenceseq;
use crate::resources::molecular_sequence::MolecularSequenceRepository;
use crate::resources::molecular_sequence::MolecularSequenceStructurevariant;
use crate::resources::molecular_sequence::MolecularSequenceVariant;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MolecularSequence Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Raw data describing a biological sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MolecularSequenceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<SequenceType>;
    /// Returns a reference to the coordinateSystem field.
    fn coordinate_system(&self) -> IntegerType;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> Option<Reference>;
    /// Returns a reference to the device field.
    fn device(&self) -> Option<Reference>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<Reference>;
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the referenceSeq field.
    fn reference_seq(&self) -> Option<MolecularSequenceReferenceseq>;
    /// Returns a reference to the variant field.
    fn variant(&self) -> &[MolecularSequenceVariant];
    /// Returns a reference to the observedSeq field.
    fn observed_seq(&self) -> Option<StringType>;
    /// Returns a reference to the quality field.
    fn quality(&self) -> &[MolecularSequenceQuality];
    /// Returns a reference to the readCoverage field.
    fn read_coverage(&self) -> Option<IntegerType>;
    /// Returns a reference to the repository field.
    fn repository(&self) -> &[MolecularSequenceRepository];
    /// Returns a reference to the pointer field.
    fn pointer(&self) -> &[Reference];
    /// Returns a reference to the structureVariant field.
    fn structure_variant(&self) -> &[MolecularSequenceStructurevariant];
}
/// MolecularSequence Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Raw data describing a biological sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 4.0.1
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
    /// use hl7_fhir_r4_core::resources::molecular_sequence::MolecularSequence;
    /// use hl7_fhir_r4_core::traits::molecular_sequence::MolecularSequenceMutators;
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
    /// Sets the coordinateSystem field and returns self for chaining.
    fn set_coordinate_system(self, value: i32) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Reference) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Reference) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the referenceSeq field and returns self for chaining.
    fn set_reference_seq(self, value: MolecularSequenceReferenceseq) -> Self;
    /// Sets the variant field and returns self for chaining.
    fn set_variant(self, value: Vec<MolecularSequenceVariant>) -> Self;
    /// Adds an item to the variant field and returns self for chaining.
    fn add_variant(self, item: MolecularSequenceVariant) -> Self;
    /// Sets the observedSeq field and returns self for chaining.
    fn set_observed_seq(self, value: String) -> Self;
    /// Sets the quality field and returns self for chaining.
    fn set_quality(self, value: Vec<MolecularSequenceQuality>) -> Self;
    /// Adds an item to the quality field and returns self for chaining.
    fn add_quality(self, item: MolecularSequenceQuality) -> Self;
    /// Sets the readCoverage field and returns self for chaining.
    fn set_read_coverage(self, value: i32) -> Self;
    /// Sets the repository field and returns self for chaining.
    fn set_repository(self, value: Vec<MolecularSequenceRepository>) -> Self;
    /// Adds an item to the repository field and returns self for chaining.
    fn add_repository(self, item: MolecularSequenceRepository) -> Self;
    /// Sets the pointer field and returns self for chaining.
    fn set_pointer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the pointer field and returns self for chaining.
    fn add_pointer(self, item: Reference) -> Self;
    /// Sets the structureVariant field and returns self for chaining.
    fn set_structure_variant(self, value: Vec<MolecularSequenceStructurevariant>) -> Self;
    /// Adds an item to the structureVariant field and returns self for chaining.
    fn add_structure_variant(self, item: MolecularSequenceStructurevariant) -> Self;
}
/// MolecularSequence Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Raw data describing a biological sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 4.0.1
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
    /// Returns true if the coordinate_system field is present (Some).
    fn has_coordinate_system(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the specimen field is present (Some).
    fn has_specimen(&self) -> bool;
    /// Returns true if the device field is present (Some).
    fn has_device(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the reference_seq field is present (Some).
    fn has_reference_seq(&self) -> bool;
    /// Returns true if the variant field is not empty.
    fn has_variant(&self) -> bool;
    /// Returns true if the observed_seq field is present (Some).
    fn has_observed_seq(&self) -> bool;
    /// Returns true if the quality field is not empty.
    fn has_quality(&self) -> bool;
    /// Returns true if the read_coverage field is present (Some).
    fn has_read_coverage(&self) -> bool;
    /// Returns true if the repository field is not empty.
    fn has_repository(&self) -> bool;
    /// Returns true if the pointer field is not empty.
    fn has_pointer(&self) -> bool;
    /// Returns true if the structure_variant field is not empty.
    fn has_structure_variant(&self) -> bool;
}
