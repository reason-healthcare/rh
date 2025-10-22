use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use crate::resources::substance_source_material::SubstanceSourceMaterialFractiondescription;
use crate::resources::substance_source_material::SubstanceSourceMaterialOrganism;
use crate::resources::substance_source_material::SubstanceSourceMaterialPartdescription;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// SubstanceSourceMaterial Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSourceMaterial
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSourceMaterialAccessors: DomainResourceAccessors {
    /// Returns a reference to the sourceMaterialClass field.
    fn source_material_class(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the sourceMaterialType field.
    fn source_material_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the sourceMaterialState field.
    fn source_material_state(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the organismId field.
    fn organism_id(&self) -> Option<Identifier>;
    /// Returns a reference to the organismName field.
    fn organism_name(&self) -> Option<StringType>;
    /// Returns a reference to the parentSubstanceId field.
    fn parent_substance_id(&self) -> &[Identifier];
    /// Returns a reference to the parentSubstanceName field.
    fn parent_substance_name(&self) -> &[StringType];
    /// Returns a reference to the countryOfOrigin field.
    fn country_of_origin(&self) -> &[CodeableConcept];
    /// Returns a reference to the geographicalLocation field.
    fn geographical_location(&self) -> &[StringType];
    /// Returns a reference to the developmentStage field.
    fn development_stage(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the fractionDescription field.
    fn fraction_description(&self) -> &[SubstanceSourceMaterialFractiondescription];
    /// Returns a reference to the organism field.
    fn organism(&self) -> Option<SubstanceSourceMaterialOrganism>;
    /// Returns a reference to the partDescription field.
    fn part_description(&self) -> &[SubstanceSourceMaterialPartdescription];
}
/// SubstanceSourceMaterial Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSourceMaterial
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSourceMaterialMutators: DomainResourceMutators {
    /// Create a new SubstanceSourceMaterial with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance_source_material::SubstanceSourceMaterial;
    /// use hl7_fhir_r4_core::traits::substance_source_material::SubstanceSourceMaterialMutators;
    ///
    /// let resource = SubstanceSourceMaterial::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the sourceMaterialClass field and returns self for chaining.
    fn set_source_material_class(self, value: CodeableConcept) -> Self;
    /// Sets the sourceMaterialType field and returns self for chaining.
    fn set_source_material_type(self, value: CodeableConcept) -> Self;
    /// Sets the sourceMaterialState field and returns self for chaining.
    fn set_source_material_state(self, value: CodeableConcept) -> Self;
    /// Sets the organismId field and returns self for chaining.
    fn set_organism_id(self, value: Identifier) -> Self;
    /// Sets the organismName field and returns self for chaining.
    fn set_organism_name(self, value: String) -> Self;
    /// Sets the parentSubstanceId field and returns self for chaining.
    fn set_parent_substance_id(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the parentSubstanceId field and returns self for chaining.
    fn add_parent_substance_id(self, item: Identifier) -> Self;
    /// Sets the parentSubstanceName field and returns self for chaining.
    fn set_parent_substance_name(self, value: Vec<String>) -> Self;
    /// Adds an item to the parentSubstanceName field and returns self for chaining.
    fn add_parent_substance_name(self, item: String) -> Self;
    /// Sets the countryOfOrigin field and returns self for chaining.
    fn set_country_of_origin(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the countryOfOrigin field and returns self for chaining.
    fn add_country_of_origin(self, item: CodeableConcept) -> Self;
    /// Sets the geographicalLocation field and returns self for chaining.
    fn set_geographical_location(self, value: Vec<String>) -> Self;
    /// Adds an item to the geographicalLocation field and returns self for chaining.
    fn add_geographical_location(self, item: String) -> Self;
    /// Sets the developmentStage field and returns self for chaining.
    fn set_development_stage(self, value: CodeableConcept) -> Self;
    /// Sets the fractionDescription field and returns self for chaining.
    fn set_fraction_description(
        self,
        value: Vec<SubstanceSourceMaterialFractiondescription>,
    ) -> Self;
    /// Adds an item to the fractionDescription field and returns self for chaining.
    fn add_fraction_description(self, item: SubstanceSourceMaterialFractiondescription) -> Self;
    /// Sets the organism field and returns self for chaining.
    fn set_organism(self, value: SubstanceSourceMaterialOrganism) -> Self;
    /// Sets the partDescription field and returns self for chaining.
    fn set_part_description(self, value: Vec<SubstanceSourceMaterialPartdescription>) -> Self;
    /// Adds an item to the partDescription field and returns self for chaining.
    fn add_part_description(self, item: SubstanceSourceMaterialPartdescription) -> Self;
}
/// SubstanceSourceMaterial Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSourceMaterial
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceSourceMaterialExistence: DomainResourceExistence {
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
    /// Returns true if the source_material_class field is present (Some).
    fn has_source_material_class(&self) -> bool;
    /// Returns true if the source_material_type field is present (Some).
    fn has_source_material_type(&self) -> bool;
    /// Returns true if the source_material_state field is present (Some).
    fn has_source_material_state(&self) -> bool;
    /// Returns true if the organism_id field is present (Some).
    fn has_organism_id(&self) -> bool;
    /// Returns true if the organism_name field is present (Some).
    fn has_organism_name(&self) -> bool;
    /// Returns true if the parent_substance_id field is not empty.
    fn has_parent_substance_id(&self) -> bool;
    /// Returns true if the parent_substance_name field is not empty.
    fn has_parent_substance_name(&self) -> bool;
    /// Returns true if the country_of_origin field is not empty.
    fn has_country_of_origin(&self) -> bool;
    /// Returns true if the geographical_location field is not empty.
    fn has_geographical_location(&self) -> bool;
    /// Returns true if the development_stage field is present (Some).
    fn has_development_stage(&self) -> bool;
    /// Returns true if the fraction_description field is not empty.
    fn has_fraction_description(&self) -> bool;
    /// Returns true if the organism field is present (Some).
    fn has_organism(&self) -> bool;
    /// Returns true if the part_description field is not empty.
    fn has_part_description(&self) -> bool;
}
