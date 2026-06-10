use crate::bindings::condition_precondition_type::ConditionPreconditionType;
use crate::bindings::condition_questionnaire_purpose::ConditionQuestionnairePurpose;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ConditionDefinition
///
/// A definition of a condition and information relevant to managing it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConditionDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this condition definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the condition definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the condition definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this condition definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this condition definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate title of the event definition
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
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the condition definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for condition definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Identification of the condition, problem or diagnosis
    ///
    /// Binding: example (Identification of the condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub code: CodeableConcept,
    /// Subjective severity of condition
    ///
    /// Binding: preferred (A subjective assessment of the severity of the condition as evaluated by the clinician.)
    ///
    /// Available values:
    /// - `24484000`
    /// - `6736007`
    /// - `255604002`
    pub severity: Option<CodeableConcept>,
    /// Anatomical location, if relevant
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,
    /// Stage/grade, usually assessed formally
    ///
    /// Binding: example (Codes describing condition stages (e.g. Cancer stages).)
    ///
    /// Available values:
    /// - `416780008`
    /// - `715345007`
    pub stage: Option<CodeableConcept>,
    /// Whether Severity is appropriate
    #[serde(rename = "hasSeverity")]
    pub has_severity: Option<BooleanType>,
    /// Extension element for the 'hasSeverity' primitive field. Contains metadata and extensions.
    #[serde(rename = "_hasSeverity")]
    pub _has_severity: Option<Element>,
    /// Whether bodySite is appropriate
    #[serde(rename = "hasBodySite")]
    pub has_body_site: Option<BooleanType>,
    /// Extension element for the 'hasBodySite' primitive field. Contains metadata and extensions.
    #[serde(rename = "_hasBodySite")]
    pub _has_body_site: Option<Element>,
    /// Whether stage is appropriate
    #[serde(rename = "hasStage")]
    pub has_stage: Option<BooleanType>,
    /// Extension element for the 'hasStage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_hasStage")]
    pub _has_stage: Option<Element>,
    /// Formal Definition for the condition
    pub definition: Option<Vec<StringType>>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Observations particularly relevant to this condition
    pub observation: Option<Vec<ConditionDefinitionObservation>>,
    /// Medications particularly relevant for this condition
    pub medication: Option<Vec<ConditionDefinitionMedication>>,
    /// Observation that suggets this condition
    pub precondition: Option<Vec<ConditionDefinitionPrecondition>>,
    /// Appropriate team for this condition
    pub team: Option<Vec<Reference>>,
    /// Questionnaire for this condition
    pub questionnaire: Option<Vec<ConditionDefinitionQuestionnaire>>,
    /// Plan that is appropriate
    pub plan: Option<Vec<ConditionDefinitionPlan>>,
}
/// ConditionDefinition nested structure for the 'precondition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinitionPrecondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// sensitive | specific
    #[serde(rename = "type")]
    pub type_: ConditionPreconditionType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Code for relevant Observation
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: CodeableConcept,
    /// Value of Observation (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// Value of Observation (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
}
/// ConditionDefinition nested structure for the 'observation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinitionObservation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category that is relevant
    ///
    /// Binding: preferred (Codes for high level observation categories.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-category
    pub category: Option<CodeableConcept>,
    /// Code for relevant Observation
    ///
    /// Binding: example (Codes identifying names of simple observations.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: Option<CodeableConcept>,
}
/// ConditionDefinition nested structure for the 'questionnaire' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinitionQuestionnaire {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// preadmit | diff-diagnosis | outcome
    pub purpose: ConditionQuestionnairePurpose,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Specific Questionnaire
    pub reference: Reference,
}
/// ConditionDefinition nested structure for the 'plan' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinitionPlan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Use for the plan
    pub role: Option<CodeableConcept>,
    /// The actual plan
    pub reference: Reference,
}
/// ConditionDefinition nested structure for the 'medication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinitionMedication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Category that is relevant
    ///
    /// Binding: example (A coded concept identifying the category of medication request.  For example, where the medication is to be consumed or administered, or the type of medication treatment.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/medicationrequest-category
    pub category: Option<CodeableConcept>,
    /// Code for relevant Medication
    ///
    /// Binding: example (A coded concept identifying substance or product that can be ordered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub code: Option<CodeableConcept>,
}

impl Default for ConditionDefinition {
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
            subtitle: Default::default(),
            _subtitle: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            code: CodeableConcept::default(),
            severity: Default::default(),
            body_site: Default::default(),
            stage: Default::default(),
            has_severity: Default::default(),
            _has_severity: Default::default(),
            has_body_site: Default::default(),
            _has_body_site: Default::default(),
            has_stage: Default::default(),
            _has_stage: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            observation: Default::default(),
            medication: Default::default(),
            precondition: Default::default(),
            team: Default::default(),
            questionnaire: Default::default(),
            plan: Default::default(),
        }
    }
}

impl Default for ConditionDefinitionPrecondition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            code: CodeableConcept::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
        }
    }
}

impl Default for ConditionDefinitionObservation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            code: Default::default(),
        }
    }
}

impl Default for ConditionDefinitionQuestionnaire {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            purpose: ConditionQuestionnairePurpose::default(),
            _purpose: Default::default(),
            reference: Reference::default(),
        }
    }
}

impl Default for ConditionDefinitionPlan {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            reference: Reference::default(),
        }
    }
}

impl Default for ConditionDefinitionMedication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            category: Default::default(),
            code: Default::default(),
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
                "ConditionDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ConditionDefinition.precondition.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/condition-precondition-type|5.0.0",
            )
            .with_description("Kind of precondition for the condition."),
            rh_foundation::ElementBinding::new(
                "ConditionDefinition.questionnaire.purpose",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/condition-questionnaire-purpose|5.0.0",
            )
            .with_description("The use of a questionnaire."),
            rh_foundation::ElementBinding::new(
                "ConditionDefinition.status",
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
            rh_foundation::ElementCardinality::new("ConditionDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.severity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.stage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.hasSeverity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.hasBodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.hasStage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.definition", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.observation", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.observation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.observation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.observation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.observation.category",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.observation.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.medication", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.medication.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.medication.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.medication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.medication.category",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.medication.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.precondition", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.precondition.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.team", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.questionnaire", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.questionnaire.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.questionnaire.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.questionnaire.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.questionnaire.purpose",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.questionnaire.reference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.plan", 0, None),
            rh_foundation::ElementCardinality::new("ConditionDefinition.plan.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConditionDefinition.plan.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.plan.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConditionDefinition.plan.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConditionDefinition.plan.reference",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ConditionDefinition {
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

impl crate::traits::resource::ResourceMutators for ConditionDefinition {
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

impl crate::traits::resource::ResourceExistence for ConditionDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ConditionDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ConditionDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ConditionDefinition {
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

impl crate::traits::condition_definition::ConditionDefinitionAccessors for ConditionDefinition {
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
    fn code(&self) -> CodeableConcept {
        self.code.clone()
    }
    fn severity(&self) -> Option<CodeableConcept> {
        self.severity.clone()
    }
    fn body_site(&self) -> Option<CodeableConcept> {
        self.body_site.clone()
    }
    fn stage(&self) -> Option<CodeableConcept> {
        self.stage.clone()
    }
    fn has_severity(&self) -> Option<BooleanType> {
        self.has_severity
    }
    fn has_body_site(&self) -> Option<BooleanType> {
        self.has_body_site
    }
    fn has_stage(&self) -> Option<BooleanType> {
        self.has_stage
    }
    fn definition(&self) -> &[StringType] {
        self.definition.as_deref().unwrap_or(&[])
    }
    fn observation(&self) -> &[ConditionDefinitionObservation] {
        self.observation.as_deref().unwrap_or(&[])
    }
    fn medication(&self) -> &[ConditionDefinitionMedication] {
        self.medication.as_deref().unwrap_or(&[])
    }
    fn precondition(&self) -> &[ConditionDefinitionPrecondition] {
        self.precondition.as_deref().unwrap_or(&[])
    }
    fn team(&self) -> &[Reference] {
        self.team.as_deref().unwrap_or(&[])
    }
    fn questionnaire(&self) -> &[ConditionDefinitionQuestionnaire] {
        self.questionnaire.as_deref().unwrap_or(&[])
    }
    fn plan(&self) -> &[ConditionDefinitionPlan] {
        self.plan.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::condition_definition::ConditionDefinitionMutators for ConditionDefinition {
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
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_severity(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.severity = Some(value);
        resource
    }
    fn set_body_site(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn set_stage(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.stage = Some(value);
        resource
    }
    fn set_has_severity(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.has_severity = Some(value);
        resource
    }
    fn set_has_body_site(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.has_body_site = Some(value);
        resource
    }
    fn set_has_stage(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.has_stage = Some(value);
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
    fn set_observation(self, value: Vec<ConditionDefinitionObservation>) -> Self {
        let mut resource = self.clone();
        resource.observation = Some(value);
        resource
    }
    fn add_observation(self, item: ConditionDefinitionObservation) -> Self {
        let mut resource = self.clone();
        resource.observation.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_medication(self, value: Vec<ConditionDefinitionMedication>) -> Self {
        let mut resource = self.clone();
        resource.medication = Some(value);
        resource
    }
    fn add_medication(self, item: ConditionDefinitionMedication) -> Self {
        let mut resource = self.clone();
        resource.medication.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_precondition(self, value: Vec<ConditionDefinitionPrecondition>) -> Self {
        let mut resource = self.clone();
        resource.precondition = Some(value);
        resource
    }
    fn add_precondition(self, item: ConditionDefinitionPrecondition) -> Self {
        let mut resource = self.clone();
        resource
            .precondition
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_team(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.team = Some(value);
        resource
    }
    fn add_team(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.team.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_questionnaire(self, value: Vec<ConditionDefinitionQuestionnaire>) -> Self {
        let mut resource = self.clone();
        resource.questionnaire = Some(value);
        resource
    }
    fn add_questionnaire(self, item: ConditionDefinitionQuestionnaire) -> Self {
        let mut resource = self.clone();
        resource
            .questionnaire
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_plan(self, value: Vec<ConditionDefinitionPlan>) -> Self {
        let mut resource = self.clone();
        resource.plan = Some(value);
        resource
    }
    fn add_plan(self, item: ConditionDefinitionPlan) -> Self {
        let mut resource = self.clone();
        resource.plan.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::condition_definition::ConditionDefinitionExistence for ConditionDefinition {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
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
    fn has_code(&self) -> bool {
        true
    }
    fn has_severity(&self) -> bool {
        self.severity.is_some()
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_stage(&self) -> bool {
        self.stage.is_some()
    }
    fn has_has_severity(&self) -> bool {
        self.has_severity.is_some()
    }
    fn has_has_body_site(&self) -> bool {
        self.has_body_site.is_some()
    }
    fn has_has_stage(&self) -> bool {
        self.has_stage.is_some()
    }
    fn has_definition(&self) -> bool {
        self.definition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_observation(&self) -> bool {
        self.observation.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_medication(&self) -> bool {
        self.medication.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_precondition(&self) -> bool {
        self.precondition.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_team(&self) -> bool {
        self.team.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_questionnaire(&self) -> bool {
        self.questionnaire.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_plan(&self) -> bool {
        self.plan.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ConditionDefinition {
    fn resource_type(&self) -> &'static str {
        "ConditionDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ConditionDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::condition_definition::{
    ConditionDefinitionAccessors, ConditionDefinitionExistence, ConditionDefinitionMutators,
};
