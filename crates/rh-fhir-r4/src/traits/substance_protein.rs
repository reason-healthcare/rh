use crate::datatypes::codeable_concept::CodeableConcept;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::substance_protein::SubstanceProteinSubunit;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceProtein Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceProtein
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceProteinAccessors: DomainResourceAccessors {
    /// Returns a reference to the sequenceType field.
    fn sequence_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the numberOfSubunits field.
    fn number_of_subunits(&self) -> Option<IntegerType>;
    /// Returns a reference to the disulfideLinkage field.
    fn disulfide_linkage(&self) -> &[StringType];
    /// Returns a reference to the subunit field.
    fn subunit(&self) -> &[SubstanceProteinSubunit];
}
/// SubstanceProtein Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceProtein
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceProteinMutators: DomainResourceMutators {
    /// Create a new SubstanceProtein with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance_protein::SubstanceProtein;
    /// use hl7_fhir_r4_core::traits::substance_protein::SubstanceProteinMutators;
    ///
    /// let resource = SubstanceProtein::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the sequenceType field and returns self for chaining.
    fn set_sequence_type(self, value: CodeableConcept) -> Self;
    /// Sets the numberOfSubunits field and returns self for chaining.
    fn set_number_of_subunits(self, value: i32) -> Self;
    /// Sets the disulfideLinkage field and returns self for chaining.
    fn set_disulfide_linkage(self, value: Vec<String>) -> Self;
    /// Adds an item to the disulfideLinkage field and returns self for chaining.
    fn add_disulfide_linkage(self, item: String) -> Self;
    /// Sets the subunit field and returns self for chaining.
    fn set_subunit(self, value: Vec<SubstanceProteinSubunit>) -> Self;
    /// Adds an item to the subunit field and returns self for chaining.
    fn add_subunit(self, item: SubstanceProteinSubunit) -> Self;
}
/// SubstanceProtein Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceProtein
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceProteinExistence: DomainResourceExistence {
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
    /// Returns true if the disulfide_linkage field is not empty.
    fn has_disulfide_linkage(&self) -> bool;
    /// Returns true if the subunit field is not empty.
    fn has_subunit(&self) -> bool;
}
