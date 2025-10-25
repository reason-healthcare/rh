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
                "AdverseEvent.actuality",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/adverse-event-actuality|4.0.1",
            )
            .with_description("Overall nature of the adverse event, e.g. real or potential."),
            rh_foundation::ElementBinding::new(
                "AdverseEvent.outcome",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/adverse-event-outcome|4.0.1",
            )
            .with_description("TODO (and should this be required?)."),
            rh_foundation::ElementBinding::new(
                "AdverseEvent.severity",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/adverse-event-severity|4.0.1",
            )
            .with_description(
                "The severity of the adverse event itself, in direct relation to the subject.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("AdverseEvent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.contained", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.extension", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.actuality", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.category", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.event", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.detected", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.recordedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.resultingCondition", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.seriousness", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.severity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.outcome", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.recorder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.contributor", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.instance",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity.causality", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.assessment",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.productRelatedness",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.author",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.method",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.subjectMedicalHistory", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.referenceDocument", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.study", 0, None),
        ]
    });

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

impl crate::validation::ValidatableResource for AdverseEvent {
    fn resource_type(&self) -> &'static str {
        "AdverseEvent"
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
        Some("http://hl7.org/fhir/StructureDefinition/AdverseEvent")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::adverse_event::{
    AdverseEventAccessors, AdverseEventExistence, AdverseEventMutators,
};
