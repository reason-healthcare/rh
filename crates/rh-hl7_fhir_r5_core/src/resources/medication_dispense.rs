use crate::bindings::medicationdispense_status::MedicationdispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationDispense
///
/// Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
/// - Version: 5.0.0
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
    /// Plan that is fulfilled by this dispense
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Event that dispense is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    pub status: MedicationdispenseStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Why a dispense was not performed
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationdispense-status-reason
    #[serde(rename = "notPerformedReason")]
    pub not_performed_reason: Option<CodeableReference>,
    /// When the status changed
    #[serde(rename = "statusChanged")]
    pub status_changed: Option<DateTimeType>,
    /// Extension element for the 'statusChanged' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusChanged")]
    pub _status_changed: Option<Element>,
    /// Type of medication dispense
    ///
    /// Binding: example (A code describing where the dispensed medication is expected to be consumed or administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationdispense-admin-location
    pub category: Option<Vec<CodeableConcept>>,
    /// What medication was supplied
    ///
    /// Binding: example (A coded concept identifying which substance or product can be dispensed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub medication: CodeableReference,
    /// Who the dispense is for
    pub subject: Reference,
    /// Encounter associated with event
    pub encounter: Option<Reference>,
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
    /// Trial fill, partial fill, emergency fill, etc
    ///
    /// Binding: example (ActPharmacySupplyType )
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActPharmacySupplyType
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Amount dispensed
    pub quantity: Option<Quantity>,
    /// Amount of medication expressed as a timing amount
    #[serde(rename = "daysSupply")]
    pub days_supply: Option<Quantity>,
    /// When the recording of the dispense started
    pub recorded: Option<DateTimeType>,
    /// Extension element for the 'recorded' primitive field. Contains metadata and extensions.
    pub _recorded: Option<Element>,
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
    /// Where the medication was/will be sent
    pub destination: Option<Reference>,
    /// Who collected the medication or where the medication was delivered
    pub receiver: Option<Vec<Reference>>,
    /// Information about the dispense
    pub note: Option<Vec<Annotation>>,
    /// Full representation of the dosage instructions
    #[serde(rename = "renderedDosageInstruction")]
    pub rendered_dosage_instruction: Option<StringType>,
    /// Extension element for the 'renderedDosageInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_renderedDosageInstruction")]
    pub _rendered_dosage_instruction: Option<Element>,
    /// How the medication is to be used by the patient or administered by the caregiver
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// Whether a substitution was performed on the dispense
    pub substitution: Option<MedicationDispenseSubstitution>,
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
    /// Binding: example (ActSubstanceAdminSubstitutionCode)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActSubstanceAdminSubstitutionCode
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Why was substitution made
    ///
    /// Binding: example (SubstanceAdminSubstitutionReason)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-SubstanceAdminSubstitutionReason
    pub reason: Option<Vec<CodeableConcept>>,
    /// Who is responsible for the substitution
    #[serde(rename = "responsibleParty")]
    pub responsible_party: Option<Reference>,
}

impl Default for MedicationDispense {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: MedicationdispenseStatus::default(),
            _status: Default::default(),
            not_performed_reason: Default::default(),
            status_changed: Default::default(),
            _status_changed: Default::default(),
            category: Default::default(),
            medication: CodeableReference::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            authorizing_prescription: Default::default(),
            type_: Default::default(),
            quantity: Default::default(),
            days_supply: Default::default(),
            recorded: Default::default(),
            _recorded: Default::default(),
            when_prepared: Default::default(),
            _when_prepared: Default::default(),
            when_handed_over: Default::default(),
            _when_handed_over: Default::default(),
            destination: Default::default(),
            receiver: Default::default(),
            note: Default::default(),
            rendered_dosage_instruction: Default::default(),
            _rendered_dosage_instruction: Default::default(),
            dosage_instruction: Default::default(),
            substitution: Default::default(),
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
    rh_foundation::Invariant::new("mdd-1", rh_foundation::Severity::Error, "whenHandedOver cannot be before whenPrepared", "whenHandedOver.empty() or whenPrepared.empty() or whenHandedOver >= whenPrepared"),
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
                "MedicationDispense.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "MedicationDispense.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/medicationdispense-status|5.0.0",
            )
            .with_description("Describes the lifecycle of the dispense."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MedicationDispense.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.contained", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.extension", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.partOf", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.notPerformedReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.statusChanged", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.category", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.medication", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.supportingInformation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.performer", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.performer.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.performer.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.performer.actor",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.authorizingPrescription",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.daysSupply", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.recorded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.whenPrepared", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.whenHandedOver", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.destination", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationDispense.receiver", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.renderedDosageInstruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.dosageInstruction", 0, None),
            rh_foundation::ElementCardinality::new("MedicationDispense.substitution", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.wasSubstituted",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.reason",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationDispense.substitution.responsibleParty",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationDispense.eventHistory", 0, None),
        ]
    });

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
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> MedicationdispenseStatus {
        self.status.clone()
    }
    fn not_performed_reason(&self) -> Option<CodeableReference> {
        self.not_performed_reason.clone()
    }
    fn status_changed(&self) -> Option<DateTimeType> {
        self.status_changed.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn medication(&self) -> CodeableReference {
        self.medication.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
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
    fn recorded(&self) -> Option<DateTimeType> {
        self.recorded.clone()
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
    fn rendered_dosage_instruction(&self) -> Option<StringType> {
        self.rendered_dosage_instruction.clone()
    }
    fn dosage_instruction(&self) -> &[Dosage] {
        self.dosage_instruction.as_deref().unwrap_or(&[])
    }
    fn substitution(&self) -> Option<MedicationDispenseSubstitution> {
        self.substitution.clone()
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
    fn set_not_performed_reason(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.not_performed_reason = Some(value);
        resource
    }
    fn set_status_changed(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_changed = Some(value);
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
    fn set_medication(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.medication = value;
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
    fn set_recorded(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.recorded = Some(value);
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
    fn set_rendered_dosage_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.rendered_dosage_instruction = Some(value);
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_not_performed_reason(&self) -> bool {
        self.not_performed_reason.is_some()
    }
    fn has_status_changed(&self) -> bool {
        self.status_changed.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
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
    fn has_recorded(&self) -> bool {
        self.recorded.is_some()
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
    fn has_rendered_dosage_instruction(&self) -> bool {
        self.rendered_dosage_instruction.is_some()
    }
    fn has_dosage_instruction(&self) -> bool {
        self.dosage_instruction
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_substitution(&self) -> bool {
        self.substitution.is_some()
    }
    fn has_event_history(&self) -> bool {
        self.event_history.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for MedicationDispense {
    fn resource_type(&self) -> &'static str {
        "MedicationDispense"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicationDispense")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication_dispense::{
    MedicationDispenseAccessors, MedicationDispenseExistence, MedicationDispenseMutators,
};
