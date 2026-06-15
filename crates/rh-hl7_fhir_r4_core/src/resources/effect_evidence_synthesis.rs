use crate::bindings::exposure_state::ExposureState;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EffectEvidenceSynthesis
///
/// The EffectEvidenceSynthesis resource describes the difference in an outcome between exposures states in a population where the effect estimate is derived from a combination of research studies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EffectEvidenceSynthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: EffectEvidenceSynthesis
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this effect evidence synthesis, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the effect evidence synthesis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the effect evidence synthesis
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this effect evidence synthesis (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this effect evidence synthesis (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the effect evidence synthesis
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for effect evidence synthesis (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the effect evidence synthesis was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the effect evidence synthesis was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the effect evidence synthesis is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// The category of the EffectEvidenceSynthesis, such as Education, Treatment, Assessment, etc.
    ///
    /// Binding: example (High-level categorization of the definition, used for searching, sorting, and filtering.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<CodeableConcept>,
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
    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// Type of synthesis
    ///
    /// Binding: extensible (Types of combining results from a body of evidence (eg. summary data meta-analysis).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/synthesis-type
    #[serde(rename = "synthesisType")]
    pub synthesis_type: Option<CodeableConcept>,
    /// Type of study
    ///
    /// Binding: extensible (Types of research studies (types of research methods).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/study-type
    #[serde(rename = "studyType")]
    pub study_type: Option<CodeableConcept>,
    /// What population?
    pub population: Reference,
    /// What exposure?
    pub exposure: Reference,
    /// What comparison exposure?
    #[serde(rename = "exposureAlternative")]
    pub exposure_alternative: Reference,
    /// What outcome?
    pub outcome: Reference,
    /// What sample size was involved?
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<EffectEvidenceSynthesisSamplesize>,
    /// What was the result per exposure?
    #[serde(rename = "resultsByExposure")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results_by_exposure: Vec<EffectEvidenceSynthesisResultsbyexposure>,
    /// What was the estimated effect
    #[serde(rename = "effectEstimate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effect_estimate: Vec<EffectEvidenceSynthesisEffectestimate>,
    /// How certain is the effect
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<EffectEvidenceSynthesisCertainty>,
}
/// EffectEvidenceSynthesisEffectestimate nested structure for the 'precisionEstimate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisEffectestimatePrecisionestimate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of precision estimate
    ///
    /// Binding: extensible (Method of reporting variability of estimates, such as confidence intervals, interquartile range or standard deviation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/precision-estimate-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Level of confidence interval
    pub level: Option<DecimalType>,
    /// Extension element for the 'level' primitive field. Contains metadata and extensions.
    pub _level: Option<Element>,
    /// Lower bound
    pub from: Option<DecimalType>,
    /// Extension element for the 'from' primitive field. Contains metadata and extensions.
    pub _from: Option<Element>,
    /// Upper bound
    pub to: Option<DecimalType>,
    /// Extension element for the 'to' primitive field. Contains metadata and extensions.
    pub _to: Option<Element>,
}
/// EffectEvidenceSynthesis nested structure for the 'resultsByExposure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisResultsbyexposure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of results by exposure
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// exposure | exposure-alternative
    #[serde(rename = "exposureState")]
    pub exposure_state: Option<ExposureState>,
    /// Extension element for the 'exposureState' primitive field. Contains metadata and extensions.
    #[serde(rename = "_exposureState")]
    pub _exposure_state: Option<Element>,
    /// Variant exposure states
    ///
    /// Binding: extensible (Used for results by exposure in variant states such as low-risk, medium-risk and high-risk states.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-variant-state
    #[serde(rename = "variantState")]
    pub variant_state: Option<CodeableConcept>,
    /// Risk evidence synthesis
    #[serde(rename = "riskEvidenceSynthesis")]
    pub risk_evidence_synthesis: Reference,
}
/// EffectEvidenceSynthesisCertainty nested structure for the 'certaintySubcomponent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisCertaintyCertaintysubcomponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of subcomponent of certainty rating
    ///
    /// Binding: extensible (The subcomponent classification of quality of evidence rating systems.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-subcomponent-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Subcomponent certainty rating
    ///
    /// Binding: extensible (The quality rating of the subcomponent of a quality of evidence rating.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/certainty-subcomponent-rating
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rating: Vec<CodeableConcept>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// EffectEvidenceSynthesis nested structure for the 'effectEstimate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisEffectestimate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How precise the estimate is
    #[serde(rename = "precisionEstimate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precision_estimate: Vec<EffectEvidenceSynthesisEffectestimatePrecisionestimate>,
    /// Description of effect estimate
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Type of efffect estimate
    ///
    /// Binding: extensible (Whether the effect estimate is an absolute effect estimate (absolute difference) or a relative effect estimate (relative difference), and the specific type of effect estimate (eg relative risk or median difference).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/effect-estimate-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Variant exposure states
    ///
    /// Binding: extensible (Used for results by exposure in variant states such as low-risk, medium-risk and high-risk states.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-variant-state
    #[serde(rename = "variantState")]
    pub variant_state: Option<CodeableConcept>,
    /// Point estimate
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,
}
/// EffectEvidenceSynthesis nested structure for the 'certainty' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisCertainty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A component that contributes to the overall certainty
    #[serde(rename = "certaintySubcomponent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty_subcomponent: Vec<EffectEvidenceSynthesisCertaintyCertaintysubcomponent>,
    /// Certainty rating
    ///
    /// Binding: extensible (The quality of the evidence described. The code system used specifies the quality scale used to grade this evidence source while the code specifies the actual quality score (represented as a coded value) associated with the evidence.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-quality
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rating: Vec<CodeableConcept>,
    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// EffectEvidenceSynthesis nested structure for the 'sampleSize' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEvidenceSynthesisSamplesize {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of sample size
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// How many studies?
    #[serde(rename = "numberOfStudies")]
    pub number_of_studies: Option<IntegerType>,
    /// Extension element for the 'numberOfStudies' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfStudies")]
    pub _number_of_studies: Option<Element>,
    /// How many participants?
    #[serde(rename = "numberOfParticipants")]
    pub number_of_participants: Option<IntegerType>,
    /// Extension element for the 'numberOfParticipants' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfParticipants")]
    pub _number_of_participants: Option<Element>,
}

impl Default for EffectEvidenceSynthesis {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            note: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            effective_period: Default::default(),
            topic: Default::default(),
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            related_artifact: Default::default(),
            synthesis_type: Default::default(),
            study_type: Default::default(),
            population: Reference::default(),
            exposure: Reference::default(),
            exposure_alternative: Reference::default(),
            outcome: Reference::default(),
            sample_size: Default::default(),
            results_by_exposure: Default::default(),
            effect_estimate: Default::default(),
            certainty: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisEffectestimatePrecisionestimate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            level: Default::default(),
            _level: Default::default(),
            from: Default::default(),
            _from: Default::default(),
            to: Default::default(),
            _to: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisResultsbyexposure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            exposure_state: Default::default(),
            _exposure_state: Default::default(),
            variant_state: Default::default(),
            risk_evidence_synthesis: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisCertaintyCertaintysubcomponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            rating: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisEffectestimate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            precision_estimate: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            variant_state: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            unit_of_measure: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisCertainty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            certainty_subcomponent: Default::default(),
            rating: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for EffectEvidenceSynthesisSamplesize {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            number_of_studies: Default::default(),
            _number_of_studies: Default::default(),
            number_of_participants: Default::default(),
            _number_of_participants: Default::default(),
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ees-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("EffectEvidenceSynthesis.effectEstimate.unitOfMeasure", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/ucum-units|4.0.1").with_description("Unified Code for Units of Measure (UCUM)."),
    rh_foundation::ElementBinding::new("EffectEvidenceSynthesis.resultsByExposure.exposureState", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/exposure-state|4.0.1").with_description("Whether the results by exposure is describing the results for the primary exposure of interest (exposure) or the alternative state (exposureAlternative)."),
    rh_foundation::ElementBinding::new("EffectEvidenceSynthesis.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.contained", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.contact", 0, None),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.note", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.useContext", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.approvalDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.lastReviewDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.topic", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.author", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.editor", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.endorser", 0, None),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.relatedArtifact",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.synthesisType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.studyType", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.population",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.exposure", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.exposureAlternative",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.outcome", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.numberOfStudies",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.sampleSize.numberOfParticipants",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.exposureState",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.variantState",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.resultsByExposure.riskEvidenceSynthesis",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.variantState",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.value",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.unitOfMeasure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.level",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.from",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.effectEstimate.precisionEstimate.to",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EffectEvidenceSynthesis.certainty", 0, None),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.rating",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.note",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.rating",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EffectEvidenceSynthesis.certainty.certaintySubcomponent.note",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EffectEvidenceSynthesis {
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

impl crate::traits::resource::ResourceMutators for EffectEvidenceSynthesis {
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

impl crate::traits::resource::ResourceExistence for EffectEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EffectEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceMutators for EffectEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceExistence for EffectEvidenceSynthesis {
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

impl crate::traits::effect_evidence_synthesis::EffectEvidenceSynthesisAccessors
    for EffectEvidenceSynthesis
{
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
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn effective_period(&self) -> Option<Period> {
        self.effective_period.clone()
    }
    fn topic(&self) -> &[CodeableConcept] {
        self.topic.as_slice()
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
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn synthesis_type(&self) -> Option<CodeableConcept> {
        self.synthesis_type.clone()
    }
    fn study_type(&self) -> Option<CodeableConcept> {
        self.study_type.clone()
    }
    fn population(&self) -> Reference {
        self.population.clone()
    }
    fn exposure(&self) -> Reference {
        self.exposure.clone()
    }
    fn exposure_alternative(&self) -> Reference {
        self.exposure_alternative.clone()
    }
    fn outcome(&self) -> Reference {
        self.outcome.clone()
    }
    fn sample_size(&self) -> Option<EffectEvidenceSynthesisSamplesize> {
        self.sample_size.clone()
    }
    fn results_by_exposure(&self) -> &[EffectEvidenceSynthesisResultsbyexposure] {
        self.results_by_exposure.as_slice()
    }
    fn effect_estimate(&self) -> &[EffectEvidenceSynthesisEffectestimate] {
        self.effect_estimate.as_slice()
    }
    fn certainty(&self) -> &[EffectEvidenceSynthesisCertainty] {
        self.certainty.as_slice()
    }
}

impl crate::traits::effect_evidence_synthesis::EffectEvidenceSynthesisMutators
    for EffectEvidenceSynthesis
{
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
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
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
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
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
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
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
    fn set_effective_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.effective_period = Some(value);
        resource
    }
    fn set_topic(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.topic = value;
        resource
    }
    fn add_topic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.topic.push(item);
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
    fn set_synthesis_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.synthesis_type = Some(value);
        resource
    }
    fn set_study_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.study_type = Some(value);
        resource
    }
    fn set_population(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.population = value;
        resource
    }
    fn set_exposure(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.exposure = value;
        resource
    }
    fn set_exposure_alternative(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.exposure_alternative = value;
        resource
    }
    fn set_outcome(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.outcome = value;
        resource
    }
    fn set_sample_size(self, value: EffectEvidenceSynthesisSamplesize) -> Self {
        let mut resource = self.clone();
        resource.sample_size = Some(value);
        resource
    }
    fn set_results_by_exposure(self, value: Vec<EffectEvidenceSynthesisResultsbyexposure>) -> Self {
        let mut resource = self.clone();
        resource.results_by_exposure = value;
        resource
    }
    fn add_results_by_exposure(self, item: EffectEvidenceSynthesisResultsbyexposure) -> Self {
        let mut resource = self.clone();
        resource.results_by_exposure.push(item);
        resource
    }
    fn set_effect_estimate(self, value: Vec<EffectEvidenceSynthesisEffectestimate>) -> Self {
        let mut resource = self.clone();
        resource.effect_estimate = value;
        resource
    }
    fn add_effect_estimate(self, item: EffectEvidenceSynthesisEffectestimate) -> Self {
        let mut resource = self.clone();
        resource.effect_estimate.push(item);
        resource
    }
    fn set_certainty(self, value: Vec<EffectEvidenceSynthesisCertainty>) -> Self {
        let mut resource = self.clone();
        resource.certainty = value;
        resource
    }
    fn add_certainty(self, item: EffectEvidenceSynthesisCertainty) -> Self {
        let mut resource = self.clone();
        resource.certainty.push(item);
        resource
    }
}

impl crate::traits::effect_evidence_synthesis::EffectEvidenceSynthesisExistence
    for EffectEvidenceSynthesis
{
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
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_effective_period(&self) -> bool {
        self.effective_period.is_some()
    }
    fn has_topic(&self) -> bool {
        !self.topic.is_empty()
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
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_synthesis_type(&self) -> bool {
        self.synthesis_type.is_some()
    }
    fn has_study_type(&self) -> bool {
        self.study_type.is_some()
    }
    fn has_population(&self) -> bool {
        true
    }
    fn has_exposure(&self) -> bool {
        true
    }
    fn has_exposure_alternative(&self) -> bool {
        true
    }
    fn has_outcome(&self) -> bool {
        true
    }
    fn has_sample_size(&self) -> bool {
        self.sample_size.is_some()
    }
    fn has_results_by_exposure(&self) -> bool {
        !self.results_by_exposure.is_empty()
    }
    fn has_effect_estimate(&self) -> bool {
        !self.effect_estimate.is_empty()
    }
    fn has_certainty(&self) -> bool {
        !self.certainty.is_empty()
    }
}

impl crate::validation::ValidatableResource for EffectEvidenceSynthesis {
    fn resource_type(&self) -> &'static str {
        "EffectEvidenceSynthesis"
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
        Some("http://hl7.org/fhir/StructureDefinition/EffectEvidenceSynthesis")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::effect_evidence_synthesis::{
    EffectEvidenceSynthesisAccessors, EffectEvidenceSynthesisExistence,
    EffectEvidenceSynthesisMutators,
};
