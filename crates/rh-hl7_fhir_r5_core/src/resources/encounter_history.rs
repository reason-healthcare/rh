use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EncounterHistory
///
/// A record of significant events/milestones key data throughout the history of an Encounter
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EncounterHistory
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// The Encounter associated with this set of historic values
    pub encounter: Option<Reference>,
    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<Identifier>>,
    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: EncounterStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Classification of patient encounter
    ///
    /// Binding: extensible (Classification of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActEncounterCode
    pub class: CodeableConcept,
    /// Specific type of encounter
    ///
    /// Binding: example (A specific code indicating type of service provided)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Specific type of service
    ///
    /// Binding: example (Broad categorization of the service that is to be provided.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The patient or group related to this encounter
    pub subject: Option<Reference>,
    /// The current status of the subject in relation to the Encounter
    ///
    /// Binding: example (Current status of the subject  within the encounter.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-subject-status
    #[serde(rename = "subjectStatus")]
    pub subject_status: Option<CodeableConcept>,
    /// The actual start and end time associated with this set of values associated with the encounter
    #[serde(rename = "actualPeriod")]
    pub actual_period: Option<Period>,
    /// The planned start date/time (or admission date) of the encounter
    #[serde(rename = "plannedStartDate")]
    pub planned_start_date: Option<DateTimeType>,
    /// Extension element for the 'plannedStartDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_plannedStartDate")]
    pub _planned_start_date: Option<Element>,
    /// The planned end date/time (or discharge date) of the encounter
    #[serde(rename = "plannedEndDate")]
    pub planned_end_date: Option<DateTimeType>,
    /// Extension element for the 'plannedEndDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_plannedEndDate")]
    pub _planned_end_date: Option<Element>,
    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<Duration>,
    /// Location of the patient at this point in the encounter
    pub location: Option<Vec<EncounterHistoryLocation>>,
}
/// EncounterHistory nested structure for the 'location' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterHistoryLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Location the encounter takes place
    pub location: Reference,
    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    ///
    /// Binding: example (Physical form of the location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-form
    pub form: Option<CodeableConcept>,
}

impl Default for EncounterHistory {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            encounter: Default::default(),
            identifier: Default::default(),
            status: EncounterStatus::default(),
            _status: Default::default(),
            class: CodeableConcept::default(),
            type_: Default::default(),
            service_type: Default::default(),
            subject: Default::default(),
            subject_status: Default::default(),
            actual_period: Default::default(),
            planned_start_date: Default::default(),
            _planned_start_date: Default::default(),
            planned_end_date: Default::default(),
            _planned_end_date: Default::default(),
            length: Default::default(),
            location: Default::default(),
        }
    }
}

impl Default for EncounterHistoryLocation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            location: Reference::default(),
            form: Default::default(),
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
                "EncounterHistory.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "EncounterHistory.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/encounter-status|5.0.0",
            )
            .with_description("Current state of the encounter."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EncounterHistory.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.contained", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.extension", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.class", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.type", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.serviceType", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.subjectStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.actualPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.plannedStartDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.plannedEndDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.length", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.location", 0, None),
            rh_foundation::ElementCardinality::new("EncounterHistory.location.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EncounterHistory.location.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "EncounterHistory.location.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "EncounterHistory.location.location",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EncounterHistory.location.form", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EncounterHistory {
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

impl crate::traits::resource::ResourceMutators for EncounterHistory {
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

impl crate::traits::resource::ResourceExistence for EncounterHistory {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EncounterHistory {
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

impl crate::traits::domain_resource::DomainResourceMutators for EncounterHistory {
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

impl crate::traits::domain_resource::DomainResourceExistence for EncounterHistory {
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

impl crate::traits::encounter_history::EncounterHistoryAccessors for EncounterHistory {
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EncounterStatus {
        self.status.clone()
    }
    fn class(&self) -> CodeableConcept {
        self.class.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn service_type(&self) -> &[CodeableReference] {
        self.service_type.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn subject_status(&self) -> Option<CodeableConcept> {
        self.subject_status.clone()
    }
    fn actual_period(&self) -> Option<Period> {
        self.actual_period.clone()
    }
    fn planned_start_date(&self) -> Option<DateTimeType> {
        self.planned_start_date.clone()
    }
    fn planned_end_date(&self) -> Option<DateTimeType> {
        self.planned_end_date.clone()
    }
    fn length(&self) -> Option<Duration> {
        self.length.clone()
    }
    fn location(&self) -> &[EncounterHistoryLocation] {
        self.location.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::encounter_history::EncounterHistoryMutators for EncounterHistory {
    fn new() -> Self {
        Self::default()
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
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
    fn set_status(self, value: EncounterStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_class(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.class = value;
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
    fn set_service_type(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.service_type = Some(value);
        resource
    }
    fn add_service_type(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource
            .service_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_subject_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.subject_status = Some(value);
        resource
    }
    fn set_actual_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.actual_period = Some(value);
        resource
    }
    fn set_planned_start_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.planned_start_date = Some(value);
        resource
    }
    fn set_planned_end_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.planned_end_date = Some(value);
        resource
    }
    fn set_length(self, value: Duration) -> Self {
        let mut resource = self.clone();
        resource.length = Some(value);
        resource
    }
    fn set_location(self, value: Vec<EncounterHistoryLocation>) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn add_location(self, item: EncounterHistoryLocation) -> Self {
        let mut resource = self.clone();
        resource.location.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::encounter_history::EncounterHistoryExistence for EncounterHistory {
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_class(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_type(&self) -> bool {
        self.service_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_subject_status(&self) -> bool {
        self.subject_status.is_some()
    }
    fn has_actual_period(&self) -> bool {
        self.actual_period.is_some()
    }
    fn has_planned_start_date(&self) -> bool {
        self.planned_start_date.is_some()
    }
    fn has_planned_end_date(&self) -> bool {
        self.planned_end_date.is_some()
    }
    fn has_length(&self) -> bool {
        self.length.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for EncounterHistory {
    fn resource_type(&self) -> &'static str {
        "EncounterHistory"
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
        Some("http://hl7.org/fhir/StructureDefinition/EncounterHistory")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::encounter_history::{
    EncounterHistoryAccessors, EncounterHistoryExistence, EncounterHistoryMutators,
};
