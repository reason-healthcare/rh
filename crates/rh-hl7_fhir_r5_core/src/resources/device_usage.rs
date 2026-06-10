use crate::bindings::deviceusage_status::DeviceusageStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceUsage
///
/// A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceUsage
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceUsage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUsage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier for this record
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// active | completed | not-done | entered-in-error +
    pub status: DeviceusageStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The category of the statement - classifying how the statement is made
    pub category: Option<Vec<CodeableConcept>>,
    /// Patient using device
    pub patient: Reference,
    /// Supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Reference>>,
    /// The encounter or episode of care that establishes the context for this device use statement
    pub context: Option<Reference>,
    /// How often  the device was used (Timing)
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,
    /// How often  the device was used (Period)
    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,
    /// How often  the device was used (dateTime)
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<DateTimeType>,
    /// When the statement was made (and recorded)
    #[serde(rename = "dateAsserted")]
    pub date_asserted: Option<DateTimeType>,
    /// Extension element for the 'dateAsserted' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dateAsserted")]
    pub _date_asserted: Option<Element>,
    /// The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement
    #[serde(rename = "usageStatus")]
    pub usage_status: Option<CodeableConcept>,
    /// The reason for asserting the usage status - for example forgot, lost, stolen, broken
    #[serde(rename = "usageReason")]
    pub usage_reason: Option<Vec<CodeableConcept>>,
    /// How device is being used
    pub adherence: Option<DeviceUsageAdherence>,
    /// Who made the statement
    #[serde(rename = "informationSource")]
    pub information_source: Option<Reference>,
    /// Code or Reference to device used
    pub device: CodeableReference,
    /// Why device was used
    pub reason: Option<Vec<CodeableReference>>,
    /// Target body site
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableReference>,
    /// Addition details (comments, instructions)
    pub note: Option<Vec<Annotation>>,
}
/// DeviceUsage nested structure for the 'adherence' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUsageAdherence {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// always | never | sometimes
    ///
    /// Binding: example (Codes for adherence)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/deviceusage-adherence-code
    pub code: CodeableConcept,
    /// lost | stolen | prescribed | broken | burned | forgot
    ///
    /// Binding: example (Codes for adherence reason)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/deviceusage-adherence-reason
    pub reason: Vec<CodeableConcept>,
}

impl Default for DeviceUsage {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            based_on: Default::default(),
            status: DeviceusageStatus::default(),
            _status: Default::default(),
            category: Default::default(),
            patient: Reference::default(),
            derived_from: Default::default(),
            context: Default::default(),
            timing_timing: Default::default(),
            timing_period: Default::default(),
            timing_date_time: Default::default(),
            date_asserted: Default::default(),
            _date_asserted: Default::default(),
            usage_status: Default::default(),
            usage_reason: Default::default(),
            adherence: Default::default(),
            information_source: Default::default(),
            device: CodeableReference::default(),
            reason: Default::default(),
            body_site: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for DeviceUsageAdherence {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: CodeableConcept::default(),
            reason: Vec::new(),
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
                "DeviceUsage.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DeviceUsage.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/deviceusage-status|5.0.0",
            )
            .with_description("A coded concept indicating the current status of the Device Usage."),
            rh_foundation::ElementBinding::new(
                "DeviceUsage.usageStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/deviceusage-status|5.0.0",
            )
            .with_description("Codes representing the usage status of the device."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DeviceUsage.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.contained", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.extension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.category", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.patient", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.context", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.dateAsserted", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.usageStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.usageReason", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.adherence", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.adherence.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.adherence.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceUsage.adherence.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DeviceUsage.adherence.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.adherence.reason", 1, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.informationSource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.device", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.reason", 0, None),
            rh_foundation::ElementCardinality::new("DeviceUsage.bodySite", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceUsage.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceUsage {
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

impl crate::traits::resource::ResourceMutators for DeviceUsage {
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

impl crate::traits::resource::ResourceExistence for DeviceUsage {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceUsage {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceUsage {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceUsage {
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

impl crate::traits::device_usage::DeviceUsageAccessors for DeviceUsage {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DeviceusageStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Reference {
        self.patient.clone()
    }
    fn derived_from(&self) -> &[Reference] {
        self.derived_from.as_deref().unwrap_or(&[])
    }
    fn context(&self) -> Option<Reference> {
        self.context.clone()
    }
    fn date_asserted(&self) -> Option<DateTimeType> {
        self.date_asserted.clone()
    }
    fn usage_status(&self) -> Option<CodeableConcept> {
        self.usage_status.clone()
    }
    fn usage_reason(&self) -> &[CodeableConcept] {
        self.usage_reason.as_deref().unwrap_or(&[])
    }
    fn adherence(&self) -> Option<DeviceUsageAdherence> {
        self.adherence.clone()
    }
    fn information_source(&self) -> Option<Reference> {
        self.information_source.clone()
    }
    fn device(&self) -> CodeableReference {
        self.device.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> Option<CodeableReference> {
        self.body_site.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_usage::DeviceUsageMutators for DeviceUsage {
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
    fn set_status(self, value: DeviceusageStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = value;
        resource
    }
    fn set_derived_from(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_context(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
        resource
    }
    fn set_date_asserted(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date_asserted = Some(value);
        resource
    }
    fn set_usage_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.usage_status = Some(value);
        resource
    }
    fn set_usage_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.usage_reason = Some(value);
        resource
    }
    fn add_usage_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .usage_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_adherence(self, value: DeviceUsageAdherence) -> Self {
        let mut resource = self.clone();
        resource.adherence = Some(value);
        resource
    }
    fn set_information_source(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.information_source = Some(value);
        resource
    }
    fn set_device(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.device = value;
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_body_site(self, value: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
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
}

impl crate::traits::device_usage::DeviceUsageExistence for DeviceUsage {
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
    fn has_timing(&self) -> bool {
        self.timing_timing.is_some()
            || self.timing_period.is_some()
            || self.timing_date_time.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient(&self) -> bool {
        true
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_context(&self) -> bool {
        self.context.is_some()
    }
    fn has_date_asserted(&self) -> bool {
        self.date_asserted.is_some()
    }
    fn has_usage_status(&self) -> bool {
        self.usage_status.is_some()
    }
    fn has_usage_reason(&self) -> bool {
        self.usage_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_adherence(&self) -> bool {
        self.adherence.is_some()
    }
    fn has_information_source(&self) -> bool {
        self.information_source.is_some()
    }
    fn has_device(&self) -> bool {
        true
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceUsage {
    fn resource_type(&self) -> &'static str {
        "DeviceUsage"
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
        Some("http://hl7.org/fhir/StructureDefinition/DeviceUsage")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device_usage::{
    DeviceUsageAccessors, DeviceUsageExistence, DeviceUsageMutators,
};
