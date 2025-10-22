use crate::primitives::string::StringType;
use crate::resources::substance_reference_information::SubstanceReferenceInformationClassification;
use crate::resources::substance_reference_information::SubstanceReferenceInformationGene;
use crate::resources::substance_reference_information::SubstanceReferenceInformationGeneelement;
use crate::resources::substance_reference_information::SubstanceReferenceInformationTarget;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceReferenceInformation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceReferenceInformation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceReferenceInformationAccessors: DomainResourceAccessors {
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the gene field.
    fn gene(&self) -> &[SubstanceReferenceInformationGene];
    /// Returns a reference to the geneElement field.
    fn gene_element(&self) -> &[SubstanceReferenceInformationGeneelement];
    /// Returns a reference to the classification field.
    fn classification(&self) -> &[SubstanceReferenceInformationClassification];
    /// Returns a reference to the target field.
    fn target(&self) -> &[SubstanceReferenceInformationTarget];
}
/// SubstanceReferenceInformation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceReferenceInformation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceReferenceInformationMutators: DomainResourceMutators {
    /// Create a new SubstanceReferenceInformation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance_reference_information::SubstanceReferenceInformation;
    /// use hl7_fhir_r4_core::traits::substance_reference_information::SubstanceReferenceInformationMutators;
    ///
    /// let resource = SubstanceReferenceInformation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the gene field and returns self for chaining.
    fn set_gene(self, value: Vec<SubstanceReferenceInformationGene>) -> Self;
    /// Adds an item to the gene field and returns self for chaining.
    fn add_gene(self, item: SubstanceReferenceInformationGene) -> Self;
    /// Sets the geneElement field and returns self for chaining.
    fn set_gene_element(self, value: Vec<SubstanceReferenceInformationGeneelement>) -> Self;
    /// Adds an item to the geneElement field and returns self for chaining.
    fn add_gene_element(self, item: SubstanceReferenceInformationGeneelement) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: Vec<SubstanceReferenceInformationClassification>) -> Self;
    /// Adds an item to the classification field and returns self for chaining.
    fn add_classification(self, item: SubstanceReferenceInformationClassification) -> Self;
    /// Sets the target field and returns self for chaining.
    fn set_target(self, value: Vec<SubstanceReferenceInformationTarget>) -> Self;
    /// Adds an item to the target field and returns self for chaining.
    fn add_target(self, item: SubstanceReferenceInformationTarget) -> Self;
}
/// SubstanceReferenceInformation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Todo.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceReferenceInformation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceReferenceInformationExistence: DomainResourceExistence {
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
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the gene field is not empty.
    fn has_gene(&self) -> bool;
    /// Returns true if the gene_element field is not empty.
    fn has_gene_element(&self) -> bool;
    /// Returns true if the classification field is not empty.
    fn has_classification(&self) -> bool;
    /// Returns true if the target field is not empty.
    fn has_target(&self) -> bool;
}
