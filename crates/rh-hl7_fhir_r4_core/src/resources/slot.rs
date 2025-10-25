use crate::bindings::slotstatus::Slotstatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Slot
///
/// A slot of time on a schedule that may be available for booking appointments.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Slot
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Slot
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// A broad categorization of the service that is to be performed during this appointment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-category
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides the value provided on the availability resource
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableConcept>>,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    ///
    /// Binding: preferred (Additional details about where the content was created (e.g. clinical specialty).)
    ///
    /// Available values:
    /// - `408467006`: Adult mental illness
    /// - `394577000`: Anesthetics
    /// - `394578005`: Audiological medicine
    /// - `421661004`: Blood banking and transfusion medicine
    /// - `408462000`: Burns care
    /// - `394579002`: Cardiology
    /// - `394804000`: Clinical cytogenetics and molecular genetics
    /// - `394580004`: Clinical genetics
    /// - `394803006`: Clinical hematology
    /// - `408480009`: Clinical immunology
    /// - ... and 107 more values
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The style of appointment or patient that may be booked in the slot (not service type)
    ///
    /// Binding: preferred (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0276
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<CodeableConcept>,
    /// The schedule resource that this slot defines an interval of status information
    pub schedule: Reference,
    /// busy | free | busy-unavailable | busy-tentative | entered-in-error
    pub status: Slotstatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Date/Time that the slot is to begin
    pub start: InstantType,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// Date/Time that the slot is to conclude
    pub end: InstantType,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// This slot has already been overbooked, appointments are unlikely to be accepted for this time
    pub overbooked: Option<BooleanType>,
    /// Extension element for the 'overbooked' primitive field. Contains metadata and extensions.
    pub _overbooked: Option<Element>,
    /// Comments on the slot to describe any extended information. Such as custom constraints on the slot
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            service_category: Default::default(),
            service_type: Default::default(),
            specialty: Default::default(),
            appointment_type: Default::default(),
            schedule: Reference::default(),
            status: Slotstatus::default(),
            _status: Default::default(),
            start: InstantType::default(),
            _start: Default::default(),
            end: InstantType::default(),
            _end: Default::default(),
            overbooked: Default::default(),
            _overbooked: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
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
        vec![rh_foundation::ElementBinding::new(
            "Slot.status",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/slotstatus|4.0.1",
        )
        .with_description("The free/busy status of the slot.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Slot.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.contained", 0, None),
            rh_foundation::ElementCardinality::new("Slot.extension", 0, None),
            rh_foundation::ElementCardinality::new("Slot.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Slot.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Slot.serviceCategory", 0, None),
            rh_foundation::ElementCardinality::new("Slot.serviceType", 0, None),
            rh_foundation::ElementCardinality::new("Slot.specialty", 0, None),
            rh_foundation::ElementCardinality::new("Slot.appointmentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.schedule", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.start", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.end", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.overbooked", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Slot.comment", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Slot {
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

impl crate::traits::resource::ResourceMutators for Slot {
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

impl crate::traits::resource::ResourceExistence for Slot {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Slot {
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

impl crate::traits::domain_resource::DomainResourceMutators for Slot {
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

impl crate::traits::domain_resource::DomainResourceExistence for Slot {
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

impl crate::traits::slot::SlotAccessors for Slot {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn service_category(&self) -> &[CodeableConcept] {
        self.service_category.as_deref().unwrap_or(&[])
    }
    fn service_type(&self) -> &[CodeableConcept] {
        self.service_type.as_deref().unwrap_or(&[])
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_deref().unwrap_or(&[])
    }
    fn appointment_type(&self) -> Option<CodeableConcept> {
        self.appointment_type.clone()
    }
    fn schedule(&self) -> Reference {
        self.schedule.clone()
    }
    fn status(&self) -> Slotstatus {
        self.status.clone()
    }
    fn start(&self) -> InstantType {
        self.start.clone()
    }
    fn end(&self) -> InstantType {
        self.end.clone()
    }
    fn overbooked(&self) -> Option<BooleanType> {
        self.overbooked
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
}

impl crate::traits::slot::SlotMutators for Slot {
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
    fn set_service_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_category = Some(value);
        resource
    }
    fn add_service_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_category
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_service_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_type = Some(value);
        resource
    }
    fn add_service_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.specialty = Some(value);
        resource
    }
    fn add_specialty(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.specialty.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_appointment_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.appointment_type = Some(value);
        resource
    }
    fn set_schedule(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.schedule = value;
        resource
    }
    fn set_status(self, value: Slotstatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_start(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.start = value;
        resource
    }
    fn set_end(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.end = value;
        resource
    }
    fn set_overbooked(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.overbooked = Some(value);
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
}

impl crate::traits::slot::SlotExistence for Slot {
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
    fn has_service_category(&self) -> bool {
        self.service_category
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_service_type(&self) -> bool {
        self.service_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specialty(&self) -> bool {
        self.specialty.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_appointment_type(&self) -> bool {
        self.appointment_type.is_some()
    }
    fn has_schedule(&self) -> bool {
        true
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_start(&self) -> bool {
        true
    }
    fn has_end(&self) -> bool {
        true
    }
    fn has_overbooked(&self) -> bool {
        self.overbooked.is_some()
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
}

impl crate::validation::ValidatableResource for Slot {
    fn resource_type(&self) -> &'static str {
        "Slot"
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
        Some("http://hl7.org/fhir/StructureDefinition/Slot")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::slot::{SlotAccessors, SlotExistence, SlotMutators};
