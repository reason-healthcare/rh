use crate::bindings::location_mode::LocationMode;
use crate::bindings::location_status::LocationStatus;
use crate::datatypes::address::Address;
use crate::datatypes::availability::Availability;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::extended_contact_detail::ExtendedContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::virtual_service_detail::VirtualServiceDetail;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Location
///
/// Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Location
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Location
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique code or number identifying the location to its users
    pub identifier: Option<Vec<Identifier>>,
    /// active | suspended | inactive
    pub status: Option<LocationStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The operational status of the location (typically only for a bed/room)
    ///
    /// Binding: preferred (The operational status if the location (where typically a bed/room).)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0116
    #[serde(rename = "operationalStatus")]
    pub operational_status: Option<Coding>,
    /// Name of the location as used by humans
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// A list of alternate names that the location is known as, or was known as, in the past
    pub alias: Option<Vec<StringType>>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    pub _alias: Option<Element>,
    /// Additional details about the location that could be displayed as further information to identify the location beyond its name
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// instance | kind
    pub mode: Option<LocationMode>,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Type of function performed
    ///
    /// Binding: extensible (Indicates the type of function performed at the location.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ServiceDeliveryLocationRoleType
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Official contact details for the location
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Physical location
    pub address: Option<Address>,
    /// Physical form of the location
    ///
    /// Binding: example (Physical form of the location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-form
    pub form: Option<CodeableConcept>,
    /// The absolute geographic location
    pub position: Option<LocationPosition>,
    /// Organization responsible for provisioning and upkeep
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Another Location this one is physically a part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Reference>,
    /// Collection of characteristics (attributes)
    ///
    /// Binding: example (A custom attribute that could be provided at a service (e.g. Wheelchair accessibiliy).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-characteristic
    pub characteristic: Option<Vec<CodeableConcept>>,
    /// What days/times during a week is this location usually open (including exceptions)
    #[serde(rename = "hoursOfOperation")]
    pub hours_of_operation: Option<Vec<Availability>>,
    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,
    /// Technical endpoints providing access to services operated for the location
    pub endpoint: Option<Vec<Reference>>,
}
/// Location nested structure for the 'position' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationPosition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Longitude with WGS84 datum
    pub longitude: DecimalType,
    /// Extension element for the 'longitude' primitive field. Contains metadata and extensions.
    pub _longitude: Option<Element>,
    /// Latitude with WGS84 datum
    pub latitude: DecimalType,
    /// Extension element for the 'latitude' primitive field. Contains metadata and extensions.
    pub _latitude: Option<Element>,
    /// Altitude with WGS84 datum
    pub altitude: Option<DecimalType>,
    /// Extension element for the 'altitude' primitive field. Contains metadata and extensions.
    pub _altitude: Option<Element>,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            operational_status: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            mode: Default::default(),
            _mode: Default::default(),
            type_: Default::default(),
            contact: Default::default(),
            address: Default::default(),
            form: Default::default(),
            position: Default::default(),
            managing_organization: Default::default(),
            part_of: Default::default(),
            characteristic: Default::default(),
            hours_of_operation: Default::default(),
            virtual_service: Default::default(),
            endpoint: Default::default(),
        }
    }
}

impl Default for LocationPosition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            longitude: DecimalType::default(),
            _longitude: Default::default(),
            latitude: DecimalType::default(),
            _latitude: Default::default(),
            altitude: Default::default(),
            _altitude: Default::default(),
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
    rh_foundation::ElementBinding::new("Location.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("Location.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/location-mode|5.0.0").with_description("Indicates whether a resource instance represents a specific location or a class of locations."),
    rh_foundation::ElementBinding::new("Location.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/location-status|5.0.0").with_description("Indicates whether the location is still in use."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Location.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.contained", 0, None),
            rh_foundation::ElementCardinality::new("Location.extension", 0, None),
            rh_foundation::ElementCardinality::new("Location.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Location.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Location.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.operationalStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.alias", 0, None),
            rh_foundation::ElementCardinality::new("Location.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.mode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.type", 0, None),
            rh_foundation::ElementCardinality::new("Location.contact", 0, None),
            rh_foundation::ElementCardinality::new("Location.address", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.form", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.position", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.position.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.position.extension", 0, None),
            rh_foundation::ElementCardinality::new("Location.position.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Location.position.longitude", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Location.position.latitude", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Location.position.altitude", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.managingOrganization", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.partOf", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Location.characteristic", 0, None),
            rh_foundation::ElementCardinality::new("Location.hoursOfOperation", 0, None),
            rh_foundation::ElementCardinality::new("Location.virtualService", 0, None),
            rh_foundation::ElementCardinality::new("Location.endpoint", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Location {
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

impl crate::traits::resource::ResourceMutators for Location {
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

impl crate::traits::resource::ResourceExistence for Location {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Location {
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

impl crate::traits::domain_resource::DomainResourceMutators for Location {
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

impl crate::traits::domain_resource::DomainResourceExistence for Location {
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

impl crate::traits::location::LocationAccessors for Location {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<LocationStatus> {
        self.status.clone()
    }
    fn operational_status(&self) -> Option<Coding> {
        self.operational_status.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn alias(&self) -> &[StringType] {
        self.alias.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn mode(&self) -> Option<LocationMode> {
        self.mode.clone()
    }
    fn type_(&self) -> &[CodeableConcept] {
        self.type_.as_deref().unwrap_or(&[])
    }
    fn contact(&self) -> &[ExtendedContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn address(&self) -> Option<Address> {
        self.address.clone()
    }
    fn form(&self) -> Option<CodeableConcept> {
        self.form.clone()
    }
    fn position(&self) -> Option<LocationPosition> {
        self.position.clone()
    }
    fn managing_organization(&self) -> Option<Reference> {
        self.managing_organization.clone()
    }
    fn part_of(&self) -> Option<Reference> {
        self.part_of.clone()
    }
    fn characteristic(&self) -> &[CodeableConcept] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
    fn hours_of_operation(&self) -> &[Availability] {
        self.hours_of_operation.as_deref().unwrap_or(&[])
    }
    fn virtual_service(&self) -> &[VirtualServiceDetail] {
        self.virtual_service.as_deref().unwrap_or(&[])
    }
    fn endpoint(&self) -> &[Reference] {
        self.endpoint.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::location::LocationMutators for Location {
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
    fn set_status(self, value: LocationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_operational_status(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.operational_status = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_alias(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.alias = Some(value);
        resource
    }
    fn add_alias(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.alias.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_mode(self, value: LocationMode) -> Self {
        let mut resource = self.clone();
        resource.mode = Some(value);
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
    fn set_contact(self, value: Vec<ExtendedContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ExtendedContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_address(self, value: Address) -> Self {
        let mut resource = self.clone();
        resource.address = Some(value);
        resource
    }
    fn set_form(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.form = Some(value);
        resource
    }
    fn set_position(self, value: LocationPosition) -> Self {
        let mut resource = self.clone();
        resource.position = Some(value);
        resource
    }
    fn set_managing_organization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.managing_organization = Some(value);
        resource
    }
    fn set_part_of(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn set_characteristic(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_hours_of_operation(self, value: Vec<Availability>) -> Self {
        let mut resource = self.clone();
        resource.hours_of_operation = Some(value);
        resource
    }
    fn add_hours_of_operation(self, item: Availability) -> Self {
        let mut resource = self.clone();
        resource
            .hours_of_operation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_virtual_service(self, value: Vec<VirtualServiceDetail>) -> Self {
        let mut resource = self.clone();
        resource.virtual_service = Some(value);
        resource
    }
    fn add_virtual_service(self, item: VirtualServiceDetail) -> Self {
        let mut resource = self.clone();
        resource
            .virtual_service
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_endpoint(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.endpoint = Some(value);
        resource
    }
    fn add_endpoint(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.endpoint.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::location::LocationExistence for Location {
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
        self.status.is_some()
    }
    fn has_operational_status(&self) -> bool {
        self.operational_status.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_alias(&self) -> bool {
        self.alias.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_mode(&self) -> bool {
        self.mode.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_address(&self) -> bool {
        self.address.is_some()
    }
    fn has_form(&self) -> bool {
        self.form.is_some()
    }
    fn has_position(&self) -> bool {
        self.position.is_some()
    }
    fn has_managing_organization(&self) -> bool {
        self.managing_organization.is_some()
    }
    fn has_part_of(&self) -> bool {
        self.part_of.is_some()
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_hours_of_operation(&self) -> bool {
        self.hours_of_operation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_virtual_service(&self) -> bool {
        self.virtual_service.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Location {
    fn resource_type(&self) -> &'static str {
        "Location"
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
        Some("http://hl7.org/fhir/StructureDefinition/Location")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::location::{LocationAccessors, LocationExistence, LocationMutators};
