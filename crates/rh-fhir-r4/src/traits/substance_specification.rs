use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::substance_specification::SubstanceSpecificationCode;
use crate::resources::substance_specification::SubstanceSpecificationMoiety;
use crate::resources::substance_specification::SubstanceSpecificationName;
use crate::resources::substance_specification::SubstanceSpecificationProperty;
use crate::resources::substance_specification::SubstanceSpecificationRelationship;
use crate::resources::substance_specification::SubstanceSpecificationStructure;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceSpecification Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSpecification
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSpecification
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSpecificationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the domain field.
    fn domain(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the source field.
    fn source(&self) -> &[Reference];
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the moiety field.
    fn moiety(&self) -> &[SubstanceSpecificationMoiety];
    /// Returns a reference to the property field.
    fn property(&self) -> &[SubstanceSpecificationProperty];
    /// Returns a reference to the referenceInformation field.
    fn reference_information(&self) -> Option<Reference>;
    /// Returns a reference to the structure field.
    fn structure(&self) -> Option<SubstanceSpecificationStructure>;
    /// Returns a reference to the code field.
    fn code(&self) -> &[SubstanceSpecificationCode];
    /// Returns a reference to the name field.
    fn name(&self) -> &[SubstanceSpecificationName];
    /// Returns a reference to the relationship field.
    fn relationship(&self) -> &[SubstanceSpecificationRelationship];
    /// Returns a reference to the nucleicAcid field.
    fn nucleic_acid(&self) -> Option<Reference>;
    /// Returns a reference to the polymer field.
    fn polymer(&self) -> Option<Reference>;
    /// Returns a reference to the protein field.
    fn protein(&self) -> Option<Reference>;
    /// Returns a reference to the sourceMaterial field.
    fn source_material(&self) -> Option<Reference>;
}
/// SubstanceSpecification Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSpecification
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSpecification
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSpecificationMutators: DomainResourceMutators {
    /// Create a new SubstanceSpecification with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance_specification::SubstanceSpecification;
    /// use hl7_fhir_r4_core::traits::substance_specification::SubstanceSpecificationMutators;
    ///
    /// let resource = SubstanceSpecification::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the domain field and returns self for chaining.
    fn set_domain(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the source field and returns self for chaining.
    fn add_source(self, item: Reference) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the moiety field and returns self for chaining.
    fn set_moiety(self, value: Vec<SubstanceSpecificationMoiety>) -> Self;
    /// Adds an item to the moiety field and returns self for chaining.
    fn add_moiety(self, item: SubstanceSpecificationMoiety) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<SubstanceSpecificationProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: SubstanceSpecificationProperty) -> Self;
    /// Sets the referenceInformation field and returns self for chaining.
    fn set_reference_information(self, value: Reference) -> Self;
    /// Sets the structure field and returns self for chaining.
    fn set_structure(self, value: SubstanceSpecificationStructure) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<SubstanceSpecificationCode>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: SubstanceSpecificationCode) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<SubstanceSpecificationName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: SubstanceSpecificationName) -> Self;
    /// Sets the molecularWeight field and returns self for chaining.
    fn set_molecular_weight(self, value: Vec<String>) -> Self;
    /// Adds an item to the molecularWeight field and returns self for chaining.
    fn add_molecular_weight(self, item: String) -> Self;
    /// Sets the relationship field and returns self for chaining.
    fn set_relationship(self, value: Vec<SubstanceSpecificationRelationship>) -> Self;
    /// Adds an item to the relationship field and returns self for chaining.
    fn add_relationship(self, item: SubstanceSpecificationRelationship) -> Self;
    /// Sets the nucleicAcid field and returns self for chaining.
    fn set_nucleic_acid(self, value: Reference) -> Self;
    /// Sets the polymer field and returns self for chaining.
    fn set_polymer(self, value: Reference) -> Self;
    /// Sets the protein field and returns self for chaining.
    fn set_protein(self, value: Reference) -> Self;
    /// Sets the sourceMaterial field and returns self for chaining.
    fn set_source_material(self, value: Reference) -> Self;
}
/// SubstanceSpecification Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSpecification
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSpecification
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSpecificationExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the domain field is present (Some).
    fn has_domain(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the source field is not empty.
    fn has_source(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the moiety field is not empty.
    fn has_moiety(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the reference_information field is present (Some).
    fn has_reference_information(&self) -> bool;
    /// Returns true if the structure field is present (Some).
    fn has_structure(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
    /// Returns true if the molecular_weight field is not empty.
    fn has_molecular_weight(&self) -> bool;
    /// Returns true if the relationship field is not empty.
    fn has_relationship(&self) -> bool;
    /// Returns true if the nucleic_acid field is present (Some).
    fn has_nucleic_acid(&self) -> bool;
    /// Returns true if the polymer field is present (Some).
    fn has_polymer(&self) -> bool;
    /// Returns true if the protein field is present (Some).
    fn has_protein(&self) -> bool;
    /// Returns true if the source_material field is present (Some).
    fn has_source_material(&self) -> bool;
}
