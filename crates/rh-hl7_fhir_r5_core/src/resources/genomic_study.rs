use crate::bindings::genomicstudy_status::GenomicstudyStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// GenomicStudy
///
/// A set of analyses performed to analyze and generate genomic data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GenomicStudy
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifiers for this genomic study
    pub identifier: Option<Vec<Identifier>>,
    /// registered | available | cancelled | entered-in-error | unknown
    pub status: GenomicstudyStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The type of the study (e.g., Familial variant segregation, Functional variation detection, or Gene expression profiling)
    ///
    /// Binding: example (The type relevant to GenomicStudy.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The primary subject of the genomic study
    pub subject: Reference,
    /// The healthcare event with which this genomics study is associated
    pub encounter: Option<Reference>,
    /// When the genomic study was started
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTimeType>,
    /// Extension element for the 'startDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_startDate")]
    pub _start_date: Option<Element>,
    /// Event resources that the genomic study is based on
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Healthcare professional who requested or referred the genomic study
    pub referrer: Option<Reference>,
    /// Healthcare professionals who interpreted the genomic study
    pub interpreter: Option<Vec<Reference>>,
    /// Why the genomic study was performed
    pub reason: Option<Vec<CodeableReference>>,
    /// The defined protocol that describes the study
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<StringType>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// The URL pointing to an externally maintained protocol that describes the study
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Comments related to the genomic study
    pub note: Option<Vec<Annotation>>,
    /// Description of the genomic study
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Genomic Analysis Event
    pub analysis: Option<Vec<GenomicStudyAnalysis>>,
}
/// GenomicStudyAnalysis nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudyAnalysisPerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The organization, healthcare professional, or others who participated in performing this analysis
    pub actor: Option<Reference>,
    /// Role of the actor for this analysis
    pub role: Option<CodeableConcept>,
}
/// GenomicStudy nested structure for the 'analysis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudyAnalysis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Inputs for the analysis event
    pub input: Option<Vec<GenomicStudyAnalysisInput>>,
    /// Performer for the analysis event
    pub performer: Option<Vec<GenomicStudyAnalysisPerformer>>,
    /// Devices used for the analysis (e.g., instruments, software), with settings and parameters
    pub device: Option<Vec<GenomicStudyAnalysisDevice>>,
    /// Outputs for the analysis event
    pub output: Option<Vec<GenomicStudyAnalysisOutput>>,
    /// Identifiers for the analysis event
    pub identifier: Option<Vec<Identifier>>,
    /// Type of the methods used in the analysis (e.g., FISH, Karyotyping, MSI)
    ///
    /// Binding: example (The method type of the GenomicStudy analysis.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-methodtype
    #[serde(rename = "methodType")]
    pub method_type: Option<Vec<CodeableConcept>>,
    /// Type of the genomic changes studied in the analysis (e.g., DNA, RNA, or AA change)
    ///
    /// Binding: example (The change type relevant to GenomicStudy analysis.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-changetype
    #[serde(rename = "changeType")]
    pub change_type: Option<Vec<CodeableConcept>>,
    /// Genome build that is used in this analysis
    ///
    /// Binding: extensible (Human reference sequence NCBI build ID)
    ///
    /// ValueSet: http://loinc.org/vs/LL1040-6
    #[serde(rename = "genomeBuild")]
    pub genome_build: Option<CodeableConcept>,
    /// The defined protocol that describes the analysis
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<StringType>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// The URL pointing to an externally maintained protocol that describes the analysis
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Name of the analysis event (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// What the genomic analysis is about, when it is not about the subject of record
    pub focus: Option<Vec<Reference>>,
    /// The specimen used in the analysis event
    pub specimen: Option<Vec<Reference>>,
    /// The date of the analysis event
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Any notes capture with the analysis event
    pub note: Option<Vec<Annotation>>,
    /// The protocol that was performed for the analysis event
    #[serde(rename = "protocolPerformed")]
    pub protocol_performed: Option<Reference>,
    /// The genomic regions to be studied in the analysis (BED file)
    #[serde(rename = "regionsStudied")]
    pub regions_studied: Option<Vec<Reference>>,
    /// Genomic regions actually called in the analysis event (BED file)
    #[serde(rename = "regionsCalled")]
    pub regions_called: Option<Vec<Reference>>,
}
/// GenomicStudyAnalysis nested structure for the 'input' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudyAnalysisInput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// File containing input data
    pub file: Option<Reference>,
    /// Type of input data (e.g., BAM, CRAM, or FASTA)
    ///
    /// Binding: example (The data format of the data file.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-dataformat
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The analysis event or other GenomicStudy that generated this input file (Identifier)
    #[serde(rename = "generatedByIdentifier")]
    pub generated_by_identifier: Option<Identifier>,
    /// The analysis event or other GenomicStudy that generated this input file (Reference)
    #[serde(rename = "generatedByReference")]
    pub generated_by_reference: Option<Reference>,
}
/// GenomicStudyAnalysis nested structure for the 'output' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudyAnalysisOutput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// File containing output data
    pub file: Option<Reference>,
    /// Type of output data (e.g., VCF, MAF, or BAM)
    ///
    /// Binding: example (The data format of the data file.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/genomicstudy-dataformat
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
}
/// GenomicStudyAnalysis nested structure for the 'device' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicStudyAnalysisDevice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Device used for the analysis
    pub device: Option<Reference>,
    /// Specific function for the device used for the analysis
    pub function: Option<CodeableConcept>,
}

impl Default for GenomicStudy {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: GenomicstudyStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            start_date: Default::default(),
            _start_date: Default::default(),
            based_on: Default::default(),
            referrer: Default::default(),
            interpreter: Default::default(),
            reason: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            note: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            analysis: Default::default(),
        }
    }
}

impl Default for GenomicStudyAnalysisPerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            actor: Default::default(),
            role: Default::default(),
        }
    }
}

impl Default for GenomicStudyAnalysis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            input: Default::default(),
            performer: Default::default(),
            device: Default::default(),
            output: Default::default(),
            identifier: Default::default(),
            method_type: Default::default(),
            change_type: Default::default(),
            genome_build: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            focus: Default::default(),
            specimen: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            note: Default::default(),
            protocol_performed: Default::default(),
            regions_studied: Default::default(),
            regions_called: Default::default(),
        }
    }
}

impl Default for GenomicStudyAnalysisInput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            file: Default::default(),
            type_: Default::default(),
            generated_by_identifier: Default::default(),
            generated_by_reference: Default::default(),
        }
    }
}

impl Default for GenomicStudyAnalysisOutput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            file: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for GenomicStudyAnalysisDevice {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            device: Default::default(),
            function: Default::default(),
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
                "GenomicStudy.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "GenomicStudy.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/genomicstudy-status|5.0.0",
            )
            .with_description("The status of the GenomicStudy."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("GenomicStudy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.contained", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.extension", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.identifier", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.type", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.startDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.referrer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.interpreter", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.reason", 0, None),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.instantiatesCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.instantiatesUri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.note", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.identifier", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.methodType", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.changeType", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.genomeBuild", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.instantiatesCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.instantiatesUri",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.focus", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.specimen", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.protocolPerformed",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.regionsStudied", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.regionsCalled", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.input", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.input.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.input.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.input.file", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.input.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.input.generatedBy[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.output", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.output.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.output.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.output.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.output.file", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.output.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.performer", 0, None),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.performer.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.performer.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.performer.actor",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.performer.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.device", 0, None),
            rh_foundation::ElementCardinality::new("GenomicStudy.analysis.device.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.device.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.device.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.device.device",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GenomicStudy.analysis.device.function",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for GenomicStudy {
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

impl crate::traits::resource::ResourceMutators for GenomicStudy {
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

impl crate::traits::resource::ResourceExistence for GenomicStudy {
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

impl crate::traits::domain_resource::DomainResourceAccessors for GenomicStudy {
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

impl crate::traits::domain_resource::DomainResourceMutators for GenomicStudy {
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

impl crate::traits::domain_resource::DomainResourceExistence for GenomicStudy {
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

impl crate::traits::genomic_study::GenomicStudyAccessors for GenomicStudy {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> GenomicstudyStatus {
        self.status.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn start_date(&self) -> Option<DateTimeType> {
        self.start_date.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn referrer(&self) -> Option<Reference> {
        self.referrer.clone()
    }
    fn interpreter(&self) -> &[Reference] {
        self.interpreter.as_deref().unwrap_or(&[])
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> Option<StringType> {
        self.instantiates_canonical.clone()
    }
    fn instantiates_uri(&self) -> Option<StringType> {
        self.instantiates_uri.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn analysis(&self) -> &[GenomicStudyAnalysis] {
        self.analysis.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::genomic_study::GenomicStudyMutators for GenomicStudy {
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
    fn set_status(self, value: GenomicstudyStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_start_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.start_date = Some(value);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_referrer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.referrer = Some(value);
        resource
    }
    fn set_interpreter(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.interpreter = Some(value);
        resource
    }
    fn add_interpreter(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.interpreter.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_instantiates_canonical(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn set_instantiates_uri(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
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
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_analysis(self, value: Vec<GenomicStudyAnalysis>) -> Self {
        let mut resource = self.clone();
        resource.analysis = Some(value);
        resource
    }
    fn add_analysis(self, item: GenomicStudyAnalysis) -> Self {
        let mut resource = self.clone();
        resource.analysis.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::genomic_study::GenomicStudyExistence for GenomicStudy {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_start_date(&self) -> bool {
        self.start_date.is_some()
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_referrer(&self) -> bool {
        self.referrer.is_some()
    }
    fn has_interpreter(&self) -> bool {
        self.interpreter.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical.is_some()
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_analysis(&self) -> bool {
        self.analysis.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for GenomicStudy {
    fn resource_type(&self) -> &'static str {
        "GenomicStudy"
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
        Some("http://hl7.org/fhir/StructureDefinition/GenomicStudy")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::genomic_study::{
    GenomicStudyAccessors, GenomicStudyExistence, GenomicStudyMutators,
};
