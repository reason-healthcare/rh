use crate::bindings::devicedispense_status::DevicedispenseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceDispense
///
/// Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDispense
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDispense {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this dispensation
    pub identifier: Option<Vec<Identifier>>,
    /// The order or request that this dispense is fulfilling
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// The bigger event that this dispense is a part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    pub status: DevicedispenseStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Why a dispense was or was not performed
    ///
    /// Binding: example (A code describing why a dispense was or was not performed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/devicedispense-status-reason
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableReference>,
    /// Type of device dispense
    pub category: Option<Vec<CodeableConcept>>,
    /// What device was supplied
    pub device: CodeableReference,
    /// Who the dispense is for
    pub subject: Reference,
    /// Who collected the device or where the medication was delivered
    pub receiver: Option<Reference>,
    /// Encounter associated with event
    pub encounter: Option<Reference>,
    /// Information that supports the dispensing of the device
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Who performed event
    pub performer: Option<Vec<DeviceDispensePerformer>>,
    /// Where the dispense occurred
    pub location: Option<Reference>,
    /// Trial fill, partial fill, emergency fill, etc
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Amount dispensed
    pub quantity: Option<Quantity>,
    /// When product was packaged and reviewed
    #[serde(rename = "preparedDate")]
    pub prepared_date: Option<DateTimeType>,
    /// Extension element for the 'preparedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preparedDate")]
    pub _prepared_date: Option<Element>,
    /// When product was given out
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<DateTimeType>,
    /// Extension element for the 'whenHandedOver' primitive field. Contains metadata and extensions.
    #[serde(rename = "_whenHandedOver")]
    pub _when_handed_over: Option<Element>,
    /// Where the device was sent or should be sent
    pub destination: Option<Reference>,
    /// Information about the dispense
    pub note: Option<Vec<Annotation>>,
    /// Full representation of the usage instructions
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<StringType>,
    /// Extension element for the 'usageInstruction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_usageInstruction")]
    pub _usage_instruction: Option<Element>,
    /// A list of relevant lifecycle events
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Reference>>,
}
/// DeviceDispense nested structure for the 'performer' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDispensePerformer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who performed the dispense and what they did
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    pub actor: Reference,
}

impl Default for DeviceDispense {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            part_of: Default::default(),
            status: DevicedispenseStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            category: Default::default(),
            device: CodeableReference::default(),
            subject: Reference::default(),
            receiver: Default::default(),
            encounter: Default::default(),
            supporting_information: Default::default(),
            performer: Default::default(),
            location: Default::default(),
            type_: Default::default(),
            quantity: Default::default(),
            prepared_date: Default::default(),
            _prepared_date: Default::default(),
            when_handed_over: Default::default(),
            _when_handed_over: Default::default(),
            destination: Default::default(),
            note: Default::default(),
            usage_instruction: Default::default(),
            _usage_instruction: Default::default(),
            event_history: Default::default(),
        }
    }
}

impl Default for DeviceDispensePerformer {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            function: Default::default(),
            actor: Reference::default(),
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
                "DeviceDispense.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DeviceDispense.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/devicedispense-status|5.0.0",
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
            rh_foundation::ElementCardinality::new("DeviceDispense.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.contained", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.extension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.partOf", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.category", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.device", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.receiver", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.supportingInformation", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.performer", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.performer.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.performer.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceDispense.performer.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceDispense.performer.function", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.performer.actor", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.preparedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.whenHandedOver", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.destination", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.note", 0, None),
            rh_foundation::ElementCardinality::new("DeviceDispense.usageInstruction", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceDispense.eventHistory", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceDispense {
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

impl crate::traits::resource::ResourceMutators for DeviceDispense {
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

impl crate::traits::resource::ResourceExistence for DeviceDispense {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceDispense {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceDispense {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceDispense {
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

impl crate::traits::device_dispense::DeviceDispenseAccessors for DeviceDispense {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DevicedispenseStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableReference> {
        self.status_reason.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn device(&self) -> CodeableReference {
        self.device.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn receiver(&self) -> Option<Reference> {
        self.receiver.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn performer(&self) -> &[DeviceDispensePerformer] {
        self.performer.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn prepared_date(&self) -> Option<DateTimeType> {
        self.prepared_date.clone()
    }
    fn when_handed_over(&self) -> Option<DateTimeType> {
        self.when_handed_over.clone()
    }
    fn destination(&self) -> Option<Reference> {
        self.destination.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn usage_instruction(&self) -> Option<StringType> {
        self.usage_instruction.clone()
    }
    fn event_history(&self) -> &[Reference] {
        self.event_history.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_dispense::DeviceDispenseMutators for DeviceDispense {
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
    fn set_status(self, value: DevicedispenseStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
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
    fn set_device(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.device = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_receiver(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.receiver = Some(value);
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
    fn set_performer(self, value: Vec<DeviceDispensePerformer>) -> Self {
        let mut resource = self.clone();
        resource.performer = Some(value);
        resource
    }
    fn add_performer(self, item: DeviceDispensePerformer) -> Self {
        let mut resource = self.clone();
        resource.performer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
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
    fn set_prepared_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.prepared_date = Some(value);
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
    fn set_usage_instruction(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage_instruction = Some(value);
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

impl crate::traits::device_dispense::DeviceDispenseExistence for DeviceDispense {
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
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_device(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_receiver(&self) -> bool {
        self.receiver.is_some()
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
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_prepared_date(&self) -> bool {
        self.prepared_date.is_some()
    }
    fn has_when_handed_over(&self) -> bool {
        self.when_handed_over.is_some()
    }
    fn has_destination(&self) -> bool {
        self.destination.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_usage_instruction(&self) -> bool {
        self.usage_instruction.is_some()
    }
    fn has_event_history(&self) -> bool {
        self.event_history.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceDispense {
    fn resource_type(&self) -> &'static str {
        "DeviceDispense"
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
        Some("http://hl7.org/fhir/StructureDefinition/DeviceDispense")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device_dispense::{
    DeviceDispenseAccessors, DeviceDispenseExistence, DeviceDispenseMutators,
};
