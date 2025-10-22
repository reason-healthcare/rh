use crate::bindings::days_of_week::DaysOfWeek;
use crate::bindings::location_mode::LocationMode;
use crate::bindings::location_status::LocationStatus;
use crate::datatypes::address::Address;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Location
///
/// Details and position information for a physical place where services are provided and resources and participants may be stored, found, contained, or accommodated.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Location
/// - Version: 4.0.1
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
    /// Contact details of the location
    pub telecom: Option<Vec<ContactPoint>>,
    /// Physical location
    pub address: Option<Address>,
    /// Physical form of the location
    ///
    /// Binding: example (Physical form of the location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/location-physical-type
    #[serde(rename = "physicalType")]
    pub physical_type: Option<CodeableConcept>,
    /// The absolute geographic location
    pub position: Option<LocationPosition>,
    /// Organization responsible for provisioning and upkeep
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Reference>,
    /// Another Location this one is physically a part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Reference>,
    /// What days/times during a week is this location usually open
    #[serde(rename = "hoursOfOperation")]
    pub hours_of_operation: Option<Vec<LocationHoursofoperation>>,
    /// Description of availability exceptions
    #[serde(rename = "availabilityExceptions")]
    pub availability_exceptions: Option<StringType>,
    /// Extension element for the 'availabilityExceptions' primitive field. Contains metadata and extensions.
    #[serde(rename = "_availabilityExceptions")]
    pub _availability_exceptions: Option<Element>,
    /// Technical endpoints providing access to services operated for the location
    pub endpoint: Option<Vec<Reference>>,
}
/// Distance
///
/// A calculated distance between the resource and a provided location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/location-distance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDistance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
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
/// Location nested structure for the 'hoursOfOperation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHoursofoperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Option<Vec<DaysOfWeek>>,
    /// Extension element for the 'daysOfWeek' primitive field. Contains metadata and extensions.
    #[serde(rename = "_daysOfWeek")]
    pub _days_of_week: Option<Element>,
    /// The Location is open all day
    #[serde(rename = "allDay")]
    pub all_day: Option<BooleanType>,
    /// Extension element for the 'allDay' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allDay")]
    pub _all_day: Option<Element>,
    /// Time that the Location opens
    #[serde(rename = "openingTime")]
    pub opening_time: Option<TimeType>,
    /// Extension element for the 'openingTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_openingTime")]
    pub _opening_time: Option<Element>,
    /// Time that the Location closes
    #[serde(rename = "closingTime")]
    pub closing_time: Option<TimeType>,
    /// Extension element for the 'closingTime' primitive field. Contains metadata and extensions.
    #[serde(rename = "_closingTime")]
    pub _closing_time: Option<Element>,
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
            telecom: Default::default(),
            address: Default::default(),
            physical_type: Default::default(),
            position: Default::default(),
            managing_organization: Default::default(),
            part_of: Default::default(),
            hours_of_operation: Default::default(),
            availability_exceptions: Default::default(),
            _availability_exceptions: Default::default(),
            endpoint: Default::default(),
        }
    }
}

impl Default for LocationDistance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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

impl Default for LocationHoursofoperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            days_of_week: Default::default(),
            _days_of_week: Default::default(),
            all_day: Default::default(),
            _all_day: Default::default(),
            opening_time: Default::default(),
            _opening_time: Default::default(),
            closing_time: Default::default(),
            _closing_time: Default::default(),
        }
    }
}

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
    fn telecom(&self) -> &[ContactPoint] {
        self.telecom.as_deref().unwrap_or(&[])
    }
    fn address(&self) -> Option<Address> {
        self.address.clone()
    }
    fn physical_type(&self) -> Option<CodeableConcept> {
        self.physical_type.clone()
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
    fn hours_of_operation(&self) -> &[LocationHoursofoperation] {
        self.hours_of_operation.as_deref().unwrap_or(&[])
    }
    fn availability_exceptions(&self) -> Option<StringType> {
        self.availability_exceptions.clone()
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
    fn set_telecom(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.telecom = Some(value);
        resource
    }
    fn add_telecom(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.telecom.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_address(self, value: Address) -> Self {
        let mut resource = self.clone();
        resource.address = Some(value);
        resource
    }
    fn set_physical_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.physical_type = Some(value);
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
    fn set_hours_of_operation(self, value: Vec<LocationHoursofoperation>) -> Self {
        let mut resource = self.clone();
        resource.hours_of_operation = Some(value);
        resource
    }
    fn add_hours_of_operation(self, item: LocationHoursofoperation) -> Self {
        let mut resource = self.clone();
        resource
            .hours_of_operation
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_availability_exceptions(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.availability_exceptions = Some(value);
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
    fn has_telecom(&self) -> bool {
        self.telecom.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_address(&self) -> bool {
        self.address.is_some()
    }
    fn has_physical_type(&self) -> bool {
        self.physical_type.is_some()
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
    fn has_hours_of_operation(&self) -> bool {
        self.hours_of_operation
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_availability_exceptions(&self) -> bool {
        self.availability_exceptions.is_some()
    }
    fn has_endpoint(&self) -> bool {
        self.endpoint.as_ref().is_some_and(|v| !v.is_empty())
    }
}
