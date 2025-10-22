use crate::bindings::observation_status::ObservationStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
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
/// - Version: 4.0.1
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
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why the assessment was necessary?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
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
            reason_code: Default::default(),
            reason_reference: Default::default(),
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
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
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
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
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
