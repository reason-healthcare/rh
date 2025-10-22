use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Measure
///
/// The Measure resource provides the definition of a quality measure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Measure
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Measure
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this measure, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the measure
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the measure
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this measure (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this measure (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate title of the measure
    pub subtitle: Option<StringType>,
    /// Extension element for the 'subtitle' primitive field. Contains metadata and extensions.
    pub _subtitle: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
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
    /// Natural language description of the measure
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for measure (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this measure is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Describes the clinical usage of the measure
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the measure was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the measure was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the measure is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// The category of the measure, such as Education, Treatment, Assessment, etc.
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
    /// Logic used by the measure
    pub library: Option<Vec<StringType>>,
    /// Extension element for the 'library' primitive field. Contains metadata and extensions.
    pub _library: Option<Element>,
    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<StringType>,
    /// Extension element for the 'disclaimer' primitive field. Contains metadata and extensions.
    pub _disclaimer: Option<Element>,
    /// proportion | ratio | continuous-variable | cohort
    ///
    /// Binding: extensible (The scoring type of the measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-scoring
    pub scoring: Option<CodeableConcept>,
    /// opportunity | all-or-nothing | linear | weighted
    ///
    /// Binding: extensible (The composite scoring method of the measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/composite-measure-scoring
    #[serde(rename = "compositeScoring")]
    pub composite_scoring: Option<CodeableConcept>,
    /// process | outcome | structure | patient-reported-outcome | composite
    ///
    /// Binding: extensible (The type of measure (includes codes from 2.16.840.1.113883.1.11.20368).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// How risk adjustment is applied for this measure
    #[serde(rename = "riskAdjustment")]
    pub risk_adjustment: Option<StringType>,
    /// Extension element for the 'riskAdjustment' primitive field. Contains metadata and extensions.
    #[serde(rename = "_riskAdjustment")]
    pub _risk_adjustment: Option<Element>,
    /// How is rate aggregation performed for this measure
    #[serde(rename = "rateAggregation")]
    pub rate_aggregation: Option<StringType>,
    /// Extension element for the 'rateAggregation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_rateAggregation")]
    pub _rate_aggregation: Option<Element>,
    /// Detailed description of why the measure exists
    pub rationale: Option<StringType>,
    /// Extension element for the 'rationale' primitive field. Contains metadata and extensions.
    pub _rationale: Option<Element>,
    /// Summary of clinical guidelines
    #[serde(rename = "clinicalRecommendationStatement")]
    pub clinical_recommendation_statement: Option<StringType>,
    /// Extension element for the 'clinicalRecommendationStatement' primitive field. Contains metadata and extensions.
    #[serde(rename = "_clinicalRecommendationStatement")]
    pub _clinical_recommendation_statement: Option<Element>,
    /// increase | decrease
    #[serde(rename = "improvementNotation")]
    pub improvement_notation: Option<CodeableConcept>,
    /// Defined terms used in the measure documentation
    pub definition: Option<Vec<StringType>>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Additional guidance for implementers
    pub guidance: Option<StringType>,
    /// Extension element for the 'guidance' primitive field. Contains metadata and extensions.
    pub _guidance: Option<Element>,
    /// Population criteria group
    pub group: Option<Vec<MeasureGroup>>,
    /// What other data should be reported with the measure
    #[serde(rename = "supplementalData")]
    pub supplemental_data: Option<Vec<MeasureSupplementaldata>>,
}
/// MeasureGroupStratifier nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureGroupStratifierComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Meaning of the stratifier component
    pub code: Option<CodeableConcept>,
    /// The human readable description of this stratifier component
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Component of how the measure should be stratified
    pub criteria: Expression,
}
/// MeasureGroup nested structure for the 'population' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureGroupPopulation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    ///
    /// Binding: extensible (The type of population.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-population
    pub code: Option<CodeableConcept>,
    /// The human readable description of this population criteria
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The criteria that defines this population
    pub criteria: Expression,
}
/// Measure nested structure for the 'supplementalData' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureSupplementaldata {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Meaning of the supplemental data
    pub code: Option<CodeableConcept>,
    /// supplemental-data | risk-adjustment-factor
    ///
    /// Binding: extensible (The intended usage for supplemental data elements in the measure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measure-data-usage
    pub usage: Option<Vec<CodeableConcept>>,
    /// The human readable description of this supplemental data
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Expression describing additional data to be reported
    pub criteria: Expression,
}
/// Measure nested structure for the 'group' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Stratifier criteria for the measure
    pub stratifier: Option<Vec<MeasureGroupStratifier>>,
    /// Population criteria
    pub population: Option<Vec<MeasureGroupPopulation>>,
    /// Meaning of the group
    pub code: Option<CodeableConcept>,
    /// Summary description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// MeasureGroup nested structure for the 'stratifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureGroupStratifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Meaning of the stratifier
    pub code: Option<CodeableConcept>,
    /// The human readable description of this stratifier
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// How the measure should be stratified
    pub criteria: Option<Expression>,
}

impl Default for Measure {
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
            subtitle: Default::default(),
            _subtitle: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            usage: Default::default(),
            _usage: Default::default(),
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
            library: Default::default(),
            _library: Default::default(),
            disclaimer: Default::default(),
            _disclaimer: Default::default(),
            scoring: Default::default(),
            composite_scoring: Default::default(),
            type_: Default::default(),
            risk_adjustment: Default::default(),
            _risk_adjustment: Default::default(),
            rate_aggregation: Default::default(),
            _rate_aggregation: Default::default(),
            rationale: Default::default(),
            _rationale: Default::default(),
            clinical_recommendation_statement: Default::default(),
            _clinical_recommendation_statement: Default::default(),
            improvement_notation: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            guidance: Default::default(),
            _guidance: Default::default(),
            group: Default::default(),
            supplemental_data: Default::default(),
        }
    }
}

impl Default for MeasureGroupStratifierComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            criteria: Default::default(),
        }
    }
}

impl Default for MeasureGroupPopulation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            criteria: Default::default(),
        }
    }
}

impl Default for MeasureSupplementaldata {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            usage: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            criteria: Default::default(),
        }
    }
}

impl Default for MeasureGroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            stratifier: Default::default(),
            population: Default::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for MeasureGroupStratifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            criteria: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Measure {
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

impl crate::traits::resource::ResourceMutators for Measure {
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

impl crate::traits::resource::ResourceExistence for Measure {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Measure {
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

impl crate::traits::domain_resource::DomainResourceMutators for Measure {
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

impl crate::traits::domain_resource::DomainResourceExistence for Measure {
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

impl crate::traits::measure::MeasureAccessors for Measure {
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
    fn subtitle(&self) -> Option<StringType> {
        self.subtitle.clone()
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
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn usage(&self) -> Option<StringType> {
        self.usage.clone()
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
    fn library(&self) -> &[StringType] {
        self.library.as_deref().unwrap_or(&[])
    }
    fn disclaimer(&self) -> Option<StringType> {
        self.disclaimer.clone()
    }
    fn scoring(&self) -> Option<CodeableConcept> {
        self.scoring.clone()
    }
    fn composite_scoring(&self) -> Option<CodeableConcept> {
        self.composite_scoring.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn risk_adjustment(&self) -> Option<StringType> {
        self.risk_adjustment.clone()
    }
    fn rate_aggregation(&self) -> Option<StringType> {
        self.rate_aggregation.clone()
    }
    fn rationale(&self) -> Option<StringType> {
        self.rationale.clone()
    }
    fn clinical_recommendation_statement(&self) -> Option<StringType> {
        self.clinical_recommendation_statement.clone()
    }
    fn improvement_notation(&self) -> Option<CodeableConcept> {
        self.improvement_notation.clone()
    }
    fn definition(&self) -> &[StringType] {
        self.definition.as_deref().unwrap_or(&[])
    }
    fn guidance(&self) -> Option<StringType> {
        self.guidance.clone()
    }
    fn group(&self) -> &[MeasureGroup] {
        self.group.as_deref().unwrap_or(&[])
    }
    fn supplemental_data(&self) -> &[MeasureSupplementaldata] {
        self.supplemental_data.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::measure::MeasureMutators for Measure {
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
    fn set_subtitle(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.subtitle = Some(value);
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
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_usage(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage = Some(value);
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
    fn set_library(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.library = Some(value);
        resource
    }
    fn add_library(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.library.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_disclaimer(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.disclaimer = Some(value);
        resource
    }
    fn set_scoring(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.scoring = Some(value);
        resource
    }
    fn set_composite_scoring(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.composite_scoring = Some(value);
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
    fn set_risk_adjustment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.risk_adjustment = Some(value);
        resource
    }
    fn set_rate_aggregation(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.rate_aggregation = Some(value);
        resource
    }
    fn set_rationale(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.rationale = Some(value);
        resource
    }
    fn set_clinical_recommendation_statement(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.clinical_recommendation_statement = Some(value);
        resource
    }
    fn set_improvement_notation(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.improvement_notation = Some(value);
        resource
    }
    fn set_definition(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
    fn add_definition(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.definition.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_guidance(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.guidance = Some(value);
        resource
    }
    fn set_group(self, value: Vec<MeasureGroup>) -> Self {
        let mut resource = self.clone();
        resource.group = Some(value);
        resource
    }
    fn add_group(self, item: MeasureGroup) -> Self {
        let mut resource = self.clone();
        resource.group.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_supplemental_data(self, value: Vec<MeasureSupplementaldata>) -> Self {
        let mut resource = self.clone();
        resource.supplemental_data = Some(value);
        resource
    }
    fn add_supplemental_data(self, item: MeasureSupplementaldata) -> Self {
        let mut resource = self.clone();
        resource
            .supplemental_data
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::measure::MeasureExistence for Measure {
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
    fn has_subject(&self) -> bool {
        self.subject_codeable_concept.is_some() || self.subject_reference.is_some()
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
    fn has_subtitle(&self) -> bool {
        self.subtitle.is_some()
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
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_usage(&self) -> bool {
        self.usage.is_some()
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
    fn has_library(&self) -> bool {
        self.library.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_disclaimer(&self) -> bool {
        self.disclaimer.is_some()
    }
    fn has_scoring(&self) -> bool {
        self.scoring.is_some()
    }
    fn has_composite_scoring(&self) -> bool {
        self.composite_scoring.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_risk_adjustment(&self) -> bool {
        self.risk_adjustment.is_some()
    }
    fn has_rate_aggregation(&self) -> bool {
        self.rate_aggregation.is_some()
    }
    fn has_rationale(&self) -> bool {
        self.rationale.is_some()
    }
    fn has_clinical_recommendation_statement(&self) -> bool {
        self.clinical_recommendation_statement.is_some()
    }
    fn has_improvement_notation(&self) -> bool {
        self.improvement_notation.is_some()
    }
    fn has_definition(&self) -> bool {
        self.definition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_guidance(&self) -> bool {
        self.guidance.is_some()
    }
    fn has_group(&self) -> bool {
        self.group.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supplemental_data(&self) -> bool {
        self.supplemental_data
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}
