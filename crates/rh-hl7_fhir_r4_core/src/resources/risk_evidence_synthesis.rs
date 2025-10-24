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
/// RiskEvidenceSynthesis
///
/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RiskEvidenceSynthesis
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this risk evidence synthesis, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the risk evidence synthesis
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the risk evidence synthesis
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this risk evidence synthesis (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this risk evidence synthesis (human friendly)
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
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the risk evidence synthesis
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for risk evidence synthesis (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the risk evidence synthesis was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the risk evidence synthesis was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the risk evidence synthesis is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// The category of the EffectEvidenceSynthesis, such as Education, Treatment, Assessment, etc.
    ///
    /// Binding: example (High-level categorization of the definition, used for searching, sorting, and filtering.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
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
    pub exposure: Option<Reference>,
    /// What outcome?
    pub outcome: Reference,
    /// What sample size was involved?
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<RiskEvidenceSynthesisSamplesize>,
    /// What was the estimated risk
    #[serde(rename = "riskEstimate")]
    pub risk_estimate: Option<RiskEvidenceSynthesisRiskestimate>,
    /// How certain is the risk
    pub certainty: Option<Vec<RiskEvidenceSynthesisCertainty>>,
}
/// RiskEvidenceSynthesis nested structure for the 'riskEstimate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesisRiskestimate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How precise the estimate is
    #[serde(rename = "precisionEstimate")]
    pub precision_estimate: Option<Vec<RiskEvidenceSynthesisRiskestimatePrecisionestimate>>,
    /// Description of risk estimate
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Type of risk estimate
    ///
    /// Binding: extensible (Whether the risk estimate is dichotomous, continuous or qualitative and the specific type of risk estimate (eg proportion or median).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/risk-estimate-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Point estimate
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,
    /// Sample size for group measured
    #[serde(rename = "denominatorCount")]
    pub denominator_count: Option<IntegerType>,
    /// Extension element for the 'denominatorCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_denominatorCount")]
    pub _denominator_count: Option<Element>,
    /// Number with the outcome
    #[serde(rename = "numeratorCount")]
    pub numerator_count: Option<IntegerType>,
    /// Extension element for the 'numeratorCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numeratorCount")]
    pub _numerator_count: Option<Element>,
}
/// RiskEvidenceSynthesisRiskestimate nested structure for the 'precisionEstimate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesisRiskestimatePrecisionestimate {
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
/// RiskEvidenceSynthesis nested structure for the 'sampleSize' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesisSamplesize {
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
/// RiskEvidenceSynthesisCertainty nested structure for the 'certaintySubcomponent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesisCertaintyCertaintysubcomponent {
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
    pub rating: Option<Vec<CodeableConcept>>,
    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
}
/// RiskEvidenceSynthesis nested structure for the 'certainty' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvidenceSynthesisCertainty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A component that contributes to the overall certainty
    #[serde(rename = "certaintySubcomponent")]
    pub certainty_subcomponent: Option<Vec<RiskEvidenceSynthesisCertaintyCertaintysubcomponent>>,
    /// Certainty rating
    ///
    /// Binding: extensible (The quality of the evidence described. The code system used specifies the quality scale used to grade this evidence source while the code specifies the actual quality score (represented as a coded value) associated with the evidence.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/evidence-quality
    pub rating: Option<Vec<CodeableConcept>>,
    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
}

impl Default for RiskEvidenceSynthesis {
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
            exposure: Default::default(),
            outcome: Reference::default(),
            sample_size: Default::default(),
            risk_estimate: Default::default(),
            certainty: Default::default(),
        }
    }
}

impl Default for RiskEvidenceSynthesisRiskestimate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            precision_estimate: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            unit_of_measure: Default::default(),
            denominator_count: Default::default(),
            _denominator_count: Default::default(),
            numerator_count: Default::default(),
            _numerator_count: Default::default(),
        }
    }
}

impl Default for RiskEvidenceSynthesisRiskestimatePrecisionestimate {
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

impl Default for RiskEvidenceSynthesisSamplesize {
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

impl Default for RiskEvidenceSynthesisCertaintyCertaintysubcomponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            rating: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for RiskEvidenceSynthesisCertainty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            certainty_subcomponent: Default::default(),
            rating: Default::default(),
            note: Default::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("rvs-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RiskEvidenceSynthesis {
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

impl crate::traits::resource::ResourceMutators for RiskEvidenceSynthesis {
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

impl crate::traits::resource::ResourceExistence for RiskEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RiskEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceMutators for RiskEvidenceSynthesis {
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

impl crate::traits::domain_resource::DomainResourceExistence for RiskEvidenceSynthesis {
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

impl crate::traits::risk_evidence_synthesis::RiskEvidenceSynthesisAccessors
    for RiskEvidenceSynthesis
{
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
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
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
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
        self.topic.as_deref().unwrap_or(&[])
    }
    fn author(&self) -> &[ContactDetail] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_deref().unwrap_or(&[])
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_deref().unwrap_or(&[])
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_deref().unwrap_or(&[])
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_deref().unwrap_or(&[])
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
    fn exposure(&self) -> Option<Reference> {
        self.exposure.clone()
    }
    fn outcome(&self) -> Reference {
        self.outcome.clone()
    }
    fn sample_size(&self) -> Option<RiskEvidenceSynthesisSamplesize> {
        self.sample_size.clone()
    }
    fn risk_estimate(&self) -> Option<RiskEvidenceSynthesisRiskestimate> {
        self.risk_estimate.clone()
    }
    fn certainty(&self) -> &[RiskEvidenceSynthesisCertainty] {
        self.certainty.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::risk_evidence_synthesis::RiskEvidenceSynthesisMutators
    for RiskEvidenceSynthesis
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
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
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
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
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
        resource.topic = Some(value);
        resource
    }
    fn add_topic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.topic.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = Some(value);
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = Some(value);
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = Some(value);
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = Some(value);
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource
            .related_artifact
            .get_or_insert_with(Vec::new)
            .push(item);
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
        resource.exposure = Some(value);
        resource
    }
    fn set_outcome(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.outcome = value;
        resource
    }
    fn set_sample_size(self, value: RiskEvidenceSynthesisSamplesize) -> Self {
        let mut resource = self.clone();
        resource.sample_size = Some(value);
        resource
    }
    fn set_risk_estimate(self, value: RiskEvidenceSynthesisRiskestimate) -> Self {
        let mut resource = self.clone();
        resource.risk_estimate = Some(value);
        resource
    }
    fn set_certainty(self, value: Vec<RiskEvidenceSynthesisCertainty>) -> Self {
        let mut resource = self.clone();
        resource.certainty = Some(value);
        resource
    }
    fn add_certainty(self, item: RiskEvidenceSynthesisCertainty) -> Self {
        let mut resource = self.clone();
        resource.certainty.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::risk_evidence_synthesis::RiskEvidenceSynthesisExistence
    for RiskEvidenceSynthesis
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
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
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
        self.topic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_editor(&self) -> bool {
        self.editor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reviewer(&self) -> bool {
        self.reviewer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endorser(&self) -> bool {
        self.endorser.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_artifact(&self) -> bool {
        self.related_artifact
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
        self.exposure.is_some()
    }
    fn has_outcome(&self) -> bool {
        true
    }
    fn has_sample_size(&self) -> bool {
        self.sample_size.is_some()
    }
    fn has_risk_estimate(&self) -> bool {
        self.risk_estimate.is_some()
    }
    fn has_certainty(&self) -> bool {
        self.certainty.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for RiskEvidenceSynthesis {
    fn resource_type(&self) -> &'static str {
        "RiskEvidenceSynthesis"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis")
    }
}
