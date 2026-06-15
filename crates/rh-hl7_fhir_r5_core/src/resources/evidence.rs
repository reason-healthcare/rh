use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::variable_handling::VariableHandling;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Evidence
///
/// The Evidence Resource provides a machine-interpretable expression of an evidence concept including the evidence variables (e.g., population, exposures/interventions, comparators, outcomes, measured variables, confounding variables), the statistics, and the certainty of this evidence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Evidence
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Evidence
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this evidence, represented as a globally unique URI
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the summary
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of this summary
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this summary (machine friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this summary (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Citation for this evidence (Reference)
    #[serde(rename = "citeAsReference")]
    pub cite_as_reference: Option<Reference>,
    /// Citation for this evidence (markdown)
    #[serde(rename = "citeAsMarkdown")]
    pub cite_as_markdown: Option<StringType>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// When the summary was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the summary was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<ContactDetail>,
    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<ContactDetail>,
    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<ContactDetail>,
    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<ContactDetail>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Why this Evidence is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// Link or citation to artifact associated with the summary
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// Description of the particular summary
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Declarative description of the Evidence
    pub assertion: Option<StringType>,
    /// Extension element for the 'assertion' primitive field. Contains metadata and extensions.
    pub _assertion: Option<Element>,
    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Evidence variable such as population, exposure, or outcome
    #[serde(rename = "variableDefinition")]
    pub variable_definition: Vec<EvidenceVariabledefinition>,
    /// The method to combine studies
    ///
    /// Binding: extensible (Types of combining results from a body of evidence (e.g. summary data meta-analysis).)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/synthesis-type
    #[serde(rename = "synthesisType")]
    pub synthesis_type: Option<CodeableConcept>,
    /// The design of the study that produced this evidence
    ///
    /// Binding: extensible (This is a set of terms for study design characteristics.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/study-design
    #[serde(rename = "studyDesign")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study_design: Vec<CodeableConcept>,
    /// Values and parameters for a single statistic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statistic: Vec<EvidenceStatistic>,
    /// Certainty or quality of the evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<EvidenceCertainty>,
}
/// EvidenceStatistic nested structure for the 'sampleSize' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStatisticSamplesize {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Textual description of sample size for statistic
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Footnote or explanatory note about the sample size
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Number of contributing studies
    #[serde(rename = "numberOfStudies")]
    pub number_of_studies: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfStudies' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfStudies")]
    pub _number_of_studies: Option<Element>,
    /// Cumulative number of participants
    #[serde(rename = "numberOfParticipants")]
    pub number_of_participants: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfParticipants' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfParticipants")]
    pub _number_of_participants: Option<Element>,
    /// Number of participants with known results for measured variables
    #[serde(rename = "knownDataCount")]
    pub known_data_count: Option<UnsignedIntType>,
    /// Extension element for the 'knownDataCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_knownDataCount")]
    pub _known_data_count: Option<Element>,
}
/// Evidence nested structure for the 'statistic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStatistic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// An attribute of the Statistic
    #[serde(rename = "attributeEstimate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeestimate>,
    /// An aspect of the statistical model
    #[serde(rename = "modelCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_characteristic: Vec<EvidenceStatisticModelcharacteristic>,
    /// Number of samples in the statistic
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<EvidenceStatisticSamplesize>,
    /// Description of content
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Type of statistic, e.g., relative risk
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/statistic-type
    #[serde(rename = "statisticType")]
    pub statistic_type: Option<CodeableConcept>,
    /// Associated category for categorical variable
    pub category: Option<CodeableConcept>,
    /// Statistic value
    pub quantity: Option<Quantity>,
    /// The number of events associated with the statistic
    #[serde(rename = "numberOfEvents")]
    pub number_of_events: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfEvents' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfEvents")]
    pub _number_of_events: Option<Element>,
    /// The number of participants affected
    #[serde(rename = "numberAffected")]
    pub number_affected: Option<UnsignedIntType>,
    /// Extension element for the 'numberAffected' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberAffected")]
    pub _number_affected: Option<Element>,
}
/// Evidence nested structure for the 'certainty' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCertainty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Textual description of certainty
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Aspect of certainty being rated
    ///
    /// Binding: extensible (The aspect of quality, confidence, or certainty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Assessment or judgement of the aspect
    ///
    /// Binding: extensible (The assessment of quality, confidence, or certainty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-rating
    pub rating: Option<CodeableConcept>,
    /// Individual or group who did the rating
    pub rater: Option<StringType>,
    /// Extension element for the 'rater' primitive field. Contains metadata and extensions.
    pub _rater: Option<Element>,
    /// A domain or subdomain of certainty
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subcomponent: Vec<StringType>,
}
/// Evidence nested structure for the 'variableDefinition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceVariabledefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A text description or summary of the variable
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// population | subpopulation | exposure | referenceExposure | measuredVariable | confounder
    ///
    /// Binding: extensible (The role that the assertion variable plays.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/variable-role
    #[serde(rename = "variableRole")]
    pub variable_role: CodeableConcept,
    /// Definition of the actual variable related to the statistic(s)
    pub observed: Option<Reference>,
    /// Definition of the intended variable related to the Evidence
    pub intended: Option<Reference>,
    /// low | moderate | high | exact
    ///
    /// Binding: extensible (The quality of how direct the match is.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/directness
    #[serde(rename = "directnessMatch")]
    pub directness_match: Option<CodeableConcept>,
}
/// EvidenceStatisticModelcharacteristic nested structure for the 'variable' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStatisticModelcharacteristicVariable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of the variable
    #[serde(rename = "variableDefinition")]
    pub variable_definition: Reference,
    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<VariableHandling>,
    /// Extension element for the 'handling' primitive field. Contains metadata and extensions.
    pub _handling: Option<Element>,
    /// Description for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueCategory")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_category: Vec<CodeableConcept>,
    /// Discrete value for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueQuantity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_quantity: Vec<Quantity>,
    /// Range of values for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueRange")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_range: Vec<Range>,
}
/// EvidenceStatistic nested structure for the 'attributeEstimate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStatisticAttributeestimate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Textual description of the attribute estimate
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Footnote or explanatory note about the estimate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// The type of attribute estimate, e.g., confidence interval or p value
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/attribute-estimate-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The singular quantity of the attribute estimate, for attribute estimates represented as single values; also used to report unit of measure
    pub quantity: Option<Quantity>,
    /// Level of confidence interval, e.g., 0.95 for 95% confidence interval
    pub level: Option<DecimalType>,
    /// Extension element for the 'level' primitive field. Contains metadata and extensions.
    pub _level: Option<Element>,
    /// Lower and upper bound values of the attribute estimate
    pub range: Option<Range>,
    /// A nested attribute estimate; which is the attribute estimate of an attribute estimate
    #[serde(rename = "attributeEstimate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<StringType>,
}
/// EvidenceStatistic nested structure for the 'modelCharacteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceStatisticModelcharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Model specification
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/statistic-model-code
    pub code: CodeableConcept,
    /// Numerical value to complete model specification
    pub value: Option<Quantity>,
    /// An attribute of the statistic used as a model characteristic
    #[serde(rename = "attributeEstimate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<StringType>,
}

impl Default for Evidence {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            cite_as_reference: Default::default(),
            cite_as_markdown: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            use_context: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            related_artifact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            assertion: Default::default(),
            _assertion: Default::default(),
            note: Default::default(),
            variable_definition: Vec::new(),
            synthesis_type: Default::default(),
            study_design: Default::default(),
            statistic: Default::default(),
            certainty: Default::default(),
        }
    }
}

impl Default for EvidenceStatisticSamplesize {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            number_of_studies: Default::default(),
            _number_of_studies: Default::default(),
            number_of_participants: Default::default(),
            _number_of_participants: Default::default(),
            known_data_count: Default::default(),
            _known_data_count: Default::default(),
        }
    }
}

impl Default for EvidenceStatistic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            attribute_estimate: Default::default(),
            model_characteristic: Default::default(),
            sample_size: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            statistic_type: Default::default(),
            category: Default::default(),
            quantity: Default::default(),
            number_of_events: Default::default(),
            _number_of_events: Default::default(),
            number_affected: Default::default(),
            _number_affected: Default::default(),
        }
    }
}

impl Default for EvidenceCertainty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            type_: Default::default(),
            rating: Default::default(),
            rater: Default::default(),
            _rater: Default::default(),
            subcomponent: Default::default(),
        }
    }
}

impl Default for EvidenceVariabledefinition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            variable_role: Default::default(),
            observed: Default::default(),
            intended: Default::default(),
            directness_match: Default::default(),
        }
    }
}

impl Default for EvidenceStatisticModelcharacteristicVariable {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            variable_definition: Default::default(),
            handling: Default::default(),
            _handling: Default::default(),
            value_category: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
        }
    }
}

impl Default for EvidenceStatisticAttributeestimate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            type_: Default::default(),
            quantity: Default::default(),
            level: Default::default(),
            _level: Default::default(),
            range: Default::default(),
            attribute_estimate: Default::default(),
        }
    }
}

impl Default for EvidenceStatisticModelcharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            value: Default::default(),
            attribute_estimate: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
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
                "Evidence.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Evidence.statistic.modelCharacteristic.variable.handling",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/variable-handling|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Evidence.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Evidence.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.contained", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.extension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.citeAs[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.contact", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.author", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.editor", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.endorser", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.assertion", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.note", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.variableDefinition", 1, None),
            rh_foundation::ElementCardinality::new("Evidence.variableDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Evidence.variableDefinition.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.variableRole",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.observed",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.intended",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.variableDefinition.directnessMatch",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Evidence.synthesisType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.studyDesign", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.statistic", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.statistic.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.extension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.statistic.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.statistic.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.note", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.statistic.statisticType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.numberOfEvents", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.numberAffected", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.sampleSize", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.statistic.sampleSize.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Evidence.statistic.sampleSize.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.numberOfStudies",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.numberOfParticipants",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.sampleSize.knownDataCount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Evidence.statistic.attributeEstimate", 0, None),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.note",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.level",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.range",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.attributeEstimate.attributeEstimate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.value",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.variableDefinition",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.handling",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.valueCategory",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.valueQuantity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.variable.valueRange",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Evidence.statistic.modelCharacteristic.attributeEstimate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Evidence.certainty", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.certainty.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.certainty.extension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.certainty.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.certainty.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.certainty.note", 0, None),
            rh_foundation::ElementCardinality::new("Evidence.certainty.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.certainty.rating", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.certainty.rater", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Evidence.certainty.subcomponent", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Evidence {
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

impl crate::traits::resource::ResourceMutators for Evidence {
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

impl crate::traits::resource::ResourceExistence for Evidence {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Evidence {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for Evidence {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Evidence {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::evidence::EvidenceAccessors for Evidence {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_slice()
    }
    fn author(&self) -> &[ContactDetail] {
        self.author.as_slice()
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_slice()
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_slice()
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_slice()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn assertion(&self) -> Option<StringType> {
        self.assertion.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn variable_definition(&self) -> &[EvidenceVariabledefinition] {
        &self.variable_definition
    }
    fn synthesis_type(&self) -> Option<CodeableConcept> {
        self.synthesis_type.clone()
    }
    fn study_design(&self) -> &[CodeableConcept] {
        self.study_design.as_slice()
    }
    fn statistic(&self) -> &[EvidenceStatistic] {
        self.statistic.as_slice()
    }
    fn certainty(&self) -> &[EvidenceCertainty] {
        self.certainty.as_slice()
    }
}

impl crate::traits::evidence::EvidenceMutators for Evidence {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_approval_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.approval_date = Some(value);
        resource
    }
    fn set_last_review_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_review_date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = value;
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = value;
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = value;
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = value;
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.push(item);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = value;
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource.related_artifact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_assertion(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.assertion = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = value;
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.push(item);
        resource
    }
    fn set_variable_definition(self, value: Vec<EvidenceVariabledefinition>) -> Self {
        let mut resource = self.clone();
        resource.variable_definition = value;
        resource
    }
    fn add_variable_definition(self, item: EvidenceVariabledefinition) -> Self {
        let mut resource = self.clone();
        resource.variable_definition.push(item);
        resource
    }
    fn set_synthesis_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.synthesis_type = Some(value);
        resource
    }
    fn set_study_design(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.study_design = value;
        resource
    }
    fn add_study_design(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.study_design.push(item);
        resource
    }
    fn set_statistic(self, value: Vec<EvidenceStatistic>) -> Self {
        let mut resource = self.clone();
        resource.statistic = value;
        resource
    }
    fn add_statistic(self, item: EvidenceStatistic) -> Self {
        let mut resource = self.clone();
        resource.statistic.push(item);
        resource
    }
    fn set_certainty(self, value: Vec<EvidenceCertainty>) -> Self {
        let mut resource = self.clone();
        resource.certainty = value;
        resource
    }
    fn add_certainty(self, item: EvidenceCertainty) -> Self {
        let mut resource = self.clone();
        resource.certainty.push(item);
        resource
    }
}

impl crate::traits::evidence::EvidenceExistence for Evidence {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_cite_as(&self) -> bool {
        self.cite_as_reference.is_some() || self.cite_as_markdown.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_editor(&self) -> bool {
        !self.editor.is_empty()
    }
    fn has_reviewer(&self) -> bool {
        !self.reviewer.is_empty()
    }
    fn has_endorser(&self) -> bool {
        !self.endorser.is_empty()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_assertion(&self) -> bool {
        self.assertion.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_variable_definition(&self) -> bool {
        !self.variable_definition.is_empty()
    }
    fn has_synthesis_type(&self) -> bool {
        self.synthesis_type.is_some()
    }
    fn has_study_design(&self) -> bool {
        !self.study_design.is_empty()
    }
    fn has_statistic(&self) -> bool {
        !self.statistic.is_empty()
    }
    fn has_certainty(&self) -> bool {
        !self.certainty.is_empty()
    }
}

impl crate::validation::ValidatableResource for Evidence {
    fn resource_type(&self) -> &'static str {
        "Evidence"
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
        Some("http://hl7.org/fhir/StructureDefinition/Evidence")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::evidence::{EvidenceAccessors, EvidenceExistence, EvidenceMutators};
