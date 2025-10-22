use crate::bindings::encounter_location_status::EncounterLocationStatus;
use crate::bindings::encounter_status::EncounterStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::positive_int::PositiveIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Encounter
///
/// An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Encounter
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Encounter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<Identifier>>,
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    pub status: EncounterStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// List of past encounter statuses
    #[serde(rename = "statusHistory")]
    pub status_history: Option<Vec<EncounterStatushistory>>,
    /// Classification of patient encounter
    ///
    /// Binding: extensible (Classification of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActEncounterCode
    pub class: Coding,
    /// List of past encounter classes
    #[serde(rename = "classHistory")]
    pub class_history: Option<Vec<EncounterClasshistory>>,
    /// Specific type of encounter
    ///
    /// Binding: example (The type of encounter.)
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
    pub service_type: Option<CodeableConcept>,
    /// Indicates the urgency of the encounter
    ///
    /// Binding: example (Indicates the urgency of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActPriority
    pub priority: Option<CodeableConcept>,
    /// The patient or group present at the encounter
    pub subject: Option<Reference>,
    /// Episode(s) of care that this encounter should be recorded against
    #[serde(rename = "episodeOfCare")]
    pub episode_of_care: Option<Vec<Reference>>,
    /// The ServiceRequest that initiated this encounter
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// List of participants involved in the encounter
    pub participant: Option<Vec<EncounterParticipant>>,
    /// The appointment that scheduled this encounter
    pub appointment: Option<Vec<Reference>>,
    /// The start and end time of the encounter
    pub period: Option<Period>,
    /// Quantity of time the encounter lasted (less time absent)
    pub length: Option<Duration>,
    /// Coded reason the encounter takes place
    ///
    /// Binding: preferred (Reason why the encounter takes place.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-reason
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Reason the encounter takes place (reference)
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// The list of diagnosis relevant to this encounter
    pub diagnosis: Option<Vec<EncounterDiagnosis>>,
    /// The set of accounts that may be used for billing for this Encounter
    pub account: Option<Vec<Reference>>,
    /// Details about the admission to a healthcare service
    pub hospitalization: Option<EncounterHospitalization>,
    /// List of locations where the patient has been
    pub location: Option<Vec<EncounterLocation>>,
    /// The organization (facility) responsible for this encounter
    #[serde(rename = "serviceProvider")]
    pub service_provider: Option<Reference>,
    /// Another Encounter this encounter is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Reference>,
}
/// Encounter nested structure for the 'classHistory' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterClasshistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// inpatient | outpatient | ambulatory | emergency +
    ///
    /// Binding: extensible (Classification of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActEncounterCode
    pub class: Coding,
    /// The time that the episode was in the specified class
    pub period: Period,
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
    /// The physical type of the location (usually the level in the location hierachy - bed room ward etc.)
    ///
    /// Binding: example (Physical form of the location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-physical-type
    #[serde(rename = "physicalType")]
    pub physical_type: Option<CodeableConcept>,
    /// Time period during which the patient was present at the location
    pub period: Option<Period>,
}
/// Encounter nested structure for the 'hospitalization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterHospitalization {
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
    /// The type of hospital re-admission that has occurred (if any). If the value is absent, then this is not identified as a readmission
    ///
    /// Binding: example (The reason for re-admission of this hospitalization encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0092
    #[serde(rename = "reAdmission")]
    pub re_admission: Option<CodeableConcept>,
    /// Diet preferences reported by the patient
    ///
    /// Binding: example (Medical, cultural or ethical food preferences to help with catering requirements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-diet
    #[serde(rename = "dietPreference")]
    pub diet_preference: Option<Vec<CodeableConcept>>,
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
    pub special_courtesy: Option<Vec<CodeableConcept>>,
    /// Wheelchair, translator, stretcher, etc.
    ///
    /// Binding: preferred (Special arrangements.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-special-arrangements
    #[serde(rename = "specialArrangement")]
    pub special_arrangement: Option<Vec<CodeableConcept>>,
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
/// Encounter nested structure for the 'statusHistory' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterStatushistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    pub status: EncounterStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The time that the episode was in the specified status
    pub period: Period,
}
/// Encounter nested structure for the 'diagnosis' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterDiagnosis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The diagnosis or procedure relevant to the encounter
    pub condition: Reference,
    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    ///
    /// Binding: preferred (The type of diagnosis this condition represents.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/diagnosis-role
    #[serde(rename = "use")]
    pub use_: Option<CodeableConcept>,
    /// Ranking of the diagnosis (for each role type)
    pub rank: Option<PositiveIntType>,
    /// Extension element for the 'rank' primitive field. Contains metadata and extensions.
    pub _rank: Option<Element>,
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
    /// Available values:
    /// - `SPRF`
    /// - `PPRF`
    /// - `PART`
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Period of time during the encounter that the participant participated
    pub period: Option<Period>,
    /// Persons involved in the encounter other than the patient
    pub individual: Option<Reference>,
}

impl Default for Encounter {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: EncounterStatus::default(),
            _status: Default::default(),
            status_history: Default::default(),
            class: Coding::default(),
            class_history: Default::default(),
            type_: Default::default(),
            service_type: Default::default(),
            priority: Default::default(),
            subject: Default::default(),
            episode_of_care: Default::default(),
            based_on: Default::default(),
            participant: Default::default(),
            appointment: Default::default(),
            period: Default::default(),
            length: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            diagnosis: Default::default(),
            account: Default::default(),
            hospitalization: Default::default(),
            location: Default::default(),
            service_provider: Default::default(),
            part_of: Default::default(),
        }
    }
}

impl Default for EncounterClasshistory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            class: Default::default(),
            period: Default::default(),
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
            physical_type: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for EncounterHospitalization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            pre_admission_identifier: Default::default(),
            origin: Default::default(),
            admit_source: Default::default(),
            re_admission: Default::default(),
            diet_preference: Default::default(),
            special_courtesy: Default::default(),
            special_arrangement: Default::default(),
            destination: Default::default(),
            discharge_disposition: Default::default(),
        }
    }
}

impl Default for EncounterStatushistory {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            status: Default::default(),
            _status: Default::default(),
            period: Default::default(),
        }
    }
}

impl Default for EncounterDiagnosis {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            condition: Reference::default(),
            use_: Default::default(),
            rank: Default::default(),
            _rank: Default::default(),
        }
    }
}

impl Default for EncounterParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            period: Default::default(),
            individual: Default::default(),
        }
    }
}

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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for Encounter {
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

impl crate::traits::encounter::EncounterAccessors for Encounter {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> EncounterStatus {
        self.status.clone()
    }
    fn status_history(&self) -> &[EncounterStatushistory] {
        self.status_history.as_deref().unwrap_or(&[])
    }
    fn class(&self) -> Coding {
        self.class.clone()
    }
    fn class_history(&self) -> &[EncounterClasshistory] {
        self.class_history.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn service_type(&self) -> Option<CodeableConcept> {
        self.service_type.clone()
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn episode_of_care(&self) -> &[Reference] {
        self.episode_of_care.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn participant(&self) -> &[EncounterParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn appointment(&self) -> &[Reference] {
        self.appointment.as_deref().unwrap_or(&[])
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn length(&self) -> Option<Duration> {
        self.length.clone()
    }
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn diagnosis(&self) -> &[EncounterDiagnosis] {
        self.diagnosis.as_deref().unwrap_or(&[])
    }
    fn account(&self) -> &[Reference] {
        self.account.as_deref().unwrap_or(&[])
    }
    fn hospitalization(&self) -> Option<EncounterHospitalization> {
        self.hospitalization.clone()
    }
    fn location(&self) -> &[EncounterLocation] {
        self.location.as_deref().unwrap_or(&[])
    }
    fn service_provider(&self) -> Option<Reference> {
        self.service_provider.clone()
    }
    fn part_of(&self) -> Option<Reference> {
        self.part_of.clone()
    }
}

impl crate::traits::encounter::EncounterMutators for Encounter {
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
    fn set_status(self, value: EncounterStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_history(self, value: Vec<EncounterStatushistory>) -> Self {
        let mut resource = self.clone();
        resource.status_history = Some(value);
        resource
    }
    fn add_status_history(self, item: EncounterStatushistory) -> Self {
        let mut resource = self.clone();
        resource
            .status_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_class(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.class = value;
        resource
    }
    fn set_class_history(self, value: Vec<EncounterClasshistory>) -> Self {
        let mut resource = self.clone();
        resource.class_history = Some(value);
        resource
    }
    fn add_class_history(self, item: EncounterClasshistory) -> Self {
        let mut resource = self.clone();
        resource
            .class_history
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_service_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.service_type = Some(value);
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_episode_of_care(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.episode_of_care = Some(value);
        resource
    }
    fn add_episode_of_care(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .episode_of_care
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_participant(self, value: Vec<EncounterParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: EncounterParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_appointment(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.appointment = Some(value);
        resource
    }
    fn add_appointment(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.appointment.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_length(self, value: Duration) -> Self {
        let mut resource = self.clone();
        resource.length = Some(value);
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
    fn set_diagnosis(self, value: Vec<EncounterDiagnosis>) -> Self {
        let mut resource = self.clone();
        resource.diagnosis = Some(value);
        resource
    }
    fn add_diagnosis(self, item: EncounterDiagnosis) -> Self {
        let mut resource = self.clone();
        resource.diagnosis.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_account(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.account = Some(value);
        resource
    }
    fn add_account(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.account.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_hospitalization(self, value: EncounterHospitalization) -> Self {
        let mut resource = self.clone();
        resource.hospitalization = Some(value);
        resource
    }
    fn set_location(self, value: Vec<EncounterLocation>) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn add_location(self, item: EncounterLocation) -> Self {
        let mut resource = self.clone();
        resource.location.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_service_provider(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.service_provider = Some(value);
        resource
    }
    fn set_part_of(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
}

impl crate::traits::encounter::EncounterExistence for Encounter {
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
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_history(&self) -> bool {
        self.status_history.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_class(&self) -> bool {
        true
    }
    fn has_class_history(&self) -> bool {
        self.class_history.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_type(&self) -> bool {
        self.service_type.is_some()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_episode_of_care(&self) -> bool {
        self.episode_of_care.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_appointment(&self) -> bool {
        self.appointment.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_length(&self) -> bool {
        self.length.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_diagnosis(&self) -> bool {
        self.diagnosis.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_account(&self) -> bool {
        self.account.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_hospitalization(&self) -> bool {
        self.hospitalization.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_provider(&self) -> bool {
        self.service_provider.is_some()
    }
    fn has_part_of(&self) -> bool {
        self.part_of.is_some()
    }
}
