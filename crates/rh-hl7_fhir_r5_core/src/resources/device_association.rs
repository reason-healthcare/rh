use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceAssociation
///
/// A record of association of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceAssociation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAssociation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Reference to the devices associated with the patient or group
    pub device: Reference,
    /// Describes the relationship between the device and subject
    pub category: Option<Vec<CodeableConcept>>,
    /// implanted | explanted | attached | entered-in-error | unknown
    pub status: CodeableConcept,
    /// The reasons given for the current association status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// The individual, group of individuals or device that the device is on or associated with
    pub subject: Option<Reference>,
    /// Current anatomical location of the device in/on subject
    #[serde(rename = "bodyStructure")]
    pub body_structure: Option<Reference>,
    /// Begin and end dates and times for the device association
    pub period: Option<Period>,
    /// The details about the device when it is in use to describe its operation
    pub operation: Option<Vec<DeviceAssociationOperation>>,
}
/// DeviceAssociation nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAssociationOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Device operational condition
    ///
    /// Binding: example (Describes the the status of the association operation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/deviceassociation-operationstatus
    pub status: CodeableConcept,
    /// The individual performing the action enabled by the device
    pub operator: Option<Vec<Reference>>,
    /// Begin and end dates and times for the device's operation
    pub period: Option<Period>,
}

impl Default for DeviceAssociation {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            device: Reference::default(),
            category: Default::default(),
            status: CodeableConcept::default(),
            status_reason: Default::default(),
            subject: Default::default(),
            body_structure: Default::default(),
            period: Default::default(),
            operation: Default::default(),
        }
    }
}

impl Default for DeviceAssociationOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            status: CodeableConcept::default(),
            operator: Default::default(),
            period: Default::default(),
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
                "DeviceAssociation.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DeviceAssociation.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/deviceassociation-status|5.0.0",
            )
            .with_description("Describes the lifecycle of the association."),
            rh_foundation::ElementBinding::new(
                "DeviceAssociation.statusReason",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/deviceassociation-status-reason|5.0.0",
            )
            .with_description("Describes the reason for changing the status of the association."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DeviceAssociation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.contained", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.extension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.device", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.category", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.statusReason", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.bodyStructure", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DeviceAssociation.operation", 0, None),
            rh_foundation::ElementCardinality::new("DeviceAssociation.operation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DeviceAssociation.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceAssociation.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DeviceAssociation.operation.status",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DeviceAssociation.operation.operator", 0, None),
            rh_foundation::ElementCardinality::new(
                "DeviceAssociation.operation.period",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceAssociation {
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

impl crate::traits::resource::ResourceMutators for DeviceAssociation {
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

impl crate::traits::resource::ResourceExistence for DeviceAssociation {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceAssociation {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceAssociation {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceAssociation {
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

impl crate::traits::device_association::DeviceAssociationAccessors for DeviceAssociation {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn device(&self) -> Reference {
        self.device.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> CodeableConcept {
        self.status.clone()
    }
    fn status_reason(&self) -> &[CodeableConcept] {
        self.status_reason.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn body_structure(&self) -> Option<Reference> {
        self.body_structure.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn operation(&self) -> &[DeviceAssociationOperation] {
        self.operation.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_association::DeviceAssociationMutators for DeviceAssociation {
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
    fn set_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.device = value;
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
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn add_status_reason(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .status_reason
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_body_structure(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.body_structure = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_operation(self, value: Vec<DeviceAssociationOperation>) -> Self {
        let mut resource = self.clone();
        resource.operation = Some(value);
        resource
    }
    fn add_operation(self, item: DeviceAssociationOperation) -> Self {
        let mut resource = self.clone();
        resource.operation.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::device_association::DeviceAssociationExistence for DeviceAssociation {
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
    fn has_device(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_body_structure(&self) -> bool {
        self.body_structure.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_operation(&self) -> bool {
        self.operation.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceAssociation {
    fn resource_type(&self) -> &'static str {
        "DeviceAssociation"
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
        Some("http://hl7.org/fhir/StructureDefinition/DeviceAssociation")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::device_association::{
    DeviceAssociationAccessors, DeviceAssociationExistence, DeviceAssociationMutators,
};
