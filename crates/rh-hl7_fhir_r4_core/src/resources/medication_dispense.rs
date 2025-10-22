use crate::bindings::medicationdispense_status::MedicationdispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationDispense
///
/// Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicationDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationDispense {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Event that dispense is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    pub status: MedicationdispenseStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Why a dispense was not performed (CodeableConcept)
    #[serde(rename = "statusReasonCodeableConcept")]
    pub status_reason_codeable_concept: Option<CodeableConcept>,
    /// Why a dispense was not performed (Reference)
    #[serde(rename = "statusReasonReference")]
    pub status_reason_reference: Option<Reference>,
    /// Type of medication dispense
    ///
    /// Binding: preferred (A code describing where the dispensed medication is expected to be consumed or administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationdispense-category
    pub category: Option<CodeableConcept>,
    /// What medication was supplied (CodeableConcept)
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,
    /// What medication was supplied (Reference)
    #[serde(rename = "medicationReference")]
    pub medication_reference: Reference,
    /// Who the dispense is for
    pub subject: Option<Reference>,
    /// Encounter / Episode associated with event
    pub context: Option<Reference>,
    /// Information that supports the dispensing of the medication
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Who performed event
    pub performer: Option<Vec<MedicationDispensePerformer>>,
    /// Where the dispense occurred
    pub location: Option<Reference>,
    /// Medication order that authorizes the dispense
    #[serde(rename = "authorizingPrescription")]
    pub authorizing_prescription: Option<Vec<Reference>>,
    /// Trial fill, partial fill, emergency fill, etc.
    ///
    /// Binding: example (Indicates the type of dispensing event that is performed. For example, Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActPharmacySupplyType
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Amount dispensed
    pub quantity: Option<Quantity>,
    /// Amount of medication expressed as a timing amount
    #[serde(rename = "daysSupply")]
    pub days_supply: Option<Quantity>,
    /// When product was packaged and reviewed
    #[serde(rename = "whenPrepared")]
    pub when_prepared: Option<DateTimeType>,
    /// Extension element for the 'whenPrepared' primitive field. Contains metadata and extensions.
    #[serde(rename = "_whenPrepared")]
    pub _when_prepared: Option<Element>,
    /// When product was given out
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<DateTimeType>,
    /// Extension element for the 'whenHandedOver' primitive field. Contains metadata and extensions.
    #[serde(rename = "_whenHandedOver")]
    pub _when_handed_over: Option<Element>,
    /// Where the medication was sent
    pub destination: Option<Reference>,
    /// Who collected the medication
    pub receiver: Option<Vec<Reference>>,
    /// Information about the dispense
    pub note: Option<Vec<Annotation>>,
    /// How the medication is to be used by the patient or administered by the caregiver
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// Whether a substitution was performed on the dispense
    pub substitution: Option<MedicationDispenseSubstitution>,
    /// Clinical issue with action
    #[serde(rename = "detectedIssue")]
    pub detected_issue: Option<Vec<Reference>>,
    /// A list of relevant lifecycle events
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Reference>>,
}
/// MedicationDispense nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationDispensePerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who performed the dispense and what they did
    ///
    /// Binding: example (A code describing the role an individual played in dispensing a medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationdispense-performer-function
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    pub actor: Reference,
}
/// MedicationDispense nested structure for the 'substitution' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationDispenseSubstitution {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether a substitution was or was not performed on the dispense
    #[serde(rename = "wasSubstituted")]
    pub was_substituted: BooleanType,
    /// Extension element for the 'wasSubstituted' primitive field. Contains metadata and extensions.
    #[serde(rename = "_wasSubstituted")]
    pub _was_substituted: Option<Element>,
    /// Code signifying whether a different drug was dispensed from what was prescribed
    ///
    /// Binding: example (A coded concept describing whether a different medicinal product may be dispensed other than the product as specified exactly in the prescription.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActSubstanceAdminSubstitutionCode
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Why was substitution made
    ///
    /// Binding: example (A coded concept describing the reason that a different medication should (or should not) be substituted from what was prescribed.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-SubstanceAdminSubstitutionReason
    pub reason: Option<Vec<CodeableConcept>>,
    /// Who is responsible for the substitution
    #[serde(rename = "responsibleParty")]
    pub responsible_party: Option<Vec<Reference>>,
}

impl Default for MedicationDispense {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            part_of: Default::default(),
            status: MedicationdispenseStatus::default(),
            _status: Default::default(),
            status_reason_codeable_concept: Default::default(),
            status_reason_reference: Default::default(),
            category: Default::default(),
            medication_codeable_concept: Default::default(),
            medication_reference: Default::default(),
            subject: Default::default(),
            context: Default::default(),
            supporting_information: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            authorizing_prescription: Default::default(),
            type_: Default::default(),
            quantity: Default::default(),
            days_supply: Default::default(),
            when_prepared: Default::default(),
            _when_prepared: Default::default(),
            when_handed_over: Default::default(),
            _when_handed_over: Default::default(),
            destination: Default::default(),
            receiver: Default::default(),
            note: Default::default(),
            dosage_instruction: Default::default(),
            substitution: Default::default(),
            detected_issue: Default::default(),
            event_history: Default::default(),
        }
    }
}

impl Default for MedicationDispensePerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
        }
    }
}

impl Default for MedicationDispenseSubstitution {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            was_substituted: BooleanType::default(),
            _was_substituted: Default::default(),
            type_: Default::default(),
            reason: Default::default(),
            responsible_party: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationDispense {
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

impl crate::traits::resource::ResourceMutators for MedicationDispense {
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

impl crate::traits::resource::ResourceExistence for MedicationDispense {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationDispense {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationDispense {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationDispense {
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

impl crate::traits::medication_dispense::MedicationDispenseAccessors for MedicationDispense {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MedicationdispenseStatus {
        self.status.clone()
    }
    fn category(&self) -> Option<CodeableConcept> {
        self.category.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn context(&self) -> Option<Reference> {
        self.context.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn performer(&self) -> &[MedicationDispensePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn authorizing_prescription(&self) -> &[Reference] {
        self.authorizing_prescription.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn days_supply(&self) -> Option<Quantity> {
        self.days_supply.clone()
    }
    fn when_prepared(&self) -> Option<DateTimeType> {
        self.when_prepared.clone()
    }
    fn when_handed_over(&self) -> Option<DateTimeType> {
        self.when_handed_over.clone()
    }
    fn destination(&self) -> Option<Reference> {
        self.destination.clone()
    }
    fn receiver(&self) -> &[Reference] {
        self.receiver.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn dosage_instruction(&self) -> &[Dosage] {
        self.dosage_instruction.as_deref().unwrap_or(&[])
    }
    fn substitution(&self) -> Option<MedicationDispenseSubstitution> {
        self.substitution.clone()
    }
    fn detected_issue(&self) -> &[Reference] {
        self.detected_issue.as_deref().unwrap_or(&[])
    }
    fn event_history(&self) -> &[Reference] {
        self.event_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::medication_dispense::MedicationDispenseMutators for MedicationDispense {
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
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: MedicationdispenseStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_category(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_context(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = Some(value);
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_information
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_performer(self, value: Vec<MedicationDispensePerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: MedicationDispensePerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_authorizing_prescription(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.authorizing_prescription = Some(value);
        resource
    }
    fn add_authorizing_prescription(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .authorizing_prescription
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_days_supply(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.days_supply = Some(value);
        resource
    }
    fn set_when_prepared(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.when_prepared = Some(value);
        resource
    }
    fn set_when_handed_over(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.when_handed_over = Some(value);
        resource
    }
    fn set_destination(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.destination = Some(value);
        resource
    }
    fn set_receiver(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.receiver = Some(value);
        resource
    }
    fn add_receiver(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.receiver.get_or_insert_with(Vec::new).push(item);
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
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self {
        let mut resource = self.clone();
        resource.dosage_instruction = Some(value);
        resource
    }
    fn add_dosage_instruction(self, item: Dosage) -> Self {
        let mut resource = self.clone();
        resource
            .dosage_instruction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_substitution(self, value: MedicationDispenseSubstitution) -> Self {
        let mut resource = self.clone();
        resource.substitution = Some(value);
        resource
    }
    fn set_detected_issue(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.detected_issue = Some(value);
        resource
    }
    fn add_detected_issue(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .detected_issue
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_event_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.event_history = Some(value);
        resource
    }
    fn add_event_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .event_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::medication_dispense::MedicationDispenseExistence for MedicationDispense {
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
    fn has_status_reason(&self) -> bool {
        self.status_reason_codeable_concept.is_some() || self.status_reason_reference.is_some()
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_context(&self) -> bool {
        self.context.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_performer(&self) -> bool {
        self.performer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_authorizing_prescription(&self) -> bool {
        self.authorizing_prescription
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_days_supply(&self) -> bool {
        self.days_supply.is_some()
    }
    fn has_when_prepared(&self) -> bool {
        self.when_prepared.is_some()
    }
    fn has_when_handed_over(&self) -> bool {
        self.when_handed_over.is_some()
    }
    fn has_destination(&self) -> bool {
        self.destination.is_some()
    }
    fn has_receiver(&self) -> bool {
        self.receiver.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_dosage_instruction(&self) -> bool {
        self.dosage_instruction
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_substitution(&self) -> bool {
        self.substitution.is_some()
    }
    fn has_detected_issue(&self) -> bool {
        self.detected_issue.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_event_history(&self) -> bool {
        self.event_history.as_ref().is_some_and(|v| !v.is_empty())
    }
}
