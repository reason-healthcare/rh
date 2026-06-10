use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::substance_definition::SubstanceDefinitionCharacterization;
use crate::resources::substance_definition::SubstanceDefinitionCode;
use crate::resources::substance_definition::SubstanceDefinitionMoiety;
use crate::resources::substance_definition::SubstanceDefinitionMolecularweight;
use crate::resources::substance_definition::SubstanceDefinitionName;
use crate::resources::substance_definition::SubstanceDefinitionProperty;
use crate::resources::substance_definition::SubstanceDefinitionRelationship;
use crate::resources::substance_definition::SubstanceDefinitionSourcematerial;
use crate::resources::substance_definition::SubstanceDefinitionStructure;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstanceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the classification field.
    fn classification(&self) -> &[CodeableConcept];
    /// Returns a reference to the domain field.
    fn domain(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the grade field.
    fn grade(&self) -> &[CodeableConcept];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the informationSource field.
    fn information_source(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the supplier field.
    fn supplier(&self) -> &[Reference];
    /// Returns a reference to the moiety field.
    fn moiety(&self) -> &[SubstanceDefinitionMoiety];
    /// Returns a reference to the characterization field.
    fn characterization(&self) -> &[SubstanceDefinitionCharacterization];
    /// Returns a reference to the property field.
    fn property(&self) -> &[SubstanceDefinitionProperty];
    /// Returns a reference to the referenceInformation field.
    fn reference_information(&self) -> Option<Reference>;
    /// Returns a reference to the molecularWeight field.
    fn molecular_weight(&self) -> &[SubstanceDefinitionMolecularweight];
    /// Returns a reference to the structure field.
    fn structure(&self) -> Option<SubstanceDefinitionStructure>;
    /// Returns a reference to the code field.
    fn code(&self) -> &[SubstanceDefinitionCode];
    /// Returns a reference to the name field.
    fn name(&self) -> &[SubstanceDefinitionName];
    /// Returns a reference to the relationship field.
    fn relationship(&self) -> &[SubstanceDefinitionRelationship];
    /// Returns a reference to the nucleicAcid field.
    fn nucleic_acid(&self) -> Option<Reference>;
    /// Returns a reference to the polymer field.
    fn polymer(&self) -> Option<Reference>;
    /// Returns a reference to the protein field.
    fn protein(&self) -> Option<Reference>;
    /// Returns a reference to the sourceMaterial field.
    fn source_material(&self) -> Option<SubstanceDefinitionSourcematerial>;
}
/// SubstanceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstanceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceDefinitionMutators: DomainResourceMutators {
    /// Create a new SubstanceDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::substance_definition::SubstanceDefinition;
    /// use rh_hl7_fhir_r5_core::traits::substance_definition::SubstanceDefinitionMutators;
    ///
    /// let resource = SubstanceDefinition::new();
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
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: CodeableConcept) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the classification field and returns self for chaining.
    fn add_classification(self, item: CodeableConcept) -> Self;
    /// Sets the domain field and returns self for chaining.
    fn set_domain(self, value: CodeableConcept) -> Self;
    /// Sets the grade field and returns self for chaining.
    fn set_grade(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the grade field and returns self for chaining.
    fn add_grade(self, item: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the informationSource field and returns self for chaining.
    fn set_information_source(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the informationSource field and returns self for chaining.
    fn add_information_source(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the supplier field and returns self for chaining.
    fn set_supplier(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the supplier field and returns self for chaining.
    fn add_supplier(self, item: Reference) -> Self;
    /// Sets the moiety field and returns self for chaining.
    fn set_moiety(self, value: Vec<SubstanceDefinitionMoiety>) -> Self;
    /// Adds an item to the moiety field and returns self for chaining.
    fn add_moiety(self, item: SubstanceDefinitionMoiety) -> Self;
    /// Sets the characterization field and returns self for chaining.
    fn set_characterization(self, value: Vec<SubstanceDefinitionCharacterization>) -> Self;
    /// Adds an item to the characterization field and returns self for chaining.
    fn add_characterization(self, item: SubstanceDefinitionCharacterization) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<SubstanceDefinitionProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: SubstanceDefinitionProperty) -> Self;
    /// Sets the referenceInformation field and returns self for chaining.
    fn set_reference_information(self, value: Reference) -> Self;
    /// Sets the molecularWeight field and returns self for chaining.
    fn set_molecular_weight(self, value: Vec<SubstanceDefinitionMolecularweight>) -> Self;
    /// Adds an item to the molecularWeight field and returns self for chaining.
    fn add_molecular_weight(self, item: SubstanceDefinitionMolecularweight) -> Self;
    /// Sets the structure field and returns self for chaining.
    fn set_structure(self, value: SubstanceDefinitionStructure) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<SubstanceDefinitionCode>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: SubstanceDefinitionCode) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: Vec<SubstanceDefinitionName>) -> Self;
    /// Adds an item to the name field and returns self for chaining.
    fn add_name(self, item: SubstanceDefinitionName) -> Self;
    /// Sets the relationship field and returns self for chaining.
    fn set_relationship(self, value: Vec<SubstanceDefinitionRelationship>) -> Self;
    /// Adds an item to the relationship field and returns self for chaining.
    fn add_relationship(self, item: SubstanceDefinitionRelationship) -> Self;
    /// Sets the nucleicAcid field and returns self for chaining.
    fn set_nucleic_acid(self, value: Reference) -> Self;
    /// Sets the polymer field and returns self for chaining.
    fn set_polymer(self, value: Reference) -> Self;
    /// Sets the protein field and returns self for chaining.
    fn set_protein(self, value: Reference) -> Self;
    /// Sets the sourceMaterial field and returns self for chaining.
    fn set_source_material(self, value: SubstanceDefinitionSourcematerial) -> Self;
}
/// SubstanceDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstanceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceDefinitionExistence: DomainResourceExistence {
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the classification field is not empty.
    fn has_classification(&self) -> bool;
    /// Returns true if the domain field is present (Some).
    fn has_domain(&self) -> bool;
    /// Returns true if the grade field is not empty.
    fn has_grade(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the information_source field is not empty.
    fn has_information_source(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the supplier field is not empty.
    fn has_supplier(&self) -> bool;
    /// Returns true if the moiety field is not empty.
    fn has_moiety(&self) -> bool;
    /// Returns true if the characterization field is not empty.
    fn has_characterization(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the reference_information field is present (Some).
    fn has_reference_information(&self) -> bool;
    /// Returns true if the molecular_weight field is not empty.
    fn has_molecular_weight(&self) -> bool;
    /// Returns true if the structure field is present (Some).
    fn has_structure(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the name field is not empty.
    fn has_name(&self) -> bool;
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
