use crate::bindings::medicationrequest_intent::MedicationrequestIntent;
use crate::bindings::medicationrequest_status::MedicationrequestStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicationRequest
///
/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: MedicationRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External ids for this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// A plan or request that is fulfilled in whole or in part by this medication request
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// Reference to an order/prescription that is being replaced by this MedicationRequest
    #[serde(rename = "priorPrescription")]
    pub prior_prescription: Option<Reference>,
    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// active | on-hold | ended | stopped | completed | cancelled | entered-in-error | draft | unknown
    pub status: MedicationrequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Identifies the reasons for a given status.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-status-reason
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// When the status was changed
    #[serde(rename = "statusChanged")]
    pub status_changed: Option<DateTimeType>,
    /// Extension element for the 'statusChanged' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusChanged")]
    pub _status_changed: Option<Element>,
    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: MedicationrequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Grouping or category of medication request
    ///
    /// Binding: example (A coded concept identifying where the medication is to be consumed or administered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-admin-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// True if patient is to stop taking or not to start taking the medication
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// Medication to be taken
    ///
    /// Binding: example (A coded concept identifying substance or product that can be ordered.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-codes
    pub medication: CodeableReference,
    /// Individual or group for whom the medication has been requested
    pub subject: Reference,
    /// The person or organization who provided the information about this request, if the source is someone other than the requestor
    #[serde(rename = "informationSource")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_source: Vec<Reference>,
    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<Reference>,
    /// Information to support fulfilling of the medication
    #[serde(rename = "supportingInformation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<Reference>,
    /// When request was initially authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Who/What requested the Request
    pub requester: Option<Reference>,
    /// Reported rather than primary record
    pub reported: Option<BooleanType>,
    /// Extension element for the 'reported' primitive field. Contains metadata and extensions.
    pub _reported: Option<Element>,
    /// Desired kind of performer of the medication administration
    ///
    /// Binding: extensible (Identifies the type of individual that is desired to administer the medication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-intended-performer-role
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,
    /// Intended performer of administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<Reference>,
    /// Intended type of device for the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<CodeableReference>,
    /// Person who entered the request
    pub recorder: Option<Reference>,
    /// Reason or indication for ordering or not ordering the medication
    ///
    /// Binding: example (A coded concept indicating why the medication was ordered.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<CodeableReference>,
    /// Overall pattern of medication administration
    ///
    /// Binding: extensible (Identifies the overall pattern of medication administratio.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medicationrequest-course-of-therapy
    #[serde(rename = "courseOfTherapyType")]
    pub course_of_therapy_type: Option<CodeableConcept>,
    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<Reference>,
    /// Information about the prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Annotation>,
    /// Full representation of the dosage instructions
    #[serde(rename = "renderedDosageInstruction")]
    pub rendered_dosage_instruction: Option<StringType>,
    /// Extension element for the 'renderedDosageInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_renderedDosageInstruction")]
    pub _rendered_dosage_instruction: Option<Element>,
    /// Period over which the medication is to be taken
    #[serde(rename = "effectiveDosePeriod")]
    pub effective_dose_period: Option<Period>,
    /// Specific instructions for how the medication should be taken
    #[serde(rename = "dosageInstruction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<Dosage>,
    /// Medication supply authorization
    #[serde(rename = "dispenseRequest")]
    pub dispense_request: Option<MedicationRequestDispenserequest>,
    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,
    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<Reference>,
}
/// MedicationRequest nested structure for the 'dispenseRequest' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestDispenserequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// First fill details
    #[serde(rename = "initialFill")]
    pub initial_fill: Option<MedicationRequestDispenserequestInitialfill>,
    /// Minimum period of time between dispenses
    #[serde(rename = "dispenseInterval")]
    pub dispense_interval: Option<Duration>,
    /// Time period supply is authorized for
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
    /// Number of refills authorized
    #[serde(rename = "numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed: Option<UnsignedIntType>,
    /// Extension element for the 'numberOfRepeatsAllowed' primitive field. Contains metadata and extensions.
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub _number_of_repeats_allowed: Option<Element>,
    /// Amount of medication to supply per dispense
    pub quantity: Option<Quantity>,
    /// Number of days supply per dispense
    #[serde(rename = "expectedSupplyDuration")]
    pub expected_supply_duration: Option<Duration>,
    /// Intended performer of dispense
    pub dispenser: Option<Reference>,
    /// Additional information for the dispenser
    #[serde(rename = "dispenserInstruction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dispenser_instruction: Vec<Annotation>,
    /// Type of adherence packaging to use for the dispense
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/medication-dose-aid
    #[serde(rename = "doseAdministrationAid")]
    pub dose_administration_aid: Option<CodeableConcept>,
}
/// MedicationRequest nested structure for the 'substitution' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestSubstitution {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether substitution is allowed or not (boolean)
    #[serde(rename = "allowedBoolean")]
    pub allowed_boolean: BooleanType,
    /// Whether substitution is allowed or not (CodeableConcept)
    #[serde(rename = "allowedCodeableConcept")]
    pub allowed_codeable_concept: CodeableConcept,
    /// Why should (not) substitution be made
    ///
    /// Binding: example (SubstanceAdminSubstitutionReason)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-SubstanceAdminSubstitutionReason
    pub reason: Option<CodeableConcept>,
}
/// MedicationRequestDispenserequest nested structure for the 'initialFill' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequestDispenserequestInitialfill {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// First fill quantity
    pub quantity: Option<Quantity>,
    /// First fill duration
    pub duration: Option<Duration>,
}

impl Default for MedicationRequest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            prior_prescription: Default::default(),
            group_identifier: Default::default(),
            status: MedicationrequestStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            status_changed: Default::default(),
            _status_changed: Default::default(),
            intent: MedicationrequestIntent::default(),
            _intent: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            medication: CodeableReference::default(),
            subject: Reference::default(),
            information_source: Default::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            requester: Default::default(),
            reported: Default::default(),
            _reported: Default::default(),
            performer_type: Default::default(),
            performer: Default::default(),
            device: Default::default(),
            recorder: Default::default(),
            reason: Default::default(),
            course_of_therapy_type: Default::default(),
            insurance: Default::default(),
            note: Default::default(),
            rendered_dosage_instruction: Default::default(),
            _rendered_dosage_instruction: Default::default(),
            effective_dose_period: Default::default(),
            dosage_instruction: Default::default(),
            dispense_request: Default::default(),
            substitution: Default::default(),
            event_history: Default::default(),
        }
    }
}

impl Default for MedicationRequestDispenserequest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            initial_fill: Default::default(),
            dispense_interval: Default::default(),
            validity_period: Default::default(),
            number_of_repeats_allowed: Default::default(),
            _number_of_repeats_allowed: Default::default(),
            quantity: Default::default(),
            expected_supply_duration: Default::default(),
            dispenser: Default::default(),
            dispenser_instruction: Default::default(),
            dose_administration_aid: Default::default(),
        }
    }
}

impl Default for MedicationRequestSubstitution {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            allowed_boolean: Default::default(),
            allowed_codeable_concept: Default::default(),
            reason: Default::default(),
        }
    }
}

impl Default for MedicationRequestDispenserequestInitialfill {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            duration: Default::default(),
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
    rh_foundation::ElementBinding::new("MedicationRequest.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/medicationrequest-intent|5.0.0").with_description("The kind of medication order."),
    rh_foundation::ElementBinding::new("MedicationRequest.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("MedicationRequest.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("MedicationRequest.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/medicationrequest-status|5.0.0").with_description("A coded concept specifying the state of the prescribing event. Describes the lifecycle of the prescription."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MedicationRequest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.contained", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.extension", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.identifier", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.basedOn", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.priorPrescription",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.groupIdentifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.statusChanged", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.category", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.doNotPerform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.medication", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.informationSource", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.supportingInformation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.reported", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.performerType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.performer", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.device", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.recorder", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.reason", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.courseOfTherapyType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.insurance", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.note", 0, None),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.renderedDosageInstruction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.effectiveDosePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.dosageInstruction", 0, None),
            rh_foundation::ElementCardinality::new("MedicationRequest.dispenseRequest", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.initialFill.duration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.dispenseInterval",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.validityPeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.numberOfRepeatsAllowed",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.expectedSupplyDuration",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.dispenser",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.dispenserInstruction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.dispenseRequest.doseAdministrationAid",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.substitution", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MedicationRequest.substitution.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.substitution.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.substitution.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.substitution.allowed[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "MedicationRequest.substitution.reason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("MedicationRequest.eventHistory", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicationRequest {
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

impl crate::traits::resource::ResourceMutators for MedicationRequest {
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

impl crate::traits::resource::ResourceExistence for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicationRequest {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicationRequest {
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

impl crate::traits::medication_request::MedicationRequestAccessors for MedicationRequest {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn prior_prescription(&self) -> Option<Reference> {
        self.prior_prescription.clone()
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn status(&self) -> MedicationrequestStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn status_changed(&self) -> Option<DateTimeType> {
        self.status_changed.clone()
    }
    fn intent(&self) -> MedicationrequestIntent {
        self.intent.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn do_not_perform(&self) -> Option<BooleanType> {
        self.do_not_perform
    }
    fn medication(&self) -> CodeableReference {
        self.medication.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn information_source(&self) -> &[Reference] {
        self.information_source.as_slice()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_slice()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn reported(&self) -> Option<BooleanType> {
        self.reported
    }
    fn performer_type(&self) -> Option<CodeableConcept> {
        self.performer_type.clone()
    }
    fn performer(&self) -> &[Reference] {
        self.performer.as_slice()
    }
    fn device(&self) -> &[CodeableReference] {
        self.device.as_slice()
    }
    fn recorder(&self) -> Option<Reference> {
        self.recorder.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_slice()
    }
    fn course_of_therapy_type(&self) -> Option<CodeableConcept> {
        self.course_of_therapy_type.clone()
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_slice()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_slice()
    }
    fn rendered_dosage_instruction(&self) -> Option<StringType> {
        self.rendered_dosage_instruction.clone()
    }
    fn effective_dose_period(&self) -> Option<Period> {
        self.effective_dose_period.clone()
    }
    fn dosage_instruction(&self) -> &[Dosage] {
        self.dosage_instruction.as_slice()
    }
    fn dispense_request(&self) -> Option<MedicationRequestDispenserequest> {
        self.dispense_request.clone()
    }
    fn substitution(&self) -> Option<MedicationRequestSubstitution> {
        self.substitution.clone()
    }
    fn event_history(&self) -> &[Reference] {
        self.event_history.as_slice()
    }
}

impl crate::traits::medication_request::MedicationRequestMutators for MedicationRequest {
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
    fn set_prior_prescription(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.prior_prescription = Some(value);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_status(self, value: MedicationrequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_status_changed(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_changed = Some(value);
        resource
    }
    fn set_intent(self, value: MedicationrequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
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
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_do_not_perform(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.do_not_perform = Some(value);
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
    fn set_information_source(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.information_source = value;
        resource
    }
    fn add_information_source(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.information_source.push(item);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = value;
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.supporting_information.push(item);
        resource
    }
    fn set_authored_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored_on = Some(value);
        resource
    }
    fn set_requester(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requester = Some(value);
        resource
    }
    fn set_reported(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.reported = Some(value);
        resource
    }
    fn set_performer_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn set_performer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.performer = value;
        resource
    }
    fn add_performer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.performer.push(item);
        resource
    }
    fn set_device(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.device = value;
        resource
    }
    fn add_device(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.device.push(item);
        resource
    }
    fn set_recorder(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.recorder = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = value;
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.push(item);
        resource
    }
    fn set_course_of_therapy_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.course_of_therapy_type = Some(value);
        resource
    }
    fn set_insurance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.insurance = value;
        resource
    }
    fn add_insurance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance.push(item);
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
    fn set_rendered_dosage_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.rendered_dosage_instruction = Some(value);
        resource
    }
    fn set_effective_dose_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.effective_dose_period = Some(value);
        resource
    }
    fn set_dosage_instruction(self, value: Vec<Dosage>) -> Self {
        let mut resource = self.clone();
        resource.dosage_instruction = value;
        resource
    }
    fn add_dosage_instruction(self, item: Dosage) -> Self {
        let mut resource = self.clone();
        resource.dosage_instruction.push(item);
        resource
    }
    fn set_dispense_request(self, value: MedicationRequestDispenserequest) -> Self {
        let mut resource = self.clone();
        resource.dispense_request = Some(value);
        resource
    }
    fn set_substitution(self, value: MedicationRequestSubstitution) -> Self {
        let mut resource = self.clone();
        resource.substitution = Some(value);
        resource
    }
    fn set_event_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.event_history = value;
        resource
    }
    fn add_event_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.event_history.push(item);
        resource
    }
}

impl crate::traits::medication_request::MedicationRequestExistence for MedicationRequest {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_prior_prescription(&self) -> bool {
        self.prior_prescription.is_some()
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_status_changed(&self) -> bool {
        self.status_changed.is_some()
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_medication(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_information_source(&self) -> bool {
        !self.information_source.is_empty()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_supporting_information(&self) -> bool {
        !self.supporting_information.is_empty()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_reported(&self) -> bool {
        self.reported.is_some()
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.is_some()
    }
    fn has_performer(&self) -> bool {
        !self.performer.is_empty()
    }
    fn has_device(&self) -> bool {
        !self.device.is_empty()
    }
    fn has_recorder(&self) -> bool {
        self.recorder.is_some()
    }
    fn has_reason(&self) -> bool {
        !self.reason.is_empty()
    }
    fn has_course_of_therapy_type(&self) -> bool {
        self.course_of_therapy_type.is_some()
    }
    fn has_insurance(&self) -> bool {
        !self.insurance.is_empty()
    }
    fn has_note(&self) -> bool {
        !self.note.is_empty()
    }
    fn has_rendered_dosage_instruction(&self) -> bool {
        self.rendered_dosage_instruction.is_some()
    }
    fn has_effective_dose_period(&self) -> bool {
        self.effective_dose_period.is_some()
    }
    fn has_dosage_instruction(&self) -> bool {
        !self.dosage_instruction.is_empty()
    }
    fn has_dispense_request(&self) -> bool {
        self.dispense_request.is_some()
    }
    fn has_substitution(&self) -> bool {
        self.substitution.is_some()
    }
    fn has_event_history(&self) -> bool {
        !self.event_history.is_empty()
    }
}

impl crate::validation::ValidatableResource for MedicationRequest {
    fn resource_type(&self) -> &'static str {
        "MedicationRequest"
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
        Some("http://hl7.org/fhir/StructureDefinition/MedicationRequest")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::medication_request::{
    MedicationRequestAccessors, MedicationRequestExistence, MedicationRequestMutators,
};
