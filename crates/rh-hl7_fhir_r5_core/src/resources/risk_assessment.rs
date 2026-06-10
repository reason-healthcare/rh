use crate::bindings::observation_status::ObservationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// RiskAssessment
///
/// An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RiskAssessment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier for the assessment
    pub identifier: Option<Vec<Identifier>>,
    /// Request fulfilled by this assessment
    #[serde(rename = "basedOn")]
    pub based_on: Option<Reference>,
    /// Part of this occurrence
    pub parent: Option<Reference>,
    /// registered | preliminary | final | amended +
    pub status: ObservationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Evaluation mechanism
    ///
    /// Binding: example (The mechanism or algorithm used to make the assessment; e.g. TIMI, PRISM, Cardiff Type 2 diabetes, etc.)
    pub method: Option<CodeableConcept>,
    /// Type of assessment
    pub code: Option<CodeableConcept>,
    /// Who/what does assessment apply to?
    pub subject: Reference,
    /// Where was assessment performed?
    pub encounter: Option<Reference>,
    /// When was assessment made? (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When was assessment made? (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// Condition assessed
    pub condition: Option<Reference>,
    /// Who did assessment?
    pub performer: Option<Reference>,
    /// Why the assessment was necessary?
    pub reason: Option<Vec<CodeableReference>>,
    /// Information used in assessment
    pub basis: Option<Vec<Reference>>,
    /// Outcome predicted
    pub prediction: Option<Vec<RiskAssessmentPrediction>>,
    /// How to reduce risk
    pub mitigation: Option<StringType>,
    /// Extension element for the 'mitigation' primitive field. Contains metadata and extensions.
    pub _mitigation: Option<Element>,
    /// Comments on the risk assessment
    pub note: Option<Vec<Annotation>>,
}
/// RiskAssessment nested structure for the 'prediction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentPrediction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Possible outcome for the subject
    ///
    /// Binding: example (The condition or other outcome; e.g. death, remission, amputation, infection, etc.)
    pub outcome: Option<CodeableConcept>,
    /// Likelihood of specified outcome (decimal)
    #[serde(rename = "probabilityDecimal")]
    pub probability_decimal: Option<DecimalType>,
    /// Likelihood of specified outcome (Range)
    #[serde(rename = "probabilityRange")]
    pub probability_range: Option<Range>,
    /// Likelihood of specified outcome as a qualitative value
    ///
    /// Binding: example (The likelihood of the occurrence of a specified outcome.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/risk-probability
    #[serde(rename = "qualitativeRisk")]
    pub qualitative_risk: Option<CodeableConcept>,
    /// Relative likelihood
    #[serde(rename = "relativeRisk")]
    pub relative_risk: Option<DecimalType>,
    /// Extension element for the 'relativeRisk' primitive field. Contains metadata and extensions.
    #[serde(rename = "_relativeRisk")]
    pub _relative_risk: Option<Element>,
    /// Timeframe or age range (Period)
    #[serde(rename = "whenPeriod")]
    pub when_period: Option<Period>,
    /// Timeframe or age range (Range)
    #[serde(rename = "whenRange")]
    pub when_range: Option<Range>,
    /// Explanation of prediction
    pub rationale: Option<StringType>,
    /// Extension element for the 'rationale' primitive field. Contains metadata and extensions.
    pub _rationale: Option<Element>,
}

impl Default for RiskAssessment {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            parent: Default::default(),
            status: ObservationStatus::default(),
            _status: Default::default(),
            method: Default::default(),
            code: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            condition: Default::default(),
            performer: Default::default(),
            reason: Default::default(),
            basis: Default::default(),
            prediction: Default::default(),
            mitigation: Default::default(),
            _mitigation: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for RiskAssessmentPrediction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            outcome: Default::default(),
            probability_decimal: Default::default(),
            probability_range: Default::default(),
            qualitative_risk: Default::default(),
            relative_risk: Default::default(),
            _relative_risk: Default::default(),
            when_period: Default::default(),
            when_range: Default::default(),
            rationale: Default::default(),
            _rationale: Default::default(),
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
    rh_foundation::Invariant::new("ras-1", rh_foundation::Severity::Error, "low and high must be percentages, if present", "(low.empty() or ((low.code = '%') and (low.system = %ucum))) and (high.empty() or ((high.code = '%') and (high.system = %ucum)))"),
    rh_foundation::Invariant::new("ras-2", rh_foundation::Severity::Error, "Probability as a deciml must be <= 100", "probability.empty() or ((probability is decimal) implies ((probability as decimal) <= 100))"),
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
                "RiskAssessment.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "RiskAssessment.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/observation-status|5.0.0",
            )
            .with_description(
                "The status of the risk assessment; e.g. preliminary, final, amended, etc.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RiskAssessment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.contained", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.extension", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.identifier", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.basedOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.parent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.method", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.condition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.performer", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.reason", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.basis", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.prediction", 0, None),
            rh_foundation::ElementCardinality::new("RiskAssessment.prediction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.prediction.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "RiskAssessment.prediction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RiskAssessment.prediction.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RiskAssessment.prediction.probability[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RiskAssessment.prediction.qualitativeRisk",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RiskAssessment.prediction.relativeRisk",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RiskAssessment.prediction.when[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RiskAssessment.prediction.rationale",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RiskAssessment.mitigation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RiskAssessment.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RiskAssessment {
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

impl crate::traits::resource::ResourceMutators for RiskAssessment {
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

impl crate::traits::resource::ResourceExistence for RiskAssessment {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RiskAssessment {
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

impl crate::traits::domain_resource::DomainResourceMutators for RiskAssessment {
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

impl crate::traits::domain_resource::DomainResourceExistence for RiskAssessment {
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

impl crate::traits::risk_assessment::RiskAssessmentAccessors for RiskAssessment {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> Option<Reference> {
        self.based_on.clone()
    }
    fn parent(&self) -> Option<Reference> {
        self.parent.clone()
    }
    fn status(&self) -> ObservationStatus {
        self.status.clone()
    }
    fn method(&self) -> Option<CodeableConcept> {
        self.method.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn condition(&self) -> Option<Reference> {
        self.condition.clone()
    }
    fn performer(&self) -> Option<Reference> {
        self.performer.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn basis(&self) -> &[Reference] {
        self.basis.as_deref().unwrap_or(&[])
    }
    fn prediction(&self) -> &[RiskAssessmentPrediction] {
        self.prediction.as_deref().unwrap_or(&[])
    }
    fn mitigation(&self) -> Option<StringType> {
        self.mitigation.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::risk_assessment::RiskAssessmentMutators for RiskAssessment {
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
    fn set_based_on(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn set_parent(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
    fn set_status(self, value: ObservationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_method(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.method = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
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
    fn set_condition(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.condition = Some(value);
        resource
    }
    fn set_performer(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
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
    fn set_basis(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.basis = Some(value);
        resource
    }
    fn add_basis(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.basis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_prediction(self, value: Vec<RiskAssessmentPrediction>) -> Self {
        let mut resource = self.clone();
        resource.prediction = Some(value);
        resource
    }
    fn add_prediction(self, item: RiskAssessmentPrediction) -> Self {
        let mut resource = self.clone();
        resource.prediction.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_mitigation(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.mitigation = Some(value);
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
}

impl crate::traits::risk_assessment::RiskAssessmentExistence for RiskAssessment {
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some() || self.occurrence_period.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_method(&self) -> bool {
        self.method.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_condition(&self) -> bool {
        self.condition.is_some()
    }
    fn has_performer(&self) -> bool {
        self.performer.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_basis(&self) -> bool {
        self.basis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_prediction(&self) -> bool {
        self.prediction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_mitigation(&self) -> bool {
        self.mitigation.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for RiskAssessment {
    fn resource_type(&self) -> &'static str {
        "RiskAssessment"
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
        Some("http://hl7.org/fhir/StructureDefinition/RiskAssessment")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::risk_assessment::{
    RiskAssessmentAccessors, RiskAssessmentExistence, RiskAssessmentMutators,
};
