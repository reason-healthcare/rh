use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Condition
///
/// A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Condition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Condition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this condition
    pub identifier: Option<Vec<Identifier>>,
    /// active | recurrence | relapse | inactive | remission | resolved
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,
    /// unconfirmed | provisional | differential | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodeableConcept>,
    /// problem-list-item | encounter-diagnosis
    ///
    /// Binding: extensible (A category assigned to the condition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/condition-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Subjective severity of condition
    ///
    /// Binding: preferred (A subjective assessment of the severity of the condition as evaluated by the clinician.)
    ///
    /// Available values:
    /// - `24484000`
    /// - `6736007`
    /// - `255604002`
    pub severity: Option<CodeableConcept>,
    /// Identification of the condition, problem or diagnosis
    ///
    /// Binding: example (Identification of the condition or diagnosis.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub code: Option<CodeableConcept>,
    /// Anatomical location, if relevant
    ///
    /// Binding: example (Codes describing anatomical locations. May include laterality.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// Who has the condition?
    pub subject: Reference,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// Estimated or actual date,  date-time, or age (dateTime)
    #[serde(rename = "onsetDateTime")]
    pub onset_date_time: Option<DateTimeType>,
    /// Estimated or actual date,  date-time, or age (Age)
    #[serde(rename = "onsetAge")]
    pub onset_age: Option<Age>,
    /// Estimated or actual date,  date-time, or age (Period)
    #[serde(rename = "onsetPeriod")]
    pub onset_period: Option<Period>,
    /// Estimated or actual date,  date-time, or age (Range)
    #[serde(rename = "onsetRange")]
    pub onset_range: Option<Range>,
    /// Estimated or actual date,  date-time, or age (string)
    #[serde(rename = "onsetString")]
    pub onset_string: Option<StringType>,
    /// When in resolution/remission (dateTime)
    #[serde(rename = "abatementDateTime")]
    pub abatement_date_time: Option<DateTimeType>,
    /// When in resolution/remission (Age)
    #[serde(rename = "abatementAge")]
    pub abatement_age: Option<Age>,
    /// When in resolution/remission (Period)
    #[serde(rename = "abatementPeriod")]
    pub abatement_period: Option<Period>,
    /// When in resolution/remission (Range)
    #[serde(rename = "abatementRange")]
    pub abatement_range: Option<Range>,
    /// When in resolution/remission (string)
    #[serde(rename = "abatementString")]
    pub abatement_string: Option<StringType>,
    /// Date record was first recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<DateTimeType>,
    /// Extension element for the 'recordedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recordedDate")]
    pub _recorded_date: Option<Element>,
    /// Who recorded the condition
    pub recorder: Option<Reference>,
    /// Person who asserts this condition
    pub asserter: Option<Reference>,
    /// Stage/grade, usually assessed formally
    pub stage: Option<Vec<ConditionStage>>,
    /// Supporting evidence
    pub evidence: Option<Vec<ConditionEvidence>>,
    /// Additional information about the Condition
    pub note: Option<Vec<Annotation>>,
}
/// Condition nested structure for the 'stage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionStage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Simple summary (disease specific)
    ///
    /// Binding: example (Codes describing condition stages (e.g. Cancer stages).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/condition-stage
    pub summary: Option<CodeableConcept>,
    /// Formal record of assessment
    pub assessment: Option<Vec<Reference>>,
    /// Kind of staging
    ///
    /// Binding: example (Codes describing the kind of condition staging (e.g. clinical or pathological).)
    ///
    /// Available values:
    /// - `261023001`: Pathological staging (qualifier value)
    /// - `260998006`: Clinical staging (qualifier value)
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
}
/// Condition nested structure for the 'evidence' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionEvidence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Manifestation/symptom
    ///
    /// Binding: example (Codes that describe the manifestation or symptoms of a condition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/manifestation-or-symptom
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting information found elsewhere
    pub detail: Option<Vec<Reference>>,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            clinical_status: Default::default(),
            verification_status: Default::default(),
            category: Default::default(),
            severity: Default::default(),
            code: Default::default(),
            body_site: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            onset_date_time: Default::default(),
            onset_age: Default::default(),
            onset_period: Default::default(),
            onset_range: Default::default(),
            onset_string: Default::default(),
            abatement_date_time: Default::default(),
            abatement_age: Default::default(),
            abatement_period: Default::default(),
            abatement_range: Default::default(),
            abatement_string: Default::default(),
            recorded_date: Default::default(),
            _recorded_date: Default::default(),
            recorder: Default::default(),
            asserter: Default::default(),
            stage: Default::default(),
            evidence: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for ConditionStage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            summary: Default::default(),
            assessment: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Default for ConditionEvidence {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            detail: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Condition {
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

impl crate::traits::resource::ResourceMutators for Condition {
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

impl crate::traits::resource::ResourceExistence for Condition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Condition {
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

impl crate::traits::domain_resource::DomainResourceMutators for Condition {
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

impl crate::traits::domain_resource::DomainResourceExistence for Condition {
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

impl crate::traits::condition::ConditionAccessors for Condition {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn clinical_status(&self) -> Option<CodeableConcept> {
        self.clinical_status.clone()
    }
    fn verification_status(&self) -> Option<CodeableConcept> {
        self.verification_status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn severity(&self) -> Option<CodeableConcept> {
        self.severity.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn body_site(&self) -> &[CodeableConcept] {
        self.body_site.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn recorded_date(&self) -> Option<DateTimeType> {
        self.recorded_date.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn asserter(&self) -> Option<Reference> {
        self.asserter.clone()
    }
    fn stage(&self) -> &[ConditionStage] {
        self.stage.as_deref().unwrap_or(&[])
    }
    fn evidence(&self) -> &[ConditionEvidence] {
        self.evidence.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::condition::ConditionMutators for Condition {
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
    fn set_clinical_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.clinical_status = Some(value);
        resource
    }
    fn set_verification_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.verification_status = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_severity(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.severity = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn add_body_site(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site.get_or_insert_with(Vec::new).push(item);
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
    fn set_recorded_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded_date = Some(value);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_asserter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.asserter = Some(value);
        resource
    }
    fn set_stage(self, value: Vec<ConditionStage>) -> Self {
        let mut resource = self.clone();
        resource.stage = Some(value);
        resource
    }
    fn add_stage(self, item: ConditionStage) -> Self {
        let mut resource = self.clone();
        resource.stage.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_evidence(self, value: Vec<ConditionEvidence>) -> Self {
        let mut resource = self.clone();
        resource.evidence = Some(value);
        resource
    }
    fn add_evidence(self, item: ConditionEvidence) -> Self {
        let mut resource = self.clone();
        resource.evidence.get_or_insert_with(Vec::new).push(item);
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

impl crate::traits::condition::ConditionExistence for Condition {
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
    fn has_abatement(&self) -> bool {
        self.abatement_date_time.is_some()
            || self.abatement_age.is_some()
            || self.abatement_period.is_some()
            || self.abatement_range.is_some()
            || self.abatement_string.is_some()
    }
    fn has_onset(&self) -> bool {
        self.onset_date_time.is_some()
            || self.onset_age.is_some()
            || self.onset_period.is_some()
            || self.onset_range.is_some()
            || self.onset_string.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_clinical_status(&self) -> bool {
        self.clinical_status.is_some()
    }
    fn has_verification_status(&self) -> bool {
        self.verification_status.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_severity(&self) -> bool {
        self.severity.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_body_site(&self) -> bool {
        self.body_site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_recorded_date(&self) -> bool {
        self.recorded_date.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_asserter(&self) -> bool {
        self.asserter.is_some()
    }
    fn has_stage(&self) -> bool {
        self.stage.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_evidence(&self) -> bool {
        self.evidence.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}
