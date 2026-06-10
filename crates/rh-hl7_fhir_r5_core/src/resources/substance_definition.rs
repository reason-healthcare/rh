use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubstanceDefinition
///
/// The detailed description of a substance, typically at a level beyond what is used for prescribing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubstanceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier by which this substance is known
    pub identifier: Option<Vec<Identifier>>,
    /// A business level version identifier of the substance
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Status of substance within the catalogue e.g. active, retired
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// A categorization, high level e.g. polymer or nucleic acid, or food, chemical, biological, or lower e.g. polymer linear or branch chain, or type of impurity
    pub classification: Option<Vec<CodeableConcept>>,
    /// If the substance applies to human or veterinary use
    ///
    /// Binding: example (Applicable domain for this product (e.g. human, veterinary).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicinal-product-domain
    pub domain: Option<CodeableConcept>,
    /// The quality standard, established benchmark, to which substance complies (e.g. USP/NF, BP)
    ///
    /// Binding: example (The quality standard, established benchmark, to which a substance complies)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-grade
    pub grade: Option<Vec<CodeableConcept>>,
    /// Textual description of the substance
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Supporting literature
    #[serde(rename = "informationSource")]
    pub information_source: Option<Vec<Reference>>,
    /// Textual comment about the substance's catalogue or registry record
    pub note: Option<Vec<Annotation>>,
    /// The entity that creates, makes, produces or fabricates the substance
    pub manufacturer: Option<Vec<Reference>>,
    /// An entity that is the source for the substance. It may be different from the manufacturer
    pub supplier: Option<Vec<Reference>>,
    /// Moiety, for structural modifications
    pub moiety: Option<Vec<SubstanceDefinitionMoiety>>,
    /// General specifications for this substance
    pub characterization: Option<Vec<SubstanceDefinitionCharacterization>>,
    /// General specifications for this substance
    pub property: Option<Vec<SubstanceDefinitionProperty>>,
    /// General information detailing this substance
    #[serde(rename = "referenceInformation")]
    pub reference_information: Option<Reference>,
    /// The average mass of a molecule of a compound
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<Vec<SubstanceDefinitionMolecularweight>>,
    /// Structural information
    pub structure: Option<SubstanceDefinitionStructure>,
    /// Codes associated with the substance
    pub code: Option<Vec<SubstanceDefinitionCode>>,
    /// Names applicable to this substance
    pub name: Option<Vec<SubstanceDefinitionName>>,
    /// A link between this substance and another
    pub relationship: Option<Vec<SubstanceDefinitionRelationship>>,
    /// Data items specific to nucleic acids
    #[serde(rename = "nucleicAcid")]
    pub nucleic_acid: Option<Reference>,
    /// Data items specific to polymers
    pub polymer: Option<Reference>,
    /// Data items specific to proteins
    pub protein: Option<Reference>,
    /// Material or taxonomic/anatomical source
    #[serde(rename = "sourceMaterial")]
    pub source_material: Option<SubstanceDefinitionSourcematerial>,
}
/// SubstanceDefinition nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Details of the official nature of this name
    pub official: Option<Vec<SubstanceDefinitionNameOfficial>>,
    /// The actual name
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name type e.g. 'systematic',  'scientific, 'brand'
    ///
    /// Binding: example (The type of a name given to a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-name-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The status of the name e.g. 'current', 'proposed'
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// If this is the preferred name for this substance
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
    /// Human language that the name is written in
    pub language: Option<Vec<StringType>>,
    /// The use context of this name e.g. as an active ingredient or as a food colour additive
    ///
    /// Binding: example (The use context of a substance name for example if there is a different name when used as a drug active ingredient as opposed to a food colour additive.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-name-domain
    pub domain: Option<Vec<CodeableConcept>>,
    /// The jurisdiction where this name applies
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// A synonym of this particular name, by which the substance is also known
    pub synonym: Option<Vec<StringType>>,
    /// A translation for this name into another human language
    pub translation: Option<Vec<StringType>>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceDefinition nested structure for the 'molecularWeight' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionMolecularweight {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The method by which the weight was determined
    ///
    /// Binding: example (The method by which the substance weight was measured.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-weight-method
    pub method: Option<CodeableConcept>,
    /// Type of molecular weight e.g. exact, average, weight average
    ///
    /// Binding: example (The type of substance weight measurement.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-weight-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Used to capture quantitative values for a variety of elements
    pub amount: Quantity,
}
/// SubstanceDefinition nested structure for the 'code' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specific code
    pub code: Option<CodeableConcept>,
    /// Status of the code assignment, for example 'provisional', 'approved'
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// The date at which the code status was changed
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// Any comment can be provided in this field
    pub note: Option<Vec<Annotation>>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceDefinition nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code expressing the type of property
    ///
    /// Binding: example (This value set includes all observable entity codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-characteristic-codes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// A value for the property (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// A value for the property (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// A value for the property (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// A value for the property (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// A value for the property (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
}
/// SubstanceDefinition nested structure for the 'sourceMaterial' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionSourcematerial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Classification of the origin of the raw material. e.g. cat hair is an Animal source type
    ///
    /// Binding: example (A classification that provides the origin of the substance raw material.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-source-material-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The genus of an organism e.g. the Latin epithet of the plant/animal scientific name
    ///
    /// Binding: example (The genus of an organism, typically referring to the Latin epithet of the genus element of the plant/animal scientific name.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-source-material-genus
    pub genus: Option<CodeableConcept>,
    /// The species of an organism e.g. the Latin epithet of the species of the plant/animal
    ///
    /// Binding: example (A species of origin a substance raw material.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-source-material-species
    pub species: Option<CodeableConcept>,
    /// An anatomical origin of the source material within an organism
    ///
    /// Binding: example (An anatomical origin of the source material within an organism.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-source-material-part
    pub part: Option<CodeableConcept>,
    /// The country or countries where the material is harvested
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<Vec<CodeableConcept>>,
}
/// SubstanceDefinition nested structure for the 'characterization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionCharacterization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The method used to find the characterization e.g. HPLC
    ///
    /// Binding: example (The method used to elucidate the characterization of the drug substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-structure-technique
    pub technique: Option<CodeableConcept>,
    /// Describes the nature of the chemical entity and explains, for instance, whether this is a base or a salt form
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-form
    pub form: Option<CodeableConcept>,
    /// The description or justification in support of the interpretation of the data file
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The data produced by the analytical instrument or a pictorial representation of that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum analysis
    pub file: Option<Vec<Attachment>>,
}
/// SubstanceDefinition nested structure for the 'relationship' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionRelationship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A pointer to another substance, as a resource or a representational code (Reference)
    #[serde(rename = "substanceDefinitionReference")]
    pub substance_definition_reference: Option<Reference>,
    /// A pointer to another substance, as a resource or a representational code (CodeableConcept)
    #[serde(rename = "substanceDefinitionCodeableConcept")]
    pub substance_definition_codeable_concept: Option<CodeableConcept>,
    /// For example "salt to parent", "active moiety"
    ///
    /// Binding: example (The relationship between two substance types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-relationship-type
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible relationships
    #[serde(rename = "isDefining")]
    pub is_defining: Option<BooleanType>,
    /// Extension element for the 'isDefining' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDefining")]
    pub _is_defining: Option<Element>,
    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other (Ratio)
    #[serde(rename = "amountRatio")]
    pub amount_ratio: Option<Ratio>,
    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
    /// For use when the numeric has an uncertain range
    #[serde(rename = "ratioHighLimitAmount")]
    pub ratio_high_limit_amount: Option<Ratio>,
    /// An operator for the amount, for example "average", "approximately", "less than"
    ///
    /// Binding: example (The relationship between two substance types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-amount-type
    pub comparator: Option<CodeableConcept>,
    /// Supporting literature
    pub source: Option<Vec<Reference>>,
}
/// SubstanceDefinition nested structure for the 'structure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A depiction of the structure of the substance
    pub representation: Option<Vec<SubstanceDefinitionStructureRepresentation>>,
    /// Stereochemistry type
    ///
    /// Binding: example (The optical rotation type of a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-stereochemistry
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    ///
    /// Binding: example (The optical rotation type of a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-optical-activity
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,
    /// An expression which states the number and type of atoms present in a molecule of a substance
    #[serde(rename = "molecularFormula")]
    pub molecular_formula: Option<StringType>,
    /// Extension element for the 'molecularFormula' primitive field. Contains metadata and extensions.
    #[serde(rename = "_molecularFormula")]
    pub _molecular_formula: Option<Element>,
    /// Specified per moiety according to the Hill system
    #[serde(rename = "molecularFormulaByMoiety")]
    pub molecular_formula_by_moiety: Option<StringType>,
    /// Extension element for the 'molecularFormulaByMoiety' primitive field. Contains metadata and extensions.
    #[serde(rename = "_molecularFormulaByMoiety")]
    pub _molecular_formula_by_moiety: Option<Element>,
    /// The molecular weight or weight range
    #[serde(rename = "molecularWeight")]
    pub molecular_weight: Option<StringType>,
    /// The method used to find the structure e.g. X-ray, NMR
    ///
    /// Binding: example (The method used to elucidate the structure of the drug substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-structure-technique
    pub technique: Option<Vec<CodeableConcept>>,
    /// Source of information for the structure
    #[serde(rename = "sourceDocument")]
    pub source_document: Option<Vec<Reference>>,
}
/// SubstanceDefinitionStructure nested structure for the 'representation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionStructureRepresentation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The kind of structural representation (e.g. full, partial)
    ///
    /// Binding: example (A format of a substance representation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-representation-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The structural representation as a text string in a standard format
    pub representation: Option<StringType>,
    /// Extension element for the 'representation' primitive field. Contains metadata and extensions.
    pub _representation: Option<Element>,
    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not the physical file format)
    ///
    /// Binding: example (A format of a substance representation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-representation-format
    pub format: Option<CodeableConcept>,
    /// An attachment with the structural representation e.g. a structure graphic or AnIML file
    pub document: Option<Reference>,
}
/// SubstanceDefinitionName nested structure for the 'official' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionNameOfficial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Which authority uses this official name
    ///
    /// Binding: preferred (An authority that officates substance names.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-name-authority
    pub authority: Option<CodeableConcept>,
    /// The status of the official name, for example 'draft', 'active'
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// Date of official name change
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
}
/// SubstanceDefinition nested structure for the 'moiety' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceDefinitionMoiety {
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
    ///
    /// Binding: example (The optical rotation type of a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-stereochemistry
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    ///
    /// Binding: example (The optical rotation type of a substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-optical-activity
    #[serde(rename = "opticalActivity")]
    pub optical_activity: Option<CodeableConcept>,
    /// Molecular formula for this moiety (e.g. with the Hill system)
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
    /// The measurement type of the quantitative value
    ///
    /// Binding: example (The relationship between two substance types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/substance-amount-type
    #[serde(rename = "measurementType")]
    pub measurement_type: Option<CodeableConcept>,
}

impl Default for SubstanceDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            status: Default::default(),
            classification: Default::default(),
            domain: Default::default(),
            grade: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            information_source: Default::default(),
            note: Default::default(),
            manufacturer: Default::default(),
            supplier: Default::default(),
            moiety: Default::default(),
            characterization: Default::default(),
            property: Default::default(),
            reference_information: Default::default(),
            molecular_weight: Default::default(),
            structure: Default::default(),
            code: Default::default(),
            name: Default::default(),
            relationship: Default::default(),
            nucleic_acid: Default::default(),
            polymer: Default::default(),
            protein: Default::default(),
            source_material: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionName {
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

impl Default for SubstanceDefinitionMolecularweight {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            method: Default::default(),
            type_: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionCode {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            note: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_date: Default::default(),
            value_boolean: Default::default(),
            value_attachment: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionSourcematerial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            genus: Default::default(),
            species: Default::default(),
            part: Default::default(),
            country_of_origin: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionCharacterization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            technique: Default::default(),
            form: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            file: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionRelationship {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance_definition_reference: Default::default(),
            substance_definition_codeable_concept: Default::default(),
            type_: Default::default(),
            is_defining: Default::default(),
            _is_defining: Default::default(),
            amount_quantity: Default::default(),
            amount_ratio: Default::default(),
            amount_string: Default::default(),
            ratio_high_limit_amount: Default::default(),
            comparator: Default::default(),
            source: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionStructure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            representation: Default::default(),
            stereochemistry: Default::default(),
            optical_activity: Default::default(),
            molecular_formula: Default::default(),
            _molecular_formula: Default::default(),
            molecular_formula_by_moiety: Default::default(),
            _molecular_formula_by_moiety: Default::default(),
            molecular_weight: Default::default(),
            technique: Default::default(),
            source_document: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionStructureRepresentation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            representation: Default::default(),
            _representation: Default::default(),
            format: Default::default(),
            document: Default::default(),
        }
    }
}

impl Default for SubstanceDefinitionNameOfficial {
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

impl Default for SubstanceDefinitionMoiety {
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
            measurement_type: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "SubstanceDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "SubstanceDefinition.name.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "SubstanceDefinition.sourceMaterial.countryOfOrigin",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/country|5.0.0",
            )
            .with_description("Jurisdiction codes"),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SubstanceDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.classification", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.domain", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.grade", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.informationSource",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.note", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.manufacturer", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.supplier", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.moiety", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.moiety.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.moiety.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.moiety.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.identifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.moiety.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.stereochemistry",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.opticalActivity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.molecularFormula",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.amount[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.moiety.measurementType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.characterization", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.technique",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.form",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.characterization.file",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.property", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.property.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.referenceInformation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.molecularWeight", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.method",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.molecularWeight.amount",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.structure", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.structure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.stereochemistry",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.opticalActivity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.molecularFormula",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.molecularFormulaByMoiety",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.molecularWeight",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.technique",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.sourceDocument",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.representation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.format",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.structure.representation.document",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.code.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.code.statusDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.note", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.code.source", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.preferred",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.language", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.domain", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.jurisdiction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.synonym", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.translation", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.official", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.authority",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.status",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.name.official.date",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.name.source", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.relationship", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.substanceDefinition[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.isDefining",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.amount[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.ratioHighLimitAmount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.comparator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.relationship.source",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.nucleicAcid", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.polymer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceDefinition.protein", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.genus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.species",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.part",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceDefinition.sourceMaterial.countryOfOrigin",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubstanceDefinition {
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

impl crate::traits::resource::ResourceMutators for SubstanceDefinition {
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

impl crate::traits::resource::ResourceExistence for SubstanceDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubstanceDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubstanceDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubstanceDefinition {
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

impl crate::traits::substance_definition::SubstanceDefinitionAccessors for SubstanceDefinition {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn classification(&self) -> &[CodeableConcept] {
        self.classification.as_deref().unwrap_or(&[])
    }
    fn domain(&self) -> Option<CodeableConcept> {
        self.domain.clone()
    }
    fn grade(&self) -> &[CodeableConcept] {
        self.grade.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn information_source(&self) -> &[Reference] {
        self.information_source.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn supplier(&self) -> &[Reference] {
        self.supplier.as_deref().unwrap_or(&[])
    }
    fn moiety(&self) -> &[SubstanceDefinitionMoiety] {
        self.moiety.as_deref().unwrap_or(&[])
    }
    fn characterization(&self) -> &[SubstanceDefinitionCharacterization] {
        self.characterization.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[SubstanceDefinitionProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn reference_information(&self) -> Option<Reference> {
        self.reference_information.clone()
    }
    fn molecular_weight(&self) -> &[SubstanceDefinitionMolecularweight] {
        self.molecular_weight.as_deref().unwrap_or(&[])
    }
    fn structure(&self) -> Option<SubstanceDefinitionStructure> {
        self.structure.clone()
    }
    fn code(&self) -> &[SubstanceDefinitionCode] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[SubstanceDefinitionName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn relationship(&self) -> &[SubstanceDefinitionRelationship] {
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
    fn source_material(&self) -> Option<SubstanceDefinitionSourcematerial> {
        self.source_material.clone()
    }
}

impl crate::traits::substance_definition::SubstanceDefinitionMutators for SubstanceDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_classification(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.classification = Some(value);
        resource
    }
    fn add_classification(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .classification
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_domain(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.domain = Some(value);
        resource
    }
    fn set_grade(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.grade = Some(value);
        resource
    }
    fn add_grade(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.grade.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_information_source(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.information_source = Some(value);
        resource
    }
    fn add_information_source(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .information_source
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_manufacturer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn add_manufacturer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .manufacturer
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_supplier(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supplier = Some(value);
        resource
    }
    fn add_supplier(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.supplier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_moiety(self, value: Vec<SubstanceDefinitionMoiety>) -> Self {
        let mut resource = self.clone();
        resource.moiety = Some(value);
        resource
    }
    fn add_moiety(self, item: SubstanceDefinitionMoiety) -> Self {
        let mut resource = self.clone();
        resource.moiety.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_characterization(self, value: Vec<SubstanceDefinitionCharacterization>) -> Self {
        let mut resource = self.clone();
        resource.characterization = Some(value);
        resource
    }
    fn add_characterization(self, item: SubstanceDefinitionCharacterization) -> Self {
        let mut resource = self.clone();
        resource
            .characterization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_property(self, value: Vec<SubstanceDefinitionProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: SubstanceDefinitionProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reference_information(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.reference_information = Some(value);
        resource
    }
    fn set_molecular_weight(self, value: Vec<SubstanceDefinitionMolecularweight>) -> Self {
        let mut resource = self.clone();
        resource.molecular_weight = Some(value);
        resource
    }
    fn add_molecular_weight(self, item: SubstanceDefinitionMolecularweight) -> Self {
        let mut resource = self.clone();
        resource
            .molecular_weight
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_structure(self, value: SubstanceDefinitionStructure) -> Self {
        let mut resource = self.clone();
        resource.structure = Some(value);
        resource
    }
    fn set_code(self, value: Vec<SubstanceDefinitionCode>) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: SubstanceDefinitionCode) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: Vec<SubstanceDefinitionName>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: SubstanceDefinitionName) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_relationship(self, value: Vec<SubstanceDefinitionRelationship>) -> Self {
        let mut resource = self.clone();
        resource.relationship = Some(value);
        resource
    }
    fn add_relationship(self, item: SubstanceDefinitionRelationship) -> Self {
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
    fn set_source_material(self, value: SubstanceDefinitionSourcematerial) -> Self {
        let mut resource = self.clone();
        resource.source_material = Some(value);
        resource
    }
}

impl crate::traits::substance_definition::SubstanceDefinitionExistence for SubstanceDefinition {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_classification(&self) -> bool {
        self.classification.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_domain(&self) -> bool {
        self.domain.is_some()
    }
    fn has_grade(&self) -> bool {
        self.grade.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_information_source(&self) -> bool {
        self.information_source
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supplier(&self) -> bool {
        self.supplier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_moiety(&self) -> bool {
        self.moiety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characterization(&self) -> bool {
        self.characterization
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reference_information(&self) -> bool {
        self.reference_information.is_some()
    }
    fn has_molecular_weight(&self) -> bool {
        self.molecular_weight
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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

impl crate::validation::ValidatableResource for SubstanceDefinition {
    fn resource_type(&self) -> &'static str {
        "SubstanceDefinition"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SubstanceDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::substance_definition::{
    SubstanceDefinitionAccessors, SubstanceDefinitionExistence, SubstanceDefinitionMutators,
};
