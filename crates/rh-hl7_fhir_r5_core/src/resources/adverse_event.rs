use crate::bindings::adverse_event_actuality::AdverseEventActuality;
use crate::bindings::adverse_event_status::AdverseEventStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// AdverseEvent
///
/// An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: AdverseEvent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// in-progress | completed | entered-in-error | unknown
    pub status: AdverseEventStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// actual | potential
    pub actuality: AdverseEventActuality,
    /// Extension element for the 'actuality' primitive field. Contains metadata and extensions.
    pub _actuality: Option<Element>,
    /// wrong-patient | procedure-mishap | medication-mishap | device | unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    ///
    /// Binding: example (Overall categorization of the event, e.g. product-related or situational.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// Event or incident that occurred or was averted
    ///
    /// Binding: example (Detailed type of event.)
    ///
    /// Available values:
    /// - `1912002`
    pub code: Option<CodeableConcept>,
    /// Subject impacted by event
    pub subject: Reference,
    /// The Encounter associated with the start of the AdverseEvent
    pub encounter: Option<Reference>,
    /// When the event occurred (dateTime)
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<DateTimeType>,
    /// When the event occurred (Period)
    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,
    /// When the event occurred (Timing)
    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,
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
    #[serde(rename = "resultingEffect")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resulting_effect: Vec<Reference>,
    /// Location where adverse event occurred
    pub location: Option<Reference>,
    /// Seriousness or gravity of the event
    ///
    /// Binding: example (Overall seriousness of this event for the patient.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-seriousness
    pub seriousness: Option<CodeableConcept>,
    /// Type of outcome from the adverse event
    ///
    /// Binding: example (Codes describing the type of outcome from the adverse event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome: Vec<CodeableConcept>,
    /// Who recorded the adverse event
    pub recorder: Option<Reference>,
    /// Who was involved in the adverse event or the potential adverse event and what they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<AdverseEventParticipant>,
    /// Research study that the subject is enrolled in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<Reference>,
    /// Considered likely or probable or anticipated in the research study
    #[serde(rename = "expectedInResearchStudy")]
    pub expected_in_research_study: Option<BooleanType>,
    /// Extension element for the 'expectedInResearchStudy' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expectedInResearchStudy")]
    pub _expected_in_research_study: Option<Element>,
    /// The suspected agent causing the adverse event
    #[serde(rename = "suspectEntity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suspect_entity: Vec<AdverseEventSuspectentity>,
    /// Contributing factors suspected to have increased the probability or severity of the adverse event
    #[serde(rename = "contributingFactor")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributing_factor: Vec<AdverseEventContributingfactor>,
    /// Preventive actions that contributed to avoiding the adverse event
    #[serde(rename = "preventiveAction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preventive_action: Vec<AdverseEventPreventiveaction>,
    /// Ameliorating actions taken after the adverse event occured in order to reduce the extent of harm
    #[serde(rename = "mitigatingAction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigating_action: Vec<AdverseEventMitigatingaction>,
    /// Supporting information relevant to the event
    #[serde(rename = "supportingInfo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<AdverseEventSupportinginfo>,
    /// Comment on adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
}
/// AdverseEvent nested structure for the 'contributingFactor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventContributingfactor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Item suspected to have increased the probability or severity of the adverse event (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Item suspected to have increased the probability or severity of the adverse event (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}
/// AdverseEvent nested structure for the 'mitigatingAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventMitigatingaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Ameliorating action taken after the adverse event occured in order to reduce the extent of harm (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Ameliorating action taken after the adverse event occured in order to reduce the extent of harm (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}
/// AdverseEvent nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of involvement
    ///
    /// Binding: example (Codes describing the type of involvement of the actor in the adverse event.)
    ///
    /// Available values:
    /// - `INF`
    /// - `PART`
    /// - `WIT`
    /// - `AUT`
    pub function: Option<CodeableConcept>,
    /// Who was involved in the adverse event or the potential adverse event
    pub actor: Reference,
}
/// AdverseEvent nested structure for the 'preventiveAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventPreventiveaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Action that contributed to avoiding the adverse event (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Action that contributed to avoiding the adverse event (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}
/// AdverseEvent nested structure for the 'supportingInfo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventSupportinginfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Subject medical history or document relevant to this adverse event (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// Subject medical history or document relevant to this adverse event (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}
/// AdverseEvent nested structure for the 'suspectEntity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventSuspectentity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information on the possible cause of the event
    pub causality: Option<AdverseEventSuspectentityCausality>,
    /// Refers to the specific entity that caused the adverse event (CodeableConcept)
    #[serde(rename = "instanceCodeableConcept")]
    pub instance_codeable_concept: CodeableConcept,
    /// Refers to the specific entity that caused the adverse event (Reference)
    #[serde(rename = "instanceReference")]
    pub instance_reference: Reference,
}
/// AdverseEventSuspectentity nested structure for the 'causality' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdverseEventSuspectentityCausality {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Method of evaluating the relatedness of the suspected entity to the event
    ///
    /// Binding: example (TODO.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-causality-method
    #[serde(rename = "assessmentMethod")]
    pub assessment_method: Option<CodeableConcept>,
    /// Result of the assessment regarding the relatedness of the suspected entity to the event
    ///
    /// Binding: example (Codes for the assessment of whether the entity caused the event.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/adverse-event-causality-assess
    #[serde(rename = "entityRelatedness")]
    pub entity_relatedness: Option<CodeableConcept>,
    /// Author of the information on the possible cause of the event
    pub author: Option<Reference>,
}

impl Default for AdverseEvent {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: AdverseEventStatus::default(),
            _status: Default::default(),
            actuality: AdverseEventActuality::default(),
            _actuality: Default::default(),
            category: Default::default(),
            code: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            occurrence_date_time: Default::default(),
            occurrence_period: Default::default(),
            occurrence_timing: Default::default(),
            detected: Default::default(),
            _detected: Default::default(),
            recorded_date: Default::default(),
            _recorded_date: Default::default(),
            resulting_effect: Default::default(),
            location: Default::default(),
            seriousness: Default::default(),
            outcome: Default::default(),
            recorder: Default::default(),
            participant: Default::default(),
            study: Default::default(),
            expected_in_research_study: Default::default(),
            _expected_in_research_study: Default::default(),
            suspect_entity: Default::default(),
            contributing_factor: Default::default(),
            preventive_action: Default::default(),
            mitigating_action: Default::default(),
            supporting_info: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for AdverseEventContributingfactor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_reference: Default::default(),
            item_codeable_concept: Default::default(),
        }
    }
}

impl Default for AdverseEventMitigatingaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_reference: Default::default(),
            item_codeable_concept: Default::default(),
        }
    }
}

impl Default for AdverseEventParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for AdverseEventPreventiveaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_reference: Default::default(),
            item_codeable_concept: Default::default(),
        }
    }
}

impl Default for AdverseEventSupportinginfo {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_reference: Default::default(),
            item_codeable_concept: Default::default(),
        }
    }
}

impl Default for AdverseEventSuspectentity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            causality: Default::default(),
            instance_codeable_concept: Default::default(),
            instance_reference: Default::default(),
        }
    }
}

impl Default for AdverseEventSuspectentityCausality {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            assessment_method: Default::default(),
            entity_relatedness: Default::default(),
            author: Default::default(),
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
                "http://hl7.org/fhir/ValueSet/adverse-event-actuality|5.0.0",
            )
            .with_description("Overall nature of the adverse event, e.g. real or potential."),
            rh_foundation::ElementBinding::new(
                "AdverseEvent.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "AdverseEvent.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/adverse-event-status|5.0.0",
            )
            .with_description("Codes identifying the lifecycle stage of an event."),
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
            rh_foundation::ElementCardinality::new("AdverseEvent.identifier", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.actuality", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.category", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.occurrence[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.detected", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.recordedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.resultingEffect", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.seriousness", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.outcome", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.recorder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.participant", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.participant.function", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.participant.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.study", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.expectedInResearchStudy",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("AdverseEvent.suspectEntity.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.instance[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality",
                0,
                Some(1),
            ),
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
                "AdverseEvent.suspectEntity.causality.assessmentMethod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.entityRelatedness",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.suspectEntity.causality.author",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.contributingFactor", 0, None),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.contributingFactor.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.contributingFactor.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.contributingFactor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.contributingFactor.item[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.preventiveAction", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.preventiveAction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.preventiveAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.preventiveAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.preventiveAction.item[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.mitigatingAction", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.mitigatingAction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.mitigatingAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.mitigatingAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.mitigatingAction.item[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.supportingInfo", 0, None),
            rh_foundation::ElementCardinality::new("AdverseEvent.supportingInfo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.supportingInfo.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.supportingInfo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "AdverseEvent.supportingInfo.item[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("AdverseEvent.note", 0, None),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for AdverseEvent {
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

impl crate::traits::adverse_event::AdverseEventAccessors for AdverseEvent {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn status(&self) -> AdverseEventStatus {
        self.status.clone()
    }
    fn actuality(&self) -> AdverseEventActuality {
        self.actuality.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
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
    fn detected(&self) -> Option<DateTimeType> {
        self.detected.clone()
    }
    fn recorded_date(&self) -> Option<DateTimeType> {
        self.recorded_date.clone()
    }
    fn resulting_effect(&self) -> &[Reference] {
        self.resulting_effect.as_slice()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn seriousness(&self) -> Option<CodeableConcept> {
        self.seriousness.clone()
    }
    fn outcome(&self) -> &[CodeableConcept] {
        self.outcome.as_slice()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn participant(&self) -> &[AdverseEventParticipant] {
        self.participant.as_slice()
    }
    fn study(&self) -> &[Reference] {
        self.study.as_slice()
    }
    fn expected_in_research_study(&self) -> Option<BooleanType> {
        self.expected_in_research_study
    }
    fn suspect_entity(&self) -> &[AdverseEventSuspectentity] {
        self.suspect_entity.as_slice()
    }
    fn contributing_factor(&self) -> &[AdverseEventContributingfactor] {
        self.contributing_factor.as_slice()
    }
    fn preventive_action(&self) -> &[AdverseEventPreventiveaction] {
        self.preventive_action.as_slice()
    }
    fn mitigating_action(&self) -> &[AdverseEventMitigatingaction] {
        self.mitigating_action.as_slice()
    }
    fn supporting_info(&self) -> &[AdverseEventSupportinginfo] {
        self.supporting_info.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
}

impl crate::traits::adverse_event::AdverseEventMutators for AdverseEvent {
    fn new() -> Self {
        Self::default()
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
    fn set_status(self, value: AdverseEventStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_actuality(self, value: AdverseEventActuality) -> Self {
        let mut resource = self.clone();
        resource.actuality = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
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
    fn set_resulting_effect(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.resulting_effect = value;
        resource
    }
    fn add_resulting_effect(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.resulting_effect.push(item);
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
    fn set_outcome(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.outcome = value;
        resource
    }
    fn add_outcome(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.outcome.push(item);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<AdverseEventParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = value;
        resource
    }
    fn add_participant(self, item: AdverseEventParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.push(item);
        resource
    }
    fn set_study(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.study = value;
        resource
    }
    fn add_study(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.study.push(item);
        resource
    }
    fn set_expected_in_research_study(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.expected_in_research_study = Some(value);
        resource
    }
    fn set_suspect_entity(self, value: Vec<AdverseEventSuspectentity>) -> Self {
        let mut resource = self.clone();
        resource.suspect_entity = value;
        resource
    }
    fn add_suspect_entity(self, item: AdverseEventSuspectentity) -> Self {
        let mut resource = self.clone();
        resource.suspect_entity.push(item);
        resource
    }
    fn set_contributing_factor(self, value: Vec<AdverseEventContributingfactor>) -> Self {
        let mut resource = self.clone();
        resource.contributing_factor = value;
        resource
    }
    fn add_contributing_factor(self, item: AdverseEventContributingfactor) -> Self {
        let mut resource = self.clone();
        resource.contributing_factor.push(item);
        resource
    }
    fn set_preventive_action(self, value: Vec<AdverseEventPreventiveaction>) -> Self {
        let mut resource = self.clone();
        resource.preventive_action = value;
        resource
    }
    fn add_preventive_action(self, item: AdverseEventPreventiveaction) -> Self {
        let mut resource = self.clone();
        resource.preventive_action.push(item);
        resource
    }
    fn set_mitigating_action(self, value: Vec<AdverseEventMitigatingaction>) -> Self {
        let mut resource = self.clone();
        resource.mitigating_action = value;
        resource
    }
    fn add_mitigating_action(self, item: AdverseEventMitigatingaction) -> Self {
        let mut resource = self.clone();
        resource.mitigating_action.push(item);
        resource
    }
    fn set_supporting_info(self, value: Vec<AdverseEventSupportinginfo>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = value;
        resource
    }
    fn add_supporting_info(self, item: AdverseEventSupportinginfo) -> Self {
        let mut resource = self.clone();
        resource.supporting_info.push(item);
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
}

impl crate::traits::adverse_event::AdverseEventExistence for AdverseEvent {
    fn has_occurrence(&self) -> bool {
        self.occurrence_date_time.is_some()
            || self.occurrence_period.is_some()
            || self.occurrence_timing.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_actuality(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
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
    fn has_detected(&self) -> bool {
        self.detected.is_some()
    }
    fn has_recorded_date(&self) -> bool {
        self.recorded_date.is_some()
    }
    fn has_resulting_effect(&self) -> bool {
        !self.resulting_effect.is_empty()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_seriousness(&self) -> bool {
        self.seriousness.is_some()
    }
    fn has_outcome(&self) -> bool {
        !self.outcome.is_empty()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_participant(&self) -> bool {
        !self.participant.is_empty()
    }
    fn has_study(&self) -> bool {
        !self.study.is_empty()
    }
    fn has_expected_in_research_study(&self) -> bool {
        self.expected_in_research_study.is_some()
    }
    fn has_suspect_entity(&self) -> bool {
        !self.suspect_entity.is_empty()
    }
    fn has_contributing_factor(&self) -> bool {
        !self.contributing_factor.is_empty()
    }
    fn has_preventive_action(&self) -> bool {
        !self.preventive_action.is_empty()
    }
    fn has_mitigating_action(&self) -> bool {
        !self.mitigating_action.is_empty()
    }
    fn has_supporting_info(&self) -> bool {
        !self.supporting_info.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
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
