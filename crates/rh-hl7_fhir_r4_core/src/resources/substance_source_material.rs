use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceSourceMaterial
///
/// Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSourceMaterial
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSourceMaterial
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// General high level classification of the source material specific to the origin of the material
    #[serde(rename = "sourceMaterialClass")]
    pub source_material_class: Option<CodeableConcept>,
    /// The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent
    #[serde(rename = "sourceMaterialType")]
    pub source_material_type: Option<CodeableConcept>,
    /// The state of the source material when extracted
    #[serde(rename = "sourceMaterialState")]
    pub source_material_state: Option<CodeableConcept>,
    /// The unique identifier associated with the source material parent organism shall be specified
    #[serde(rename = "organismId")]
    pub organism_id: Option<Identifier>,
    /// The organism accepted Scientific name shall be provided based on the organism taxonomy
    #[serde(rename = "organismName")]
    pub organism_name: Option<StringType>,
    /// Extension element for the 'organismName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_organismName")]
    pub _organism_name: Option<Element>,
    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)
    #[serde(rename = "parentSubstanceId")]
    pub parent_substance_id: Option<Vec<Identifier>>,
    /// The parent substance of the Herbal Drug, or Herbal preparation
    #[serde(rename = "parentSubstanceName")]
    pub parent_substance_name: Option<Vec<StringType>>,
    /// Extension element for the 'parentSubstanceName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_parentSubstanceName")]
    pub _parent_substance_name: Option<Element>,
    /// The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File. For “Plasma-derived substances” the attribute country of origin provides information about the countries used for the manufacturing of the Cryopoor plama or Crioprecipitate
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<Vec<CodeableConcept>>,
    /// The place/region where the plant is harvested or the places/regions where the animal source material has its habitat
    #[serde(rename = "geographicalLocation")]
    pub geographical_location: Option<Vec<StringType>>,
    /// Extension element for the 'geographicalLocation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_geographicalLocation")]
    pub _geographical_location: Option<Element>,
    /// Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)
    #[serde(rename = "developmentStage")]
    pub development_stage: Option<CodeableConcept>,
    /// Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels
    #[serde(rename = "fractionDescription")]
    pub fraction_description: Option<Vec<SubstanceSourceMaterialFractiondescription>>,
    /// This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf
    pub organism: Option<SubstanceSourceMaterialOrganism>,
    /// To do
    #[serde(rename = "partDescription")]
    pub part_description: Option<Vec<SubstanceSourceMaterialPartdescription>>,
}
/// SubstanceSourceMaterialOrganism nested structure for the 'hybrid' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal
    #[serde(rename = "maternalOrganismId")]
    pub maternal_organism_id: Option<StringType>,
    /// Extension element for the 'maternalOrganismId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maternalOrganismId")]
    pub _maternal_organism_id: Option<Element>,
    /// The name of the maternal species constituting the hybrid organism shall be specified. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal
    #[serde(rename = "maternalOrganismName")]
    pub maternal_organism_name: Option<StringType>,
    /// Extension element for the 'maternalOrganismName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maternalOrganismName")]
    pub _maternal_organism_name: Option<Element>,
    /// The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary
    #[serde(rename = "paternalOrganismId")]
    pub paternal_organism_id: Option<StringType>,
    /// Extension element for the 'paternalOrganismId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paternalOrganismId")]
    pub _paternal_organism_id: Option<Element>,
    /// The name of the paternal species constituting the hybrid organism shall be specified
    #[serde(rename = "paternalOrganismName")]
    pub paternal_organism_name: Option<StringType>,
    /// Extension element for the 'paternalOrganismName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_paternalOrganismName")]
    pub _paternal_organism_name: Option<Element>,
    /// The hybrid type of an organism shall be specified
    #[serde(rename = "hybridType")]
    pub hybrid_type: Option<CodeableConcept>,
}
/// SubstanceSourceMaterialOrganism nested structure for the 'author' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of author of an organism species shall be specified. The parenthetical author of an organism species refers to the first author who published the plant/animal name (of any rank). The primary author of an organism species refers to the first author(s), who validly published the plant/animal name
    #[serde(rename = "authorType")]
    pub author_type: Option<CodeableConcept>,
    /// The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable; refers to the year in which the first author(s) published the infraspecific plant/animal name (of any rank)
    #[serde(rename = "authorDescription")]
    pub author_description: Option<StringType>,
    /// Extension element for the 'authorDescription' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authorDescription")]
    pub _author_description: Option<Element>,
}
/// SubstanceSourceMaterial nested structure for the 'fractionDescription' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialFractiondescription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// This element is capturing information about the fraction of a plant part, or human plasma for fractionation
    pub fraction: Option<StringType>,
    /// Extension element for the 'fraction' primitive field. Contains metadata and extensions.
    pub _fraction: Option<Element>,
    /// The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1
    #[serde(rename = "materialType")]
    pub material_type: Option<CodeableConcept>,
}
/// SubstanceSourceMaterial nested structure for the 'organism' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialOrganism {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// 4.9.13.6.1 Author type (Conditional)
    pub author: Option<Vec<SubstanceSourceMaterialOrganismAuthor>>,
    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
    pub hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    /// 4.9.13.7.1 Kingdom (Conditional)
    #[serde(rename = "organismGeneral")]
    pub organism_general: Option<SubstanceSourceMaterialOrganismOrganismgeneral>,
    /// The family of an organism shall be specified
    pub family: Option<CodeableConcept>,
    /// The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name; it is present in names for genera, species and infraspecies
    pub genus: Option<CodeableConcept>,
    /// The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal; it is present in names for species and infraspecies
    pub species: Option<CodeableConcept>,
    /// The Intraspecific type of an organism shall be specified
    #[serde(rename = "intraspecificType")]
    pub intraspecific_type: Option<CodeableConcept>,
    /// The intraspecific description of an organism shall be specified based on a controlled vocabulary. For Influenza Vaccine, the intraspecific description shall contain the syntax of the antigen in line with the WHO convention
    #[serde(rename = "intraspecificDescription")]
    pub intraspecific_description: Option<StringType>,
    /// Extension element for the 'intraspecificDescription' primitive field. Contains metadata and extensions.
    #[serde(rename = "_intraspecificDescription")]
    pub _intraspecific_description: Option<Element>,
}
/// SubstanceSourceMaterialOrganism nested structure for the 'organismGeneral' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialOrganismOrganismgeneral {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kingdom of an organism shall be specified
    pub kingdom: Option<CodeableConcept>,
    /// The phylum of an organism shall be specified
    pub phylum: Option<CodeableConcept>,
    /// The class of an organism shall be specified
    pub class: Option<CodeableConcept>,
    /// The order of an organism shall be specified,
    pub order: Option<CodeableConcept>,
}
/// SubstanceSourceMaterial nested structure for the 'partDescription' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSourceMaterialPartdescription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Entity of anatomical origin of source material within an organism
    pub part: Option<CodeableConcept>,
    /// The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply
    #[serde(rename = "partLocation")]
    pub part_location: Option<CodeableConcept>,
}

impl Default for SubstanceSourceMaterial {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            source_material_class: Default::default(),
            source_material_type: Default::default(),
            source_material_state: Default::default(),
            organism_id: Default::default(),
            organism_name: Default::default(),
            _organism_name: Default::default(),
            parent_substance_id: Default::default(),
            parent_substance_name: Default::default(),
            _parent_substance_name: Default::default(),
            country_of_origin: Default::default(),
            geographical_location: Default::default(),
            _geographical_location: Default::default(),
            development_stage: Default::default(),
            fraction_description: Default::default(),
            organism: Default::default(),
            part_description: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialOrganismHybrid {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            maternal_organism_id: Default::default(),
            _maternal_organism_id: Default::default(),
            maternal_organism_name: Default::default(),
            _maternal_organism_name: Default::default(),
            paternal_organism_id: Default::default(),
            _paternal_organism_id: Default::default(),
            paternal_organism_name: Default::default(),
            _paternal_organism_name: Default::default(),
            hybrid_type: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialOrganismAuthor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            author_type: Default::default(),
            author_description: Default::default(),
            _author_description: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialFractiondescription {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            fraction: Default::default(),
            _fraction: Default::default(),
            material_type: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialOrganism {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            author: Default::default(),
            hybrid: Default::default(),
            organism_general: Default::default(),
            family: Default::default(),
            genus: Default::default(),
            species: Default::default(),
            intraspecific_type: Default::default(),
            intraspecific_description: Default::default(),
            _intraspecific_description: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialOrganismOrganismgeneral {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            kingdom: Default::default(),
            phylum: Default::default(),
            class: Default::default(),
            order: Default::default(),
        }
    }
}

impl Default for SubstanceSourceMaterialPartdescription {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            part: Default::default(),
            part_location: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstanceSourceMaterial {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for SubstanceSourceMaterial {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for SubstanceSourceMaterial {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceSourceMaterial {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceSourceMaterial {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceSourceMaterial {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::substance_source_material::SubstanceSourceMaterialAccessors
    for SubstanceSourceMaterial
{
    fn source_material_class(&self) -> Option<CodeableConcept> {
        self.source_material_class.clone()
    }
    fn source_material_type(&self) -> Option<CodeableConcept> {
        self.source_material_type.clone()
    }
    fn source_material_state(&self) -> Option<CodeableConcept> {
        self.source_material_state.clone()
    }
    fn organism_id(&self) -> Option<Identifier> {
        self.organism_id.clone()
    }
    fn organism_name(&self) -> Option<StringType> {
        self.organism_name.clone()
    }
    fn parent_substance_id(&self) -> &[Identifier] {
        self.parent_substance_id.as_deref().unwrap_or(&[])
    }
    fn parent_substance_name(&self) -> &[StringType] {
        self.parent_substance_name.as_deref().unwrap_or(&[])
    }
    fn country_of_origin(&self) -> &[CodeableConcept] {
        self.country_of_origin.as_deref().unwrap_or(&[])
    }
    fn geographical_location(&self) -> &[StringType] {
        self.geographical_location.as_deref().unwrap_or(&[])
    }
    fn development_stage(&self) -> Option<CodeableConcept> {
        self.development_stage.clone()
    }
    fn fraction_description(&self) -> &[SubstanceSourceMaterialFractiondescription] {
        self.fraction_description.as_deref().unwrap_or(&[])
    }
    fn organism(&self) -> Option<SubstanceSourceMaterialOrganism> {
        self.organism.clone()
    }
    fn part_description(&self) -> &[SubstanceSourceMaterialPartdescription] {
        self.part_description.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::substance_source_material::SubstanceSourceMaterialMutators
    for SubstanceSourceMaterial
{
    fn new() -> Self {
        Self::default()
    }
    fn set_source_material_class(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.source_material_class = Some(value);
        resource
    }
    fn set_source_material_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.source_material_type = Some(value);
        resource
    }
    fn set_source_material_state(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.source_material_state = Some(value);
        resource
    }
    fn set_organism_id(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.organism_id = Some(value);
        resource
    }
    fn set_organism_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.organism_name = Some(value);
        resource
    }
    fn set_parent_substance_id(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.parent_substance_id = Some(value);
        resource
    }
    fn add_parent_substance_id(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource
            .parent_substance_id
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_parent_substance_name(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.parent_substance_name = Some(value);
        resource
    }
    fn add_parent_substance_name(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .parent_substance_name
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_country_of_origin(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.country_of_origin = Some(value);
        resource
    }
    fn add_country_of_origin(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .country_of_origin
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_geographical_location(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.geographical_location = Some(value);
        resource
    }
    fn add_geographical_location(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .geographical_location
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_development_stage(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.development_stage = Some(value);
        resource
    }
    fn set_fraction_description(
        self,
        value: Vec<SubstanceSourceMaterialFractiondescription>,
    ) -> Self {
        let mut resource = self.clone();
        resource.fraction_description = Some(value);
        resource
    }
    fn add_fraction_description(self, item: SubstanceSourceMaterialFractiondescription) -> Self {
        let mut resource = self.clone();
        resource
            .fraction_description
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_organism(self, value: SubstanceSourceMaterialOrganism) -> Self {
        let mut resource = self.clone();
        resource.organism = Some(value);
        resource
    }
    fn set_part_description(self, value: Vec<SubstanceSourceMaterialPartdescription>) -> Self {
        let mut resource = self.clone();
        resource.part_description = Some(value);
        resource
    }
    fn add_part_description(self, item: SubstanceSourceMaterialPartdescription) -> Self {
        let mut resource = self.clone();
        resource
            .part_description
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::substance_source_material::SubstanceSourceMaterialExistence
    for SubstanceSourceMaterial
{
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_source_material_class(&self) -> bool {
        self.source_material_class.is_some()
    }
    fn has_source_material_type(&self) -> bool {
        self.source_material_type.is_some()
    }
    fn has_source_material_state(&self) -> bool {
        self.source_material_state.is_some()
    }
    fn has_organism_id(&self) -> bool {
        self.organism_id.is_some()
    }
    fn has_organism_name(&self) -> bool {
        self.organism_name.is_some()
    }
    fn has_parent_substance_id(&self) -> bool {
        self.parent_substance_id
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_parent_substance_name(&self) -> bool {
        self.parent_substance_name
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_country_of_origin(&self) -> bool {
        self.country_of_origin
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_geographical_location(&self) -> bool {
        self.geographical_location
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_development_stage(&self) -> bool {
        self.development_stage.is_some()
    }
    fn has_fraction_description(&self) -> bool {
        self.fraction_description
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_organism(&self) -> bool {
        self.organism.is_some()
    }
    fn has_part_description(&self) -> bool {
        self.part_description
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
