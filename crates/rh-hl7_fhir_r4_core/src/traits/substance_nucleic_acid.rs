use crate::datatypes::codeable_concept::CodeableConcept;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::substance_nucleic_acid::SubstanceNucleicAcidSubunit;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceNucleicAcid Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceNucleicAcid
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceNucleicAcidAccessors: DomainResourceAccessors {
    /// Returns a reference to the sequenceType field.
    fn sequence_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the numberOfSubunits field.
    fn number_of_subunits(&self) -> Option<IntegerType>;
    /// Returns a reference to the areaOfHybridisation field.
    fn area_of_hybridisation(&self) -> Option<StringType>;
    /// Returns a reference to the oligoNucleotideType field.
    fn oligo_nucleotide_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the subunit field.
    fn subunit(&self) -> &[SubstanceNucleicAcidSubunit];
}
/// SubstanceNucleicAcid Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceNucleicAcid
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceNucleicAcidMutators: DomainResourceMutators {
    /// Create a new SubstanceNucleicAcid with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance_nucleic_acid::SubstanceNucleicAcid;
    /// use hl7_fhir_r4_core::traits::substance_nucleic_acid::SubstanceNucleicAcidMutators;
    ///
    /// let resource = SubstanceNucleicAcid::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the sequenceType field and returns self for chaining.
    fn set_sequence_type(self, value: CodeableConcept) -> Self;
    /// Sets the numberOfSubunits field and returns self for chaining.
    fn set_number_of_subunits(self, value: i32) -> Self;
    /// Sets the areaOfHybridisation field and returns self for chaining.
    fn set_area_of_hybridisation(self, value: String) -> Self;
    /// Sets the oligoNucleotideType field and returns self for chaining.
    fn set_oligo_nucleotide_type(self, value: CodeableConcept) -> Self;
    /// Sets the subunit field and returns self for chaining.
    fn set_subunit(self, value: Vec<SubstanceNucleicAcidSubunit>) -> Self;
    /// Adds an item to the subunit field and returns self for chaining.
    fn add_subunit(self, item: SubstanceNucleicAcidSubunit) -> Self;
}
/// SubstanceNucleicAcid Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceNucleicAcid
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceNucleicAcidExistence: DomainResourceExistence {
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
    /// Returns true if the sequence_type field is present (Some).
    fn has_sequence_type(&self) -> bool;
    /// Returns true if the number_of_subunits field is present (Some).
    fn has_number_of_subunits(&self) -> bool;
    /// Returns true if the area_of_hybridisation field is present (Some).
    fn has_area_of_hybridisation(&self) -> bool;
    /// Returns true if the oligo_nucleotide_type field is present (Some).
    fn has_oligo_nucleotide_type(&self) -> bool;
    /// Returns true if the subunit field is not empty.
    fn has_subunit(&self) -> bool;
}
