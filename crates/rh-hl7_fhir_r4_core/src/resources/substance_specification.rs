use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceSpecification
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceSpecification
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: SubstanceSpecification
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecification {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier by which this substance is known
    pub identifier: Option<Identifier>,
    /// High level categorization, e.g. polymer or nucleic acid
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Status of substance within the catalogue e.g. approved
    pub status: Option<CodeableConcept>,
    /// If the substance applies to only human or veterinary use
    pub domain: Option<CodeableConcept>,
    /// Textual description of the substance
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
    /// Textual comment about this record of a substance
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Moiety, for structural modifications
    pub moiety: Option<Vec<SubstanceSpecificationMoiety>>,
    /// General specifications for this substance, including how it is related to other substances
    pub property: Option<Vec<SubstanceSpecificationProperty>>,
    /// General information detailing this substance
    #[serde(rename = "referenceInformation")]
    pub reference_information: Option<Reference>,
    /// Structural information
    pub structure: Option<SubstanceSpecificationStructure>,
    /// Codes associated with the substance
    pub code: Option<Vec<SubstanceSpecificationCode>>,
    /// Names applicable to this substance
    pub name: Option<Vec<SubstanceSpecificationName>>,
    /// The molecular weight or weight range (for proteins, polymers or nucleic acids)
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<Vec<StringType>>,
    /// A link between this substance and another, with details of the relationship
    pub relationship: Option<Vec<SubstanceSpecificationRelationship>>,
    /// Data items specific to nucleic acids
    #[serde(rename = "nucleicAcid")]
    pub nucleic_acid: Option<Reference>,
    /// Data items specific to polymers
    pub polymer: Option<Reference>,
    /// Data items specific to proteins
    pub protein: Option<Reference>,
    /// Material or taxonomic/anatomical source for the substance
    #[serde(rename = "sourceMaterial")]
    pub source_material: Option<Reference>,
}
/// SubstanceSpecification nested structure for the 'relationship' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationRelationship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A pointer to another substance, as a resource or just a representational code (Reference)
    #[serde(rename = "substanceReference")]
    pub substance_reference: Option<Reference>,
    /// A pointer to another substance, as a resource or just a representational code (CodeableConcept)
    #[serde(rename = "substanceCodeableConcept")]
    pub substance_codeable_concept: Option<CodeableConcept>,
    /// For example "salt to parent", "active moiety", "starting material"
    pub relationship: Option<CodeableConcept>,
    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships
    #[serde(rename = "isDefining")]
    pub is_defining: Option<BooleanType>,
    /// Extension element for the 'isDefining' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDefining")]
    pub _is_defining: Option<Element>,
    /// A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other (Range)
    #[serde(rename = "amountRange")]
    pub amount_range: Option<Range>,
    /// A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other (Ratio)
    #[serde(rename = "amountRatio")]
    pub amount_ratio: Option<Ratio>,
    /// A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
    /// For use when the numeric
    #[serde(rename = "amountRatioLowLimit")]
    pub amount_ratio_low_limit: Option<Ratio>,
    /// An operator for the amount, for example "average", "approximately", "less than"
    #[serde(rename = "amountType")]
    pub amount_type: Option<CodeableConcept>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceSpecification nested structure for the 'code' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specific code
    pub code: Option<CodeableConcept>,
    /// Status of the code assignment
    pub status: Option<CodeableConcept>,
    /// The date at which the code status is changed as part of the terminology maintenance
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// Any comment can be provided in this field, if necessary
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceSpecificationName nested structure for the 'official' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationNameOfficial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Which authority uses this official name
    pub authority: Option<CodeableConcept>,
    /// The status of the official name
    pub status: Option<CodeableConcept>,
    /// Date of official name change
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
}
/// SubstanceSpecification nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A category for this property, e.g. Physical, Chemical, Enzymatic
    pub category: Option<CodeableConcept>,
    /// Property type e.g. viscosity, pH, isoelectric point
    pub code: Option<CodeableConcept>,
    /// Parameters that were used in the measurement of a property (e.g. for viscosity: measured at 20C with a pH of 7.1)
    pub parameters: Option<StringType>,
    /// Extension element for the 'parameters' primitive field. Contains metadata and extensions.
    pub _parameters: Option<Element>,
    /// A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol) (Reference)
    #[serde(rename = "definingSubstanceReference")]
    pub defining_substance_reference: Option<Reference>,
    /// A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol) (CodeableConcept)
    #[serde(rename = "definingSubstanceCodeableConcept")]
    pub defining_substance_codeable_concept: Option<CodeableConcept>,
    /// Quantitative value for this property (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// Quantitative value for this property (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
}
/// SubstanceSpecification nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Details of the official nature of this name
    pub official: Option<Vec<SubstanceSpecificationNameOfficial>>,
    /// The actual name
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The status of the name
    pub status: Option<CodeableConcept>,
    /// If this is the preferred name for this substance
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
    /// Language of the name
    pub language: Option<Vec<CodeableConcept>>,
    /// The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive
    pub domain: Option<Vec<CodeableConcept>>,
    /// The jurisdiction where this name applies
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// A synonym of this name
    pub synonym: Option<Vec<StringType>>,
    /// A translation for this name
    pub translation: Option<Vec<StringType>>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceSpecificationStructure nested structure for the 'isotope' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationStructureIsotope {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Substance identifier for each non-natural or radioisotope
    pub identifier: Option<Identifier>,
    /// Substance name for each non-natural or radioisotope
    pub name: Option<CodeableConcept>,
    /// The type of isotopic substitution present in a single substance
    pub substitution: Option<CodeableConcept>,
    /// Half life - for a non-natural nuclide
    #[serde(rename = "halfLife")]
    pub half_life: Option<Quantity>,
}
/// SubstanceSpecification nested structure for the 'moiety' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationMoiety {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Role that the moiety is playing
    pub role: Option<CodeableConcept>,
    /// Identifier by which this moiety substance is known
    pub identifier: Option<Identifier>,
    /// Textual name for this moiety substance
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Stereochemistry type
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,
    /// Molecular formula
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<StringType>,
    /// Extension element for the 'molecularFormula' primitive field. Contains metadata and extensions.
    #[serde(rename = "_molecularFormula")]
    pub _molecular_formula: Option<Element>,
    /// Quantitative value for this moiety (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// Quantitative value for this moiety (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
}
/// SubstanceSpecificationStructure nested structure for the 'representation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationStructureRepresentation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of structure (e.g. Full, Partial, Representative)
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The structural representation as text string in a format e.g. InChI, SMILES, MOLFILE, CDX
    pub representation: Option<StringType>,
    /// Extension element for the 'representation' primitive field. Contains metadata and extensions.
    pub _representation: Option<Element>,
    /// An attached file with the structural representation
    pub attachment: Option<Attachment>,
}
/// SubstanceSpecificationStructureIsotope nested structure for the 'molecularWeight' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationStructureIsotopeMolecularweight {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The method by which the molecular weight was determined
    pub method: Option<CodeableConcept>,
    /// Type of molecular weight such as exact, average (also known as. number average), weight average
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field
    pub amount: Option<Quantity>,
}
/// SubstanceSpecification nested structure for the 'structure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSpecificationStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Molecular structural representation
    pub representation: Option<Vec<SubstanceSpecificationStructureRepresentation>>,
    /// Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio
    pub isotope: Option<Vec<SubstanceSpecificationStructureIsotope>>,
    /// Stereochemistry type
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,
    /// Molecular formula
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<StringType>,
    /// Extension element for the 'molecularFormula' primitive field. Contains metadata and extensions.
    #[serde(rename = "_molecularFormula")]
    pub _molecular_formula: Option<Element>,
    /// Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot
    #[serde(rename = "molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety: Option<StringType>,
    /// Extension element for the 'molecularFormulaByMoiety' primitive field. Contains metadata and extensions.
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub _molecular_formula_by_moiety: Option<Element>,
    /// The molecular weight or weight range (for proteins, polymers or nucleic acids)
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<StringType>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}

impl Default for SubstanceSpecification {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            status: Default::default(),
            domain: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            source: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            moiety: Default::default(),
            property: Default::default(),
            reference_information: Default::default(),
            structure: Default::default(),
            code: Default::default(),
            name: Default::default(),
            molecular_weight: Default::default(),
            relationship: Default::default(),
            nucleic_acid: Default::default(),
            polymer: Default::default(),
            protein: Default::default(),
            source_material: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationRelationship {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance_reference: Default::default(),
            substance_codeable_concept: Default::default(),
            relationship: Default::default(),
            is_defining: Default::default(),
            _is_defining: Default::default(),
            amount_quantity: Default::default(),
            amount_range: Default::default(),
            amount_ratio: Default::default(),
            amount_string: Default::default(),
            amount_ratio_low_limit: Default::default(),
            amount_type: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationCode {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationNameOfficial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            authority: Default::default(),
            status: Default::default(),
            date: Default::default(),
            _date: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            code: Default::default(),
            parameters: Default::default(),
            _parameters: Default::default(),
            defining_substance_reference: Default::default(),
            defining_substance_codeable_concept: Default::default(),
            amount_quantity: Default::default(),
            amount_string: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationName {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            official: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            type_: Default::default(),
            status: Default::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
            language: Default::default(),
            domain: Default::default(),
            jurisdiction: Default::default(),
            synonym: Default::default(),
            translation: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationStructureIsotope {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            name: Default::default(),
            substitution: Default::default(),
            half_life: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationMoiety {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            stereochemistry: Default::default(),
            optical_activity: Default::default(),
            molecular_formula: Default::default(),
            _molecular_formula: Default::default(),
            amount_quantity: Default::default(),
            amount_string: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationStructureRepresentation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            representation: Default::default(),
            _representation: Default::default(),
            attachment: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationStructureIsotopeMolecularweight {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            method: Default::default(),
            type_: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstanceSpecificationStructure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            representation: Default::default(),
            isotope: Default::default(),
            stereochemistry: Default::default(),
            optical_activity: Default::default(),
            molecular_formula: Default::default(),
            _molecular_formula: Default::default(),
            molecular_formula_by_moiety: Default::default(),
            _molecular_formula_by_moiety: Default::default(),
            molecular_weight: Default::default(),
            source: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstanceSpecification {
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

impl crate::traits::resource::ResourceMutators for SubstanceSpecification {
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

impl crate::traits::resource::ResourceExistence for SubstanceSpecification {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceSpecification {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceSpecification {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceSpecification {
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

impl crate::traits::substance_specification::SubstanceSpecificationAccessors
    for SubstanceSpecification
{
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn domain(&self) -> Option<CodeableConcept> {
        self.domain.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn source(&self) -> &[Reference] {
        self.source.as_deref().unwrap_or(&[])
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn moiety(&self) -> &[SubstanceSpecificationMoiety] {
        self.moiety.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[SubstanceSpecificationProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn reference_information(&self) -> Option<Reference> {
        self.reference_information.clone()
    }
    fn structure(&self) -> Option<SubstanceSpecificationStructure> {
        self.structure.clone()
    }
    fn code(&self) -> &[SubstanceSpecificationCode] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[SubstanceSpecificationName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn relationship(&self) -> &[SubstanceSpecificationRelationship] {
        self.relationship.as_deref().unwrap_or(&[])
    }
    fn nucleic_acid(&self) -> Option<Reference> {
        self.nucleic_acid.clone()
    }
    fn polymer(&self) -> Option<Reference> {
        self.polymer.clone()
    }
    fn protein(&self) -> Option<Reference> {
        self.protein.clone()
    }
    fn source_material(&self) -> Option<Reference> {
        self.source_material.clone()
    }
}

impl crate::traits::substance_specification::SubstanceSpecificationMutators
    for SubstanceSpecification
{
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_domain(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.domain = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_source(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn add_source(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.source.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_moiety(self, value: Vec<SubstanceSpecificationMoiety>) -> Self {
        let mut resource = self.clone();
        resource.moiety = Some(value);
        resource
    }
    fn add_moiety(self, item: SubstanceSpecificationMoiety) -> Self {
        let mut resource = self.clone();
        resource.moiety.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<SubstanceSpecificationProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: SubstanceSpecificationProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reference_information(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.reference_information = Some(value);
        resource
    }
    fn set_structure(self, value: SubstanceSpecificationStructure) -> Self {
        let mut resource = self.clone();
        resource.structure = Some(value);
        resource
    }
    fn set_code(self, value: Vec<SubstanceSpecificationCode>) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: SubstanceSpecificationCode) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: Vec<SubstanceSpecificationName>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: SubstanceSpecificationName) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_molecular_weight(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.molecular_weight = Some(value);
        resource
    }
    fn add_molecular_weight(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .molecular_weight
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_relationship(self, value: Vec<SubstanceSpecificationRelationship>) -> Self {
        let mut resource = self.clone();
        resource.relationship = Some(value);
        resource
    }
    fn add_relationship(self, item: SubstanceSpecificationRelationship) -> Self {
        let mut resource = self.clone();
        resource
            .relationship
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_nucleic_acid(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.nucleic_acid = Some(value);
        resource
    }
    fn set_polymer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.polymer = Some(value);
        resource
    }
    fn set_protein(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.protein = Some(value);
        resource
    }
    fn set_source_material(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.source_material = Some(value);
        resource
    }
}

impl crate::traits::substance_specification::SubstanceSpecificationExistence
    for SubstanceSpecification
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
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_domain(&self) -> bool {
        self.domain.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_source(&self) -> bool {
        self.source.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_moiety(&self) -> bool {
        self.moiety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reference_information(&self) -> bool {
        self.reference_information.is_some()
    }
    fn has_structure(&self) -> bool {
        self.structure.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_molecular_weight(&self) -> bool {
        self.molecular_weight
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_relationship(&self) -> bool {
        self.relationship.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_nucleic_acid(&self) -> bool {
        self.nucleic_acid.is_some()
    }
    fn has_polymer(&self) -> bool {
        self.polymer.is_some()
    }
    fn has_protein(&self) -> bool {
        self.protein.is_some()
    }
    fn has_source_material(&self) -> bool {
        self.source_material.is_some()
    }
}
