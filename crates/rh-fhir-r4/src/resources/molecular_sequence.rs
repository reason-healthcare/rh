use crate::bindings::orientation_type::OrientationType;
use crate::bindings::quality_type::QualityType;
use crate::bindings::repository_type::RepositoryType;
use crate::bindings::sequence_type::SequenceType;
use crate::bindings::strand_type::StrandType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MolecularSequence
///
/// Raw data describing a biological sequence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MolecularSequence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique ID for this particular sequence. This is a FHIR-defined id
    pub identifier: Option<Vec<Identifier>>,
    /// aa | dna | rna
    #[serde(rename = "type")]
    pub type_: Option<SequenceType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Base number of coordinate system (0 for 0-based numbering or coordinates, inclusive start, exclusive end, 1 for 1-based numbering, inclusive start, inclusive end)
    #[serde(rename = "coordinateSystem")]
    pub coordinate_system: IntegerType,
    /// Extension element for the 'coordinateSystem' primitive field. Contains metadata and extensions.
    #[serde(rename = "_coordinateSystem")]
    pub _coordinate_system: Option<Element>,
    /// Who and/or what this is about
    pub patient: Option<Reference>,
    /// Specimen used for sequencing
    pub specimen: Option<Reference>,
    /// The method for sequencing
    pub device: Option<Reference>,
    /// Who should be responsible for test result
    pub performer: Option<Reference>,
    /// The number of copies of the sequence of interest.  (RNASeq)
    pub quantity: Option<Quantity>,
    /// A sequence used as reference
    #[serde(rename = "referenceSeq")]
    pub reference_seq: Option<MolecularSequenceReferenceseq>,
    /// Variant in sequence
    pub variant: Option<Vec<MolecularSequenceVariant>>,
    /// Sequence that was observed
    #[serde(rename = "observedSeq")]
    pub observed_seq: Option<StringType>,
    /// Extension element for the 'observedSeq' primitive field. Contains metadata and extensions.
    #[serde(rename = "_observedSeq")]
    pub _observed_seq: Option<Element>,
    /// An set of value as quality of sequence
    pub quality: Option<Vec<MolecularSequenceQuality>>,
    /// Average number of reads representing a given nucleotide in the reconstructed sequence
    #[serde(rename = "readCoverage")]
    pub read_coverage: Option<IntegerType>,
    /// Extension element for the 'readCoverage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_readCoverage")]
    pub _read_coverage: Option<Element>,
    /// External repository which contains detailed report related with observedSeq in this resource
    pub repository: Option<Vec<MolecularSequenceRepository>>,
    /// Pointer to next atomic sequence
    pub pointer: Option<Vec<Reference>>,
    /// Structural variant
    #[serde(rename = "structureVariant")]
    pub structure_variant: Option<Vec<MolecularSequenceStructurevariant>>,
}
/// MolecularSequenceStructurevariant nested structure for the 'inner' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceStructurevariantInner {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Structural variant inner start
    pub start: Option<IntegerType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// Structural variant inner end
    pub end: Option<IntegerType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
}
/// MolecularSequence nested structure for the 'variant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceVariant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Start position of the variant on the  reference sequence
    pub start: Option<IntegerType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// End position of the variant on the reference sequence
    pub end: Option<IntegerType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Allele that was observed
    #[serde(rename = "observedAllele")]
    pub observed_allele: Option<StringType>,
    /// Extension element for the 'observedAllele' primitive field. Contains metadata and extensions.
    #[serde(rename = "_observedAllele")]
    pub _observed_allele: Option<Element>,
    /// Allele in the reference sequence
    #[serde(rename = "referenceAllele")]
    pub reference_allele: Option<StringType>,
    /// Extension element for the 'referenceAllele' primitive field. Contains metadata and extensions.
    #[serde(rename = "_referenceAllele")]
    pub _reference_allele: Option<Element>,
    /// Extended CIGAR string for aligning the sequence with reference bases
    pub cigar: Option<StringType>,
    /// Extension element for the 'cigar' primitive field. Contains metadata and extensions.
    pub _cigar: Option<Element>,
    /// Pointer to observed variant information
    #[serde(rename = "variantPointer")]
    pub variant_pointer: Option<Reference>,
}
/// MolecularSequenceStructurevariant nested structure for the 'outer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceStructurevariantOuter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Structural variant outer start
    pub start: Option<IntegerType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// Structural variant outer end
    pub end: Option<IntegerType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
}
/// MolecularSequence nested structure for the 'quality' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceQuality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Receiver Operator Characteristic (ROC) Curve
    pub roc: Option<MolecularSequenceQualityRoc>,
    /// indel | snp | unknown
    #[serde(rename = "type")]
    pub type_: QualityType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Standard sequence for comparison
    ///
    /// Binding: example (Reference identifier of the sequence that used to mark the quality of tested samples.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/sequence-quality-standardSequence
    #[serde(rename = "standardSequence")]
    pub standard_sequence: Option<CodeableConcept>,
    /// Start position of the sequence
    pub start: Option<IntegerType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// End position of the sequence
    pub end: Option<IntegerType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Quality score for the comparison
    pub score: Option<Quantity>,
    /// Method to get quality
    ///
    /// Binding: example (The method used to evaluate the numerical quality of the observed sequence.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/sequence-quality-method
    pub method: Option<CodeableConcept>,
    /// True positives from the perspective of the truth data
    #[serde(rename = "truthTP")]
    pub truth_t_p: Option<DecimalType>,
    /// Extension element for the 'truthTP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_truthTP")]
    pub _truth_t_p: Option<Element>,
    /// True positives from the perspective of the query data
    #[serde(rename = "queryTP")]
    pub query_t_p: Option<DecimalType>,
    /// Extension element for the 'queryTP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_queryTP")]
    pub _query_t_p: Option<Element>,
    /// False negatives
    #[serde(rename = "truthFN")]
    pub truth_f_n: Option<DecimalType>,
    /// Extension element for the 'truthFN' primitive field. Contains metadata and extensions.
    #[serde(rename = "_truthFN")]
    pub _truth_f_n: Option<Element>,
    /// False positives
    #[serde(rename = "queryFP")]
    pub query_f_p: Option<DecimalType>,
    /// Extension element for the 'queryFP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_queryFP")]
    pub _query_f_p: Option<Element>,
    /// False positives where the non-REF alleles in the Truth and Query Call Sets match
    #[serde(rename = "gtFP")]
    pub gt_f_p: Option<DecimalType>,
    /// Extension element for the 'gtFP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_gtFP")]
    pub _gt_f_p: Option<Element>,
    /// Precision of comparison
    pub precision: Option<DecimalType>,
    /// Extension element for the 'precision' primitive field. Contains metadata and extensions.
    pub _precision: Option<Element>,
    /// Recall of comparison
    pub recall: Option<DecimalType>,
    /// Extension element for the 'recall' primitive field. Contains metadata and extensions.
    pub _recall: Option<Element>,
    /// F-score
    #[serde(rename = "fScore")]
    pub f_score: Option<DecimalType>,
    /// Extension element for the 'fScore' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fScore")]
    pub _f_score: Option<Element>,
}
/// MolecularSequence nested structure for the 'structureVariant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceStructurevariant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Structural variant outer
    pub outer: Option<MolecularSequenceStructurevariantOuter>,
    /// Structural variant inner
    pub inner: Option<MolecularSequenceStructurevariantInner>,
    /// Structural variant change type
    #[serde(rename = "variantType")]
    pub variant_type: Option<CodeableConcept>,
    /// Does the structural variant have base pair resolution breakpoints?
    pub exact: Option<BooleanType>,
    /// Extension element for the 'exact' primitive field. Contains metadata and extensions.
    pub _exact: Option<Element>,
    /// Structural variant length
    pub length: Option<IntegerType>,
    /// Extension element for the 'length' primitive field. Contains metadata and extensions.
    pub _length: Option<Element>,
}
/// MolecularSequenceQuality nested structure for the 'roc' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceQualityRoc {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Genotype quality score
    pub score: Option<Vec<IntegerType>>,
    /// Extension element for the 'score' primitive field. Contains metadata and extensions.
    pub _score: Option<Element>,
    /// Roc score true positive numbers
    #[serde(rename = "numTP")]
    pub num_t_p: Option<Vec<IntegerType>>,
    /// Extension element for the 'numTP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numTP")]
    pub _num_t_p: Option<Element>,
    /// Roc score false positive numbers
    #[serde(rename = "numFP")]
    pub num_f_p: Option<Vec<IntegerType>>,
    /// Extension element for the 'numFP' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numFP")]
    pub _num_f_p: Option<Element>,
    /// Roc score false negative numbers
    #[serde(rename = "numFN")]
    pub num_f_n: Option<Vec<IntegerType>>,
    /// Extension element for the 'numFN' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numFN")]
    pub _num_f_n: Option<Element>,
    /// Precision of the GQ score
    pub precision: Option<Vec<DecimalType>>,
    /// Extension element for the 'precision' primitive field. Contains metadata and extensions.
    pub _precision: Option<Element>,
    /// Sensitivity of the GQ score
    pub sensitivity: Option<Vec<DecimalType>>,
    /// Extension element for the 'sensitivity' primitive field. Contains metadata and extensions.
    pub _sensitivity: Option<Element>,
    /// FScore of the GQ score
    #[serde(rename = "fMeasure")]
    pub f_measure: Option<Vec<DecimalType>>,
    /// Extension element for the 'fMeasure' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fMeasure")]
    pub _f_measure: Option<Element>,
}
/// MolecularSequence nested structure for the 'referenceSeq' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceReferenceseq {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Chromosome containing genetic finding
    ///
    /// Binding: example (Chromosome number for human.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/chromosome-human
    pub chromosome: Option<CodeableConcept>,
    /// The Genome Build used for reference, following GRCh build versions e.g. 'GRCh 37'
    #[serde(rename = "genomeBuild")]
    pub genome_build: Option<StringType>,
    /// Extension element for the 'genomeBuild' primitive field. Contains metadata and extensions.
    #[serde(rename = "_genomeBuild")]
    pub _genome_build: Option<Element>,
    /// sense | antisense
    pub orientation: Option<OrientationType>,
    /// Extension element for the 'orientation' primitive field. Contains metadata and extensions.
    pub _orientation: Option<Element>,
    /// Reference identifier
    ///
    /// Binding: example (Reference identifier.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/sequence-referenceSeq
    #[serde(rename = "referenceSeqId")]
    pub reference_seq_id: Option<CodeableConcept>,
    /// A pointer to another MolecularSequence entity as reference sequence
    #[serde(rename = "referenceSeqPointer")]
    pub reference_seq_pointer: Option<Reference>,
    /// A string to represent reference sequence
    #[serde(rename = "referenceSeqString")]
    pub reference_seq_string: Option<StringType>,
    /// Extension element for the 'referenceSeqString' primitive field. Contains metadata and extensions.
    #[serde(rename = "_referenceSeqString")]
    pub _reference_seq_string: Option<Element>,
    /// watson | crick
    pub strand: Option<StrandType>,
    /// Extension element for the 'strand' primitive field. Contains metadata and extensions.
    pub _strand: Option<Element>,
    /// Start position of the window on the  reference sequence
    #[serde(rename = "windowStart")]
    pub window_start: Option<IntegerType>,
    /// Extension element for the 'windowStart' primitive field. Contains metadata and extensions.
    #[serde(rename = "_windowStart")]
    pub _window_start: Option<Element>,
    /// End position of the window on the reference sequence
    #[serde(rename = "windowEnd")]
    pub window_end: Option<IntegerType>,
    /// Extension element for the 'windowEnd' primitive field. Contains metadata and extensions.
    #[serde(rename = "_windowEnd")]
    pub _window_end: Option<Element>,
}
/// MolecularSequence nested structure for the 'repository' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSequenceRepository {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// directlink | openapi | login | oauth | other
    #[serde(rename = "type")]
    pub type_: RepositoryType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// URI of the repository
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Repository's name
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Id of the dataset that used to call for dataset in repository
    #[serde(rename = "datasetId")]
    pub dataset_id: Option<StringType>,
    /// Extension element for the 'datasetId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_datasetId")]
    pub _dataset_id: Option<Element>,
    /// Id of the variantset that used to call for variantset in repository
    #[serde(rename = "variantsetId")]
    pub variantset_id: Option<StringType>,
    /// Extension element for the 'variantsetId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_variantsetId")]
    pub _variantset_id: Option<Element>,
    /// Id of the read
    #[serde(rename = "readsetId")]
    pub readset_id: Option<StringType>,
    /// Extension element for the 'readsetId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_readsetId")]
    pub _readset_id: Option<Element>,
}

impl Default for MolecularSequence {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            coordinate_system: IntegerType::default(),
            _coordinate_system: Default::default(),
            patient: Default::default(),
            specimen: Default::default(),
            device: Default::default(),
            performer: Default::default(),
            quantity: Default::default(),
            reference_seq: Default::default(),
            variant: Default::default(),
            observed_seq: Default::default(),
            _observed_seq: Default::default(),
            quality: Default::default(),
            read_coverage: Default::default(),
            _read_coverage: Default::default(),
            repository: Default::default(),
            pointer: Default::default(),
            structure_variant: Default::default(),
        }
    }
}

impl Default for MolecularSequenceStructurevariantInner {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
        }
    }
}

impl Default for MolecularSequenceVariant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            observed_allele: Default::default(),
            _observed_allele: Default::default(),
            reference_allele: Default::default(),
            _reference_allele: Default::default(),
            cigar: Default::default(),
            _cigar: Default::default(),
            variant_pointer: Default::default(),
        }
    }
}

impl Default for MolecularSequenceStructurevariantOuter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
        }
    }
}

impl Default for MolecularSequenceQuality {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            roc: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            standard_sequence: Default::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            score: Default::default(),
            method: Default::default(),
            truth_t_p: Default::default(),
            _truth_t_p: Default::default(),
            query_t_p: Default::default(),
            _query_t_p: Default::default(),
            truth_f_n: Default::default(),
            _truth_f_n: Default::default(),
            query_f_p: Default::default(),
            _query_f_p: Default::default(),
            gt_f_p: Default::default(),
            _gt_f_p: Default::default(),
            precision: Default::default(),
            _precision: Default::default(),
            recall: Default::default(),
            _recall: Default::default(),
            f_score: Default::default(),
            _f_score: Default::default(),
        }
    }
}

impl Default for MolecularSequenceStructurevariant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            outer: Default::default(),
            inner: Default::default(),
            variant_type: Default::default(),
            exact: Default::default(),
            _exact: Default::default(),
            length: Default::default(),
            _length: Default::default(),
        }
    }
}

impl Default for MolecularSequenceQualityRoc {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            score: Default::default(),
            _score: Default::default(),
            num_t_p: Default::default(),
            _num_t_p: Default::default(),
            num_f_p: Default::default(),
            _num_f_p: Default::default(),
            num_f_n: Default::default(),
            _num_f_n: Default::default(),
            precision: Default::default(),
            _precision: Default::default(),
            sensitivity: Default::default(),
            _sensitivity: Default::default(),
            f_measure: Default::default(),
            _f_measure: Default::default(),
        }
    }
}

impl Default for MolecularSequenceReferenceseq {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            chromosome: Default::default(),
            genome_build: Default::default(),
            _genome_build: Default::default(),
            orientation: Default::default(),
            _orientation: Default::default(),
            reference_seq_id: Default::default(),
            reference_seq_pointer: Default::default(),
            reference_seq_string: Default::default(),
            _reference_seq_string: Default::default(),
            strand: Default::default(),
            _strand: Default::default(),
            window_start: Default::default(),
            _window_start: Default::default(),
            window_end: Default::default(),
            _window_end: Default::default(),
        }
    }
}

impl Default for MolecularSequenceRepository {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            dataset_id: Default::default(),
            _dataset_id: Default::default(),
            variantset_id: Default::default(),
            _variantset_id: Default::default(),
            readset_id: Default::default(),
            _readset_id: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MolecularSequence {
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

impl crate::traits::resource::ResourceMutators for MolecularSequence {
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

impl crate::traits::resource::ResourceExistence for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceMutators for MolecularSequence {
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

impl crate::traits::domain_resource::DomainResourceExistence for MolecularSequence {
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

impl crate::traits::molecular_sequence::MolecularSequenceAccessors for MolecularSequence {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<SequenceType> {
        self.type_.clone()
    }
    fn coordinate_system(&self) -> IntegerType {
        self.coordinate_system
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn specimen(&self) -> Option<Reference> {
        self.specimen.clone()
    }
    fn device(&self) -> Option<Reference> {
        self.device.clone()
    }
    fn performer(&self) -> Option<Reference> {
        self.performer.clone()
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn reference_seq(&self) -> Option<MolecularSequenceReferenceseq> {
        self.reference_seq.clone()
    }
    fn variant(&self) -> &[MolecularSequenceVariant] {
        self.variant.as_deref().unwrap_or(&[])
    }
    fn observed_seq(&self) -> Option<StringType> {
        self.observed_seq.clone()
    }
    fn quality(&self) -> &[MolecularSequenceQuality] {
        self.quality.as_deref().unwrap_or(&[])
    }
    fn read_coverage(&self) -> Option<IntegerType> {
        self.read_coverage
    }
    fn repository(&self) -> &[MolecularSequenceRepository] {
        self.repository.as_deref().unwrap_or(&[])
    }
    fn pointer(&self) -> &[Reference] {
        self.pointer.as_deref().unwrap_or(&[])
    }
    fn structure_variant(&self) -> &[MolecularSequenceStructurevariant] {
        self.structure_variant.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::molecular_sequence::MolecularSequenceMutators for MolecularSequence {
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
    fn set_type_(self, value: SequenceType) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_coordinate_system(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.coordinate_system = value;
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_specimen(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.specimen = Some(value);
        resource
    }
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = Some(value);
        resource
    }
    fn set_performer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_reference_seq(self, value: MolecularSequenceReferenceseq) -> Self {
        let mut resource = self.clone();
        resource.reference_seq = Some(value);
        resource
    }
    fn set_variant(self, value: Vec<MolecularSequenceVariant>) -> Self {
        let mut resource = self.clone();
        resource.variant = Some(value);
        resource
    }
    fn add_variant(self, item: MolecularSequenceVariant) -> Self {
        let mut resource = self.clone();
        resource.variant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_observed_seq(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.observed_seq = Some(value);
        resource
    }
    fn set_quality(self, value: Vec<MolecularSequenceQuality>) -> Self {
        let mut resource = self.clone();
        resource.quality = Some(value);
        resource
    }
    fn add_quality(self, item: MolecularSequenceQuality) -> Self {
        let mut resource = self.clone();
        resource.quality.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_read_coverage(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.read_coverage = Some(value);
        resource
    }
    fn set_repository(self, value: Vec<MolecularSequenceRepository>) -> Self {
        let mut resource = self.clone();
        resource.repository = Some(value);
        resource
    }
    fn add_repository(self, item: MolecularSequenceRepository) -> Self {
        let mut resource = self.clone();
        resource.repository.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_pointer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.pointer = Some(value);
        resource
    }
    fn add_pointer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.pointer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_structure_variant(self, value: Vec<MolecularSequenceStructurevariant>) -> Self {
        let mut resource = self.clone();
        resource.structure_variant = Some(value);
        resource
    }
    fn add_structure_variant(self, item: MolecularSequenceStructurevariant) -> Self {
        let mut resource = self.clone();
        resource
            .structure_variant
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::molecular_sequence::MolecularSequenceExistence for MolecularSequence {
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
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_coordinate_system(&self) -> bool {
        true
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_specimen(&self) -> bool {
        self.specimen.is_some()
    }
    fn has_device(&self) -> bool {
        self.device.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_reference_seq(&self) -> bool {
        self.reference_seq.is_some()
    }
    fn has_variant(&self) -> bool {
        self.variant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_observed_seq(&self) -> bool {
        self.observed_seq.is_some()
    }
    fn has_quality(&self) -> bool {
        self.quality.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_read_coverage(&self) -> bool {
        self.read_coverage.is_some()
    }
    fn has_repository(&self) -> bool {
        self.repository.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_pointer(&self) -> bool {
        self.pointer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_structure_variant(&self) -> bool {
        self.structure_variant
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
