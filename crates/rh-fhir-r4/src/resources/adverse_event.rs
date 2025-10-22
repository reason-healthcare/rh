use crate::bindings::adverse_event_actuality::AdverseEventActuality;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AdverseEvent
///
/// Actual or  potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional monitoring, treatment, or hospitalization, or that results in death.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the event
    pub identifier: Option<Identifier>,
    /// actual | potential
    pub actuality: AdverseEventActuality,
    /// Extension element for the 'actuality' primitive field. Contains metadata and extensions.
    pub _actuality: Option<Element>,
    /// product-problem | product-quality | product-use-error | wrong-dose | incorrect-prescribing-information | wrong-technique | wrong-route-of-administration | wrong-rate | wrong-duration | wrong-time | expired-drug | medical-device-use-error | problem-different-manufacturer | unsafe-physical-environment
    ///
    /// Binding: extensible (Overall categorization of the event, e.g. product-related or situational.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of the event itself in relation to the subject
    ///
    /// Binding: example (Detailed type of event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-type
    pub event: Option<CodeableConcept>,
    /// Subject impacted by event
    pub subject: Reference,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// When the event occurred
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// When the event was detected
    pub detected: Option<DateTimeType>,
    /// Extension element for the 'detected' primitive field. Contains metadata and extensions.
    pub _detected: Option<Element>,
    /// When the event was recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<DateTimeType>,
    /// Extension element for the 'recordedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recordedDate")]
    pub _recorded_date: Option<Element>,
    /// Effect on the subject due to this event
    #[serde(rename = "resultingCondition")]
    pub resulting_condition: Option<Vec<Reference>>,
    /// Location where adverse event occurred
    pub location: Option<Reference>,
    /// Seriousness of the event
    ///
    /// Binding: example (Overall seriousness of this event for the patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-seriousness
    pub seriousness: Option<CodeableConcept>,
    /// mild | moderate | severe
    pub severity: Option<CodeableConcept>,
    /// resolved | recovering | ongoing | resolvedWithSequelae | fatal | unknown
    pub outcome: Option<CodeableConcept>,
    /// Who recorded the adverse event
    pub recorder: Option<Reference>,
    /// Who  was involved in the adverse event or the potential adverse event
    pub contributor: Option<Vec<Reference>>,
    /// The suspected agent causing the adverse event
    #[serde(rename = "suspectEntity")]
    pub suspect_entity: Option<Vec<AdverseEventSuspectentity>>,
    /// AdverseEvent.subjectMedicalHistory
    #[serde(rename = "subjectMedicalHistory")]
    pub subject_medical_history: Option<Vec<Reference>>,
    /// AdverseEvent.referenceDocument
    #[serde(rename = "referenceDocument")]
    pub reference_document: Option<Vec<Reference>>,
    /// AdverseEvent.study
    pub study: Option<Vec<Reference>>,
}
/// AdverseEventSuspectentity nested structure for the 'causality' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventSuspectentityCausality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Assessment of if the entity caused the event
    ///
    /// Binding: example (Codes for the assessment of whether the entity caused the event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-causality-assess
    pub assessment: Option<CodeableConcept>,
    /// AdverseEvent.suspectEntity.causalityProductRelatedness
    #[serde(rename = "productRelatedness")]
    pub product_relatedness: Option<StringType>,
    /// Extension element for the 'productRelatedness' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productRelatedness")]
    pub _product_relatedness: Option<Element>,
    /// AdverseEvent.suspectEntity.causalityAuthor
    pub author: Option<Reference>,
    /// ProbabilityScale | Bayesian | Checklist
    ///
    /// Binding: example (TODO.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-causality-method
    pub method: Option<CodeableConcept>,
}
/// AdverseEvent nested structure for the 'suspectEntity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventSuspectentity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information on the possible cause of the event
    pub causality: Option<Vec<AdverseEventSuspectentityCausality>>,
    /// Refers to the specific entity that caused the adverse event
    pub instance: Reference,
}

impl Default for AdverseEvent {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            actuality: AdverseEventActuality::default(),
            _actuality: Default::default(),
            category: Default::default(),
            event: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            detected: Default::default(),
            _detected: Default::default(),
            recorded_date: Default::default(),
            _recorded_date: Default::default(),
            resulting_condition: Default::default(),
            location: Default::default(),
            seriousness: Default::default(),
            severity: Default::default(),
            outcome: Default::default(),
            recorder: Default::default(),
            contributor: Default::default(),
            suspect_entity: Default::default(),
            subject_medical_history: Default::default(),
            reference_document: Default::default(),
            study: Default::default(),
        }
    }
}

impl Default for AdverseEventSuspectentityCausality {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            assessment: Default::default(),
            product_relatedness: Default::default(),
            _product_relatedness: Default::default(),
            author: Default::default(),
            method: Default::default(),
        }
    }
}

impl Default for AdverseEventSuspectentity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            causality: Default::default(),
            instance: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for AdverseEvent {
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

impl crate::traits::resource::ResourceMutators for AdverseEvent {
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

impl crate::traits::resource::ResourceExistence for AdverseEvent {
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

impl crate::traits::domain_resource::DomainResourceAccessors for AdverseEvent {
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

impl crate::traits::domain_resource::DomainResourceMutators for AdverseEvent {
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

impl crate::traits::domain_resource::DomainResourceExistence for AdverseEvent {
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

impl crate::traits::adverse_event::AdverseEventAccessors for AdverseEvent {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn actuality(&self) -> AdverseEventActuality {
        self.actuality.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn event(&self) -> Option<CodeableConcept> {
        self.event.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn detected(&self) -> Option<DateTimeType> {
        self.detected.clone()
    }
    fn recorded_date(&self) -> Option<DateTimeType> {
        self.recorded_date.clone()
    }
    fn resulting_condition(&self) -> &[Reference] {
        self.resulting_condition.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn seriousness(&self) -> Option<CodeableConcept> {
        self.seriousness.clone()
    }
    fn severity(&self) -> Option<CodeableConcept> {
        self.severity.clone()
    }
    fn outcome(&self) -> Option<CodeableConcept> {
        self.outcome.clone()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn contributor(&self) -> &[Reference] {
        self.contributor.as_deref().unwrap_or(&[])
    }
    fn suspect_entity(&self) -> &[AdverseEventSuspectentity] {
        self.suspect_entity.as_deref().unwrap_or(&[])
    }
    fn subject_medical_history(&self) -> &[Reference] {
        self.subject_medical_history.as_deref().unwrap_or(&[])
    }
    fn reference_document(&self) -> &[Reference] {
        self.reference_document.as_deref().unwrap_or(&[])
    }
    fn study(&self) -> &[Reference] {
        self.study.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::adverse_event::AdverseEventMutators for AdverseEvent {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_actuality(self, value: AdverseEventActuality) -> Self {
        let mut resource = self.clone();
        resource.actuality = value;
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
    fn set_event(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.event = Some(value);
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
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_detected(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.detected = Some(value);
        resource
    }
    fn set_recorded_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded_date = Some(value);
        resource
    }
    fn set_resulting_condition(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.resulting_condition = Some(value);
        resource
    }
    fn add_resulting_condition(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .resulting_condition
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_seriousness(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.seriousness = Some(value);
        resource
    }
    fn set_severity(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.severity = Some(value);
        resource
    }
    fn set_outcome(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.outcome = Some(value);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_contributor(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.contributor = Some(value);
        resource
    }
    fn add_contributor(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.contributor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_suspect_entity(self, value: Vec<AdverseEventSuspectentity>) -> Self {
        let mut resource = self.clone();
        resource.suspect_entity = Some(value);
        resource
    }
    fn add_suspect_entity(self, item: AdverseEventSuspectentity) -> Self {
        let mut resource = self.clone();
        resource
            .suspect_entity
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject_medical_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject_medical_history = Some(value);
        resource
    }
    fn add_subject_medical_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .subject_medical_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_reference_document(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reference_document = Some(value);
        resource
    }
    fn add_reference_document(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reference_document
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_study(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.study = Some(value);
        resource
    }
    fn add_study(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.study.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::adverse_event::AdverseEventExistence for AdverseEvent {
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
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_actuality(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_event(&self) -> bool {
        self.event.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_detected(&self) -> bool {
        self.detected.is_some()
    }
    fn has_recorded_date(&self) -> bool {
        self.recorded_date.is_some()
    }
    fn has_resulting_condition(&self) -> bool {
        self.resulting_condition
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_seriousness(&self) -> bool {
        self.seriousness.is_some()
    }
    fn has_severity(&self) -> bool {
        self.severity.is_some()
    }
    fn has_outcome(&self) -> bool {
        self.outcome.is_some()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_contributor(&self) -> bool {
        self.contributor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_suspect_entity(&self) -> bool {
        self.suspect_entity.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject_medical_history(&self) -> bool {
        self.subject_medical_history
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_reference_document(&self) -> bool {
        self.reference_document
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_study(&self) -> bool {
        self.study.as_ref().is_some_and(|v| !v.is_empty())
    }
}
