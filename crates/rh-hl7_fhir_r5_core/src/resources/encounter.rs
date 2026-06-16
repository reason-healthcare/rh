use crate::bindings::encounter_location_status::EncounterLocationStatus;
use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::virtual_service_detail::VirtualServiceDetail;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Encounter
///
/// An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: EncounterStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Classification of patient encounter context - e.g. Inpatient, outpatient
    ///
    /// Binding: preferred (Classification of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/encounter-class
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<CodeableConcept>,
    /// Indicates the urgency of the encounter
    ///
    /// Binding: example (Indicates the urgency of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActPriority
    pub priority: Option<CodeableConcept>,
    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, ...)
    ///
    /// Binding: example (A specific code indicating type of service provided)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
    /// Specific type of service
    ///
    /// Binding: example (Broad categorization of the service that is to be provided.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "serviceType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<CodeableReference>,
    /// The patient or group related to this encounter
    pub subject: Option<Reference>,
    /// The current status of the subject in relation to the Encounter
    ///
    /// Binding: example (Current status of the subject  within the encounter.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-subject-status
    #[serde(rename = "subjectStatus")]
    pub subject_status: Option<CodeableConcept>,
    /// Episode(s) of care that this encounter should be recorded against
    #[serde(rename = "episodeOfCare")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episode_of_care: Vec<Reference>,
    /// The request that initiated this encounter
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// The group(s) that are allocated to participate in this encounter
    #[serde(rename = "careTeam")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<Reference>,
    /// Another Encounter this encounter is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Reference>,
    /// The organization (facility) responsible for this encounter
    #[serde(rename = "serviceProvider")]
    pub service_provider: Option<Reference>,
    /// List of participants involved in the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<EncounterParticipant>,
    /// The appointment that scheduled this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appointment: Vec<Reference>,
    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<VirtualServiceDetail>,
    /// The actual start and end time of the encounter
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
    /// The list of medical reasons that are expected to be addressed during the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<EncounterReason>,
    /// The list of diagnosis relevant to this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EncounterDiagnosis>,
    /// The set of accounts that may be used for billing for this Encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<Reference>,
    /// Diet preferences reported by the patient
    ///
    /// Binding: example (Medical, cultural or ethical food preferences to help with catering requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diet
    #[serde(rename = "dietPreference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diet_preference: Vec<CodeableConcept>,
    /// Wheelchair, translator, stretcher, etc
    ///
    /// Binding: preferred (Special arrangements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-special-arrangements
    #[serde(rename = "specialArrangement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_arrangement: Vec<CodeableConcept>,
    /// Special courtesies (VIP, board member)
    ///
    /// Binding: preferred (Special courtesies.)
    ///
    /// Available values:
    /// - `EXT`
    /// - `NRM`
    /// - `PRF`
    /// - `STF`
    /// - `VIP`
    /// - `UNK`
    #[serde(rename = "specialCourtesy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_courtesy: Vec<CodeableConcept>,
    /// Details about the admission to a healthcare service
    pub admission: Option<EncounterAdmission>,
    /// List of locations where the patient has been
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EncounterLocation>,
}
/// Encounter nested structure for the 'admission' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterAdmission {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Pre-admission identifier
    #[serde(rename = "preAdmissionIdentifier")]
    pub pre_admission_identifier: Option<Identifier>,
    /// The location/organization from which the patient came before admission
    pub origin: Option<Reference>,
    /// From where patient was admitted (physician referral, transfer)
    ///
    /// Binding: preferred (From where the patient was admitted.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-admit-source
    #[serde(rename = "admitSource")]
    pub admit_source: Option<CodeableConcept>,
    /// Indicates that the patient is being re-admitted
    ///
    /// Binding: example (The reason for re-admission of this admission encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0092
    #[serde(rename = "reAdmission")]
    pub re_admission: Option<CodeableConcept>,
    /// Location/organization to which the patient is discharged
    pub destination: Option<Reference>,
    /// Category or kind of location after discharge
    ///
    /// Binding: example (Discharge Disposition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-discharge-disposition
    #[serde(rename = "dischargeDisposition")]
    pub discharge_disposition: Option<CodeableConcept>,
}
/// Encounter nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The diagnosis relevant to the encounter
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<CodeableReference>,
    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    ///
    /// Binding: preferred (The type of diagnosis this condition represents.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diagnosis-use
    #[serde(rename = "use")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_: Vec<CodeableConcept>,
}
/// Encounter nested structure for the 'location' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Location the encounter takes place
    pub location: Reference,
    /// planned | active | reserved | completed
    pub status: Option<EncounterLocationStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    ///
    /// Binding: example (Physical form of the location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-form
    pub form: Option<CodeableConcept>,
    /// Time period during which the patient was present at the location
    pub period: Option<Period>,
}
/// Encounter nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Role of participant in encounter
    ///
    /// Binding: extensible (Role of participant in encounter.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-participant-type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_: Vec<CodeableConcept>,
    /// Period of time during the encounter that the participant participated
    pub period: Option<Period>,
    /// The individual, device, or service participating in the encounter
    pub actor: Option<Reference>,
}
/// Encounter nested structure for the 'reason' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterReason {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What the reason value should be used for/as
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-reason-use
    #[serde(rename = "use")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_: Vec<CodeableConcept>,
    /// Reason the encounter takes place (core or reference)
    ///
    /// Binding: preferred (Reason why the encounter takes place.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CodeableReference>,
}

impl Default for Encounter {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: EncounterStatus::default(),
            _status: Default::default(),
            class: Default::default(),
            priority: Default::default(),
            type_: Default::default(),
            service_type: Default::default(),
            subject: Default::default(),
            subject_status: Default::default(),
            episode_of_care: Default::default(),
            based_on: Default::default(),
            care_team: Default::default(),
            part_of: Default::default(),
            service_provider: Default::default(),
            participant: Default::default(),
            appointment: Default::default(),
            virtual_service: Default::default(),
            actual_period: Default::default(),
            planned_start_date: Default::default(),
            _planned_start_date: Default::default(),
            planned_end_date: Default::default(),
            _planned_end_date: Default::default(),
            length: Default::default(),
            reason: Default::default(),
            diagnosis: Default::default(),
            account: Default::default(),
            diet_preference: Default::default(),
            special_arrangement: Default::default(),
            special_courtesy: Default::default(),
            admission: Default::default(),
            location: Default::default(),
        }
    }
}

impl Default for EncounterAdmission {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            pre_admission_identifier: Default::default(),
            origin: Default::default(),
            admit_source: Default::default(),
            re_admission: Default::default(),
            destination: Default::default(),
            discharge_disposition: Default::default(),
        }
    }
}

impl Default for EncounterDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            condition: Default::default(),
            use_: Default::default(),
        }
    }
}

impl Default for EncounterLocation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            location: Reference::default(),
            status: Default::default(),
            _status: Default::default(),
            form: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for EncounterParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            period: Default::default(),
            actor: Default::default(),
        }
    }
}

impl Default for EncounterReason {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            use_: Default::default(),
            value: Default::default(),
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
    rh_foundation::Invariant::new("enc-1", rh_foundation::Severity::Error, "A type must be provided when no explicit actor is specified", "actor.exists() or type.exists()"),
    rh_foundation::Invariant::new("enc-2", rh_foundation::Severity::Error, "A type cannot be provided for a patient or group participant", "actor.exists(resolve() is Patient or resolve() is Group) implies type.exists().not()"),
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
                "Encounter.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Encounter.location.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/encounter-location-status|5.0.0",
            )
            .with_description("The status of the location."),
            rh_foundation::ElementBinding::new(
                "Encounter.status",
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
            rh_foundation::ElementCardinality::new("Encounter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.contained", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.extension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.class", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.type", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.serviceType", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.subjectStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.episodeOfCare", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.careTeam", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.partOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.serviceProvider", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.participant", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Encounter.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Encounter.participant.type", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.participant.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.participant.actor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.appointment", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.virtualService", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.actualPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.plannedStartDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.plannedEndDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.length", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.reason", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.reason.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.reason.extension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.reason.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.reason.use", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.reason.value", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.diagnosis", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.diagnosis.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.diagnosis.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Encounter.diagnosis.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Encounter.diagnosis.condition", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.diagnosis.use", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.account", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.dietPreference", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.specialArrangement", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.specialCourtesy", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.admission", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.admission.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.admission.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Encounter.admission.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Encounter.admission.preAdmissionIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Encounter.admission.origin", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.admission.admitSource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.admission.reAdmission", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.admission.destination", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Encounter.admission.dischargeDisposition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Encounter.location", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.location.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.location.extension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.location.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Encounter.location.location", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.location.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.location.form", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Encounter.location.period", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Encounter {
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

impl crate::traits::resource::ResourceMutators for Encounter {
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

impl crate::traits::resource::ResourceExistence for Encounter {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Encounter {
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

impl crate::traits::domain_resource::DomainResourceMutators for Encounter {
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

impl crate::traits::domain_resource::DomainResourceExistence for Encounter {
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

impl crate::traits::encounter::EncounterAccessors for Encounter {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn status(&self) -> EncounterStatus {
        self.status.clone()
    }
    fn class(&self) -> &[CodeableConcept] {
        self.class.as_slice()
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_slice()
    }
    fn service_type(&self) -> &[CodeableReference] {
        self.service_type.as_slice()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn subject_status(&self) -> Option<CodeableConcept> {
        self.subject_status.clone()
    }
    fn episode_of_care(&self) -> &[Reference] {
        self.episode_of_care.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn care_team(&self) -> &[Reference] {
        self.care_team.as_slice()
    }
    fn part_of(&self) -> Option<Reference> {
        self.part_of.clone()
    }
    fn service_provider(&self) -> Option<Reference> {
        self.service_provider.clone()
    }
    fn participant(&self) -> &[EncounterParticipant] {
        self.participant.as_slice()
    }
    fn appointment(&self) -> &[Reference] {
        self.appointment.as_slice()
    }
    fn virtual_service(&self) -> &[VirtualServiceDetail] {
        self.virtual_service.as_slice()
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
    fn reason(&self) -> &[EncounterReason] {
        self.reason.as_slice()
    }
    fn diagnosis(&self) -> &[EncounterDiagnosis] {
        self.diagnosis.as_slice()
    }
    fn account(&self) -> &[Reference] {
        self.account.as_slice()
    }
    fn diet_preference(&self) -> &[CodeableConcept] {
        self.diet_preference.as_slice()
    }
    fn special_arrangement(&self) -> &[CodeableConcept] {
        self.special_arrangement.as_slice()
    }
    fn special_courtesy(&self) -> &[CodeableConcept] {
        self.special_courtesy.as_slice()
    }
    fn admission(&self) -> Option<EncounterAdmission> {
        self.admission.clone()
    }
    fn location(&self) -> &[EncounterLocation] {
        self.location.as_slice()
    }
}

impl crate::traits::encounter::EncounterMutators for Encounter {
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
    fn set_status(self, value: EncounterStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_class(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.class = value;
        resource
    }
    fn add_class(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.class.push(item);
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_type_(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn add_type_(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_.push(item);
        resource
    }
    fn set_service_type(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.service_type = value;
        resource
    }
    fn add_service_type(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.service_type.push(item);
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
    fn set_episode_of_care(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.episode_of_care = value;
        resource
    }
    fn add_episode_of_care(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.episode_of_care.push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
        resource
    }
    fn set_care_team(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.care_team = value;
        resource
    }
    fn add_care_team(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.care_team.push(item);
        resource
    }
    fn set_part_of(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn set_service_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.service_provider = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<EncounterParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = value;
        resource
    }
    fn add_participant(self, item: EncounterParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.push(item);
        resource
    }
    fn set_appointment(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.appointment = value;
        resource
    }
    fn add_appointment(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.appointment.push(item);
        resource
    }
    fn set_virtual_service(self, value: Vec<VirtualServiceDetail>) -> Self {
        let mut resource = self.clone();
        resource.virtual_service = value;
        resource
    }
    fn add_virtual_service(self, item: VirtualServiceDetail) -> Self {
        let mut resource = self.clone();
        resource.virtual_service.push(item);
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
    fn set_reason(self, value: Vec<EncounterReason>) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn add_reason(self, item: EncounterReason) -> Self {
        let mut resource = self.clone();
        resource.reason.push(item);
        resource
    }
    fn set_diagnosis(self, value: Vec<EncounterDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = value;
        resource
    }
    fn add_diagnosis(self, item: EncounterDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.push(item);
        resource
    }
    fn set_account(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.account = value;
        resource
    }
    fn add_account(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.account.push(item);
        resource
    }
    fn set_diet_preference(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.diet_preference = value;
        resource
    }
    fn add_diet_preference(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.diet_preference.push(item);
        resource
    }
    fn set_special_arrangement(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.special_arrangement = value;
        resource
    }
    fn add_special_arrangement(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.special_arrangement.push(item);
        resource
    }
    fn set_special_courtesy(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.special_courtesy = value;
        resource
    }
    fn add_special_courtesy(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.special_courtesy.push(item);
        resource
    }
    fn set_admission(self, value: EncounterAdmission) -> Self {
        let mut resource = self.clone();
        resource.admission = Some(value);
        resource
    }
    fn set_location(self, value: Vec<EncounterLocation>) -> Self {
        let mut resource = self.clone();
        resource.location = value;
        resource
    }
    fn add_location(self, item: EncounterLocation) -> Self {
        let mut resource = self.clone();
        resource.location.push(item);
        resource
    }
}

impl crate::traits::encounter::EncounterExistence for Encounter {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_class(&self) -> bool {
        !self.class.is_empty()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_type_(&self) -> bool {
        !self.type_.is_empty()
    }
    fn has_service_type(&self) -> bool {
        !self.service_type.is_empty()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_subject_status(&self) -> bool {
        self.subject_status.is_some()
    }
    fn has_episode_of_care(&self) -> bool {
        !self.episode_of_care.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_care_team(&self) -> bool {
        !self.care_team.is_empty()
    }
    fn has_part_of(&self) -> bool {
        self.part_of.is_some()
    }
    fn has_service_provider(&self) -> bool {
        self.service_provider.is_some()
    }
    fn has_participant(&self) -> bool {
        !self.participant.is_empty()
    }
    fn has_appointment(&self) -> bool {
        !self.appointment.is_empty()
    }
    fn has_virtual_service(&self) -> bool {
        !self.virtual_service.is_empty()
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
    fn has_reason(&self) -> bool {
        !self.reason.is_empty()
    }
    fn has_diagnosis(&self) -> bool {
        !self.diagnosis.is_empty()
    }
    fn has_account(&self) -> bool {
        !self.account.is_empty()
    }
    fn has_diet_preference(&self) -> bool {
        !self.diet_preference.is_empty()
    }
    fn has_special_arrangement(&self) -> bool {
        !self.special_arrangement.is_empty()
    }
    fn has_special_courtesy(&self) -> bool {
        !self.special_courtesy.is_empty()
    }
    fn has_admission(&self) -> bool {
        self.admission.is_some()
    }
    fn has_location(&self) -> bool {
        !self.location.is_empty()
    }
}

impl crate::validation::ValidatableResource for Encounter {
    fn resource_type(&self) -> &'static str {
        "Encounter"
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
        Some("http://hl7.org/fhir/StructureDefinition/Encounter")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::encounter::{EncounterAccessors, EncounterExistence, EncounterMutators};
