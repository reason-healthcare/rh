use crate::bindings::group_measure::GroupMeasure;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::research_element_type::ResearchElementType;
use crate::bindings::variable_type::VariableType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::timing::Timing;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ResearchElementDefinition
///
/// The ResearchElementDefinition resource describes a "PICO" element that knowledge (evidence, assertion, recommendation) is about.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ResearchElementDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ResearchElementDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchElementDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this research element definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the research element definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the research element definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this research element definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this research element definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<StringType>,
    /// Extension element for the 'shortTitle' primitive field. Contains metadata and extensions.
    #[serde(rename = "_shortTitle")]
    pub _short_title: Option<Element>,
    /// Subordinate title of the ResearchElementDefinition
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
    /// Natural language description of the research element definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Used for footnotes or explanatory notes
    pub comment: Option<Vec<StringType>>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for research element definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this research element definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Describes the clinical usage of the ResearchElementDefinition
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the research element definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the research element definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the research element definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// The category of the ResearchElementDefinition, such as Education, Treatment, Assessment, etc.
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
    /// Logic used by the ResearchElementDefinition
    pub library: Option<Vec<StringType>>,
    /// Extension element for the 'library' primitive field. Contains metadata and extensions.
    pub _library: Option<Element>,
    /// population | exposure | outcome
    #[serde(rename = "type")]
    pub type_: ResearchElementType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// dichotomous | continuous | descriptive
    #[serde(rename = "variableType")]
    pub variable_type: Option<VariableType>,
    /// Extension element for the 'variableType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_variableType")]
    pub _variable_type: Option<Element>,
    /// What defines the members of the research element
    pub characteristic: Vec<ResearchElementDefinitionCharacteristic>,
}
/// ResearchElementDefinition nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchElementDefinitionCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What code or expression defines members? (CodeableConcept)
    #[serde(rename = "definitionCodeableConcept")]
    pub definition_codeable_concept: CodeableConcept,
    /// What code or expression defines members? (canonical)
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: StringType,
    /// What code or expression defines members? (Expression)
    #[serde(rename = "definitionExpression")]
    pub definition_expression: Expression,
    /// What code or expression defines members? (DataRequirement)
    #[serde(rename = "definitionDataRequirement")]
    pub definition_data_requirement: DataRequirement,
    /// What code/value pairs define members?
    #[serde(rename = "usageContext")]
    pub usage_context: Option<Vec<UsageContext>>,
    /// Whether the characteristic includes or excludes members
    pub exclude: Option<BooleanType>,
    /// Extension element for the 'exclude' primitive field. Contains metadata and extensions.
    pub _exclude: Option<Element>,
    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,
    /// What time period does the study cover
    #[serde(rename = "studyEffectiveDescription")]
    pub study_effective_description: Option<StringType>,
    /// Extension element for the 'studyEffectiveDescription' primitive field. Contains metadata and extensions.
    #[serde(rename = "_studyEffectiveDescription")]
    pub _study_effective_description: Option<Element>,
    /// What time period does the study cover (dateTime)
    #[serde(rename = "studyEffectiveDateTime")]
    pub study_effective_date_time: Option<DateTimeType>,
    /// What time period does the study cover (Period)
    #[serde(rename = "studyEffectivePeriod")]
    pub study_effective_period: Option<Period>,
    /// What time period does the study cover (Duration)
    #[serde(rename = "studyEffectiveDuration")]
    pub study_effective_duration: Option<Duration>,
    /// What time period does the study cover (Timing)
    #[serde(rename = "studyEffectiveTiming")]
    pub study_effective_timing: Option<Timing>,
    /// Observation time from study start
    #[serde(rename = "studyEffectiveTimeFromStart")]
    pub study_effective_time_from_start: Option<Duration>,
    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "studyEffectiveGroupMeasure")]
    pub study_effective_group_measure: Option<GroupMeasure>,
    /// Extension element for the 'studyEffectiveGroupMeasure' primitive field. Contains metadata and extensions.
    #[serde(rename = "_studyEffectiveGroupMeasure")]
    pub _study_effective_group_measure: Option<Element>,
    /// What time period do participants cover
    #[serde(rename = "participantEffectiveDescription")]
    pub participant_effective_description: Option<StringType>,
    /// Extension element for the 'participantEffectiveDescription' primitive field. Contains metadata and extensions.
    #[serde(rename = "_participantEffectiveDescription")]
    pub _participant_effective_description: Option<Element>,
    /// What time period do participants cover (dateTime)
    #[serde(rename = "participantEffectiveDateTime")]
    pub participant_effective_date_time: Option<DateTimeType>,
    /// What time period do participants cover (Period)
    #[serde(rename = "participantEffectivePeriod")]
    pub participant_effective_period: Option<Period>,
    /// What time period do participants cover (Duration)
    #[serde(rename = "participantEffectiveDuration")]
    pub participant_effective_duration: Option<Duration>,
    /// What time period do participants cover (Timing)
    #[serde(rename = "participantEffectiveTiming")]
    pub participant_effective_timing: Option<Timing>,
    /// Observation time from study start
    #[serde(rename = "participantEffectiveTimeFromStart")]
    pub participant_effective_time_from_start: Option<Duration>,
    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "participantEffectiveGroupMeasure")]
    pub participant_effective_group_measure: Option<GroupMeasure>,
    /// Extension element for the 'participantEffectiveGroupMeasure' primitive field. Contains metadata and extensions.
    #[serde(rename = "_participantEffectiveGroupMeasure")]
    pub _participant_effective_group_measure: Option<Element>,
}

impl Default for ResearchElementDefinition {
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
            short_title: Default::default(),
            _short_title: Default::default(),
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
            comment: Default::default(),
            _comment: Default::default(),
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
            type_: Default::default(),
            _type: Default::default(),
            variable_type: Default::default(),
            _variable_type: Default::default(),
            characteristic: Vec::new(),
        }
    }
}

impl Default for ResearchElementDefinitionCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            definition_codeable_concept: Default::default(),
            definition_canonical: Default::default(),
            definition_expression: Default::default(),
            definition_data_requirement: Default::default(),
            usage_context: Default::default(),
            exclude: Default::default(),
            _exclude: Default::default(),
            unit_of_measure: Default::default(),
            study_effective_description: Default::default(),
            _study_effective_description: Default::default(),
            study_effective_date_time: Default::default(),
            study_effective_period: Default::default(),
            study_effective_duration: Default::default(),
            study_effective_timing: Default::default(),
            study_effective_time_from_start: Default::default(),
            study_effective_group_measure: Default::default(),
            _study_effective_group_measure: Default::default(),
            participant_effective_description: Default::default(),
            _participant_effective_description: Default::default(),
            participant_effective_date_time: Default::default(),
            participant_effective_period: Default::default(),
            participant_effective_duration: Default::default(),
            participant_effective_timing: Default::default(),
            participant_effective_time_from_start: Default::default(),
            participant_effective_group_measure: Default::default(),
            _participant_effective_group_measure: Default::default(),
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
    rh_foundation::Invariant::new("red-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ResearchElementDefinition.characteristic.participantEffectiveGroupMeasure", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/group-measure|4.0.1").with_description("Possible group measure aggregates (E.g. Mean, Median)."),
    rh_foundation::ElementBinding::new("ResearchElementDefinition.characteristic.studyEffectiveGroupMeasure", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/group-measure|4.0.1").with_description("Possible group measure aggregates (E.g. Mean, Median)."),
    rh_foundation::ElementBinding::new("ResearchElementDefinition.characteristic.unitOfMeasure", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/ucum-units|4.0.1").with_description("Unified Code for Units of Measure (UCUM)."),
    rh_foundation::ElementBinding::new("ResearchElementDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
    rh_foundation::ElementBinding::new("ResearchElementDefinition.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/research-element-type|4.0.1").with_description("The possible types of research elements (E.g. Population, Exposure, Outcome)."),
    rh_foundation::ElementBinding::new("ResearchElementDefinition.variableType", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/variable-type|4.0.1").with_description("The possible types of variables for exposures or outcomes (E.g. Dichotomous, Continuous, Descriptive)."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.shortTitle",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.subtitle",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.experimental",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.subject[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.publisher",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.comment", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.jurisdiction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.copyright",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.approvalDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.lastReviewDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.relatedArtifact",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new("ResearchElementDefinition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.variableType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.definition[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.usageContext",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.exclude",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.unitOfMeasure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.studyEffectiveDescription",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.studyEffective[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.studyEffectiveTimeFromStart",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.studyEffectiveGroupMeasure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.participantEffectiveDescription",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.participantEffective[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.participantEffectiveTimeFromStart",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ResearchElementDefinition.characteristic.participantEffectiveGroupMeasure",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ResearchElementDefinition {
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

impl crate::traits::resource::ResourceMutators for ResearchElementDefinition {
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

impl crate::traits::resource::ResourceExistence for ResearchElementDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ResearchElementDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ResearchElementDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ResearchElementDefinition {
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

impl crate::traits::research_element_definition::ResearchElementDefinitionAccessors
    for ResearchElementDefinition
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
    fn short_title(&self) -> Option<StringType> {
        self.short_title.clone()
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
    fn comment(&self) -> &[StringType] {
        self.comment.as_deref().unwrap_or(&[])
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
    fn type_(&self) -> ResearchElementType {
        self.type_.clone()
    }
    fn variable_type(&self) -> Option<VariableType> {
        self.variable_type.clone()
    }
    fn characteristic(&self) -> &[ResearchElementDefinitionCharacteristic] {
        &self.characteristic
    }
}

impl crate::traits::research_element_definition::ResearchElementDefinitionMutators
    for ResearchElementDefinition
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
    fn set_short_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.short_title = Some(value);
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
    fn set_comment(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn add_comment(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.comment.get_or_insert_with(Vec::new).push(item);
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
    fn set_type_(self, value: ResearchElementType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_variable_type(self, value: VariableType) -> Self {
        let mut resource = self.clone();
        resource.variable_type = Some(value);
        resource
    }
    fn set_characteristic(self, value: Vec<ResearchElementDefinitionCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = value;
        resource
    }
    fn add_characteristic(self, item: ResearchElementDefinitionCharacteristic) -> Self {
        let mut resource = self.clone();
        resource.characteristic.push(item);
        resource
    }
}

impl crate::traits::research_element_definition::ResearchElementDefinitionExistence
    for ResearchElementDefinition
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
    fn has_short_title(&self) -> bool {
        self.short_title.is_some()
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
    fn has_comment(&self) -> bool {
        self.comment.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_type_(&self) -> bool {
        true
    }
    fn has_variable_type(&self) -> bool {
        self.variable_type.is_some()
    }
    fn has_characteristic(&self) -> bool {
        !self.characteristic.is_empty()
    }
}

impl crate::validation::ValidatableResource for ResearchElementDefinition {
    fn resource_type(&self) -> &'static str {
        "ResearchElementDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ResearchElementDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::research_element_definition::{
    ResearchElementDefinitionAccessors, ResearchElementDefinitionExistence,
    ResearchElementDefinitionMutators,
};
