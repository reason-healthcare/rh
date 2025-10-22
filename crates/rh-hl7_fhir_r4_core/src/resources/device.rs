use crate::bindings::device_nametype::DeviceNametype;
use crate::bindings::device_status::DeviceStatus;
use crate::bindings::udi_entry_type::UdiEntryType;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Device
///
/// A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Device
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Device
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// The reference to the definition for the device
    pub definition: Option<Reference>,
    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiCarrier")]
    pub udi_carrier: Option<Vec<DeviceUdicarrier>>,
    /// active | inactive | entered-in-error | unknown
    pub status: Option<DeviceStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// online | paused | standby | offline | not-ready | transduc-discon | hw-discon | off
    ///
    /// Binding: extensible (The availability status reason of the device.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-status-reason
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// The distinct identification string
    #[serde(rename = "distinctIdentifier")]
    pub distinct_identifier: Option<StringType>,
    /// Extension element for the 'distinctIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_distinctIdentifier")]
    pub _distinct_identifier: Option<Element>,
    /// Name of device manufacturer
    pub manufacturer: Option<StringType>,
    /// Extension element for the 'manufacturer' primitive field. Contains metadata and extensions.
    pub _manufacturer: Option<Element>,
    /// Date when the device was made
    #[serde(rename = "manufactureDate")]
    pub manufacture_date: Option<DateTimeType>,
    /// Extension element for the 'manufactureDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_manufactureDate")]
    pub _manufacture_date: Option<Element>,
    /// Date and time of expiry of this device (if applicable)
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTimeType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// Lot number of manufacture
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// Serial number assigned by the manufacturer
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<StringType>,
    /// Extension element for the 'serialNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_serialNumber")]
    pub _serial_number: Option<Element>,
    /// The name of the device as given by the manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDevicename>>,
    /// The model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<StringType>,
    /// Extension element for the 'modelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_modelNumber")]
    pub _model_number: Option<Element>,
    /// The part number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<StringType>,
    /// Extension element for the 'partNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_partNumber")]
    pub _part_number: Option<Element>,
    /// The kind or type of device
    ///
    /// Binding: example (Codes to identify medical devices.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication
    pub specialization: Option<Vec<DeviceSpecialization>>,
    /// The actual design of the device or software version running on the device
    pub version: Option<Vec<DeviceVersion>>,
    /// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
    pub property: Option<Vec<DeviceProperty>>,
    /// Patient to whom Device is affixed
    pub patient: Option<Reference>,
    /// Organization responsible for device
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,
    /// Where the device is found
    pub location: Option<Reference>,
    /// Network address to contact device
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,
    /// Safety Characteristics of Device
    pub safety: Option<Vec<CodeableConcept>>,
    /// The parent device
    pub parent: Option<Reference>,
}
/// Device nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of the device version
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// A single component of the device version
    pub component: Option<Identifier>,
    /// The version text
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// Device nested structure for the 'udiCarrier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUdicarrier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Mandatory fixed portion of UDI
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: Option<StringType>,
    /// Extension element for the 'deviceIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceIdentifier")]
    pub _device_identifier: Option<Element>,
    /// UDI Issuing Organization
    pub issuer: Option<StringType>,
    /// Extension element for the 'issuer' primitive field. Contains metadata and extensions.
    pub _issuer: Option<Element>,
    /// Regional UDI authority
    pub jurisdiction: Option<StringType>,
    /// Extension element for the 'jurisdiction' primitive field. Contains metadata and extensions.
    pub _jurisdiction: Option<Element>,
    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_a_i_d_c: Option<Base64BinaryType>,
    /// Extension element for the 'carrierAIDC' primitive field. Contains metadata and extensions.
    #[serde(rename = "_carrierAIDC")]
    pub _carrier_a_i_d_c: Option<Element>,
    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_h_r_f: Option<StringType>,
    /// Extension element for the 'carrierHRF' primitive field. Contains metadata and extensions.
    #[serde(rename = "_carrierHRF")]
    pub _carrier_h_r_f: Option<Element>,
    /// barcode | rfid | manual +
    #[serde(rename = "entryType")]
    pub entry_type: Option<UdiEntryType>,
    /// Extension element for the 'entryType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_entryType")]
    pub _entry_type: Option<Element>,
}
/// Device nested structure for the 'deviceName' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDevicename {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The name of the device
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// udi-label-name | user-friendly-name | patient-reported-name | manufacturer-name | model-name | other
    #[serde(rename = "type")]
    pub type_: DeviceNametype,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// Status of Implantable Devices
///
/// Codes to represent the functional status of a device implanted in a patient.  Both overall device status and an implant status need to be considered. The implant status should only be used when the [device status](device-definitions.html#Device.status) is `active `.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/device-implantStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceImplantStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Device nested structure for the 'specialization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpecialization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The standard that is used to operate and communicate
    #[serde(rename = "systemType")]
    pub system_type: CodeableConcept,
    /// The version of the standard that is used to operate and communicate
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
}
/// Device nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code that specifies the property DeviceDefinitionPropetyCode (Extensible)
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Property value as a quantity
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Vec<Quantity>>,
    /// Property value as a code, e.g., NTP4 (synced to NTP)
    #[serde(rename = "valueCode")]
    pub value_code: Option<Vec<CodeableConcept>>,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            definition: Default::default(),
            udi_carrier: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            distinct_identifier: Default::default(),
            _distinct_identifier: Default::default(),
            manufacturer: Default::default(),
            _manufacturer: Default::default(),
            manufacture_date: Default::default(),
            _manufacture_date: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            serial_number: Default::default(),
            _serial_number: Default::default(),
            device_name: Default::default(),
            model_number: Default::default(),
            _model_number: Default::default(),
            part_number: Default::default(),
            _part_number: Default::default(),
            type_: Default::default(),
            specialization: Default::default(),
            version: Default::default(),
            property: Default::default(),
            patient: Default::default(),
            owner: Default::default(),
            contact: Default::default(),
            location: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            note: Default::default(),
            safety: Default::default(),
            parent: Default::default(),
        }
    }
}

impl Default for DeviceVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            component: Default::default(),
            value: StringType::default(),
            _value: Default::default(),
        }
    }
}

impl Default for DeviceUdicarrier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            device_identifier: Default::default(),
            _device_identifier: Default::default(),
            issuer: Default::default(),
            _issuer: Default::default(),
            jurisdiction: Default::default(),
            _jurisdiction: Default::default(),
            carrier_a_i_d_c: Default::default(),
            _carrier_a_i_d_c: Default::default(),
            carrier_h_r_f: Default::default(),
            _carrier_h_r_f: Default::default(),
            entry_type: Default::default(),
            _entry_type: Default::default(),
        }
    }
}

impl Default for DeviceDevicename {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for DeviceImplantStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for DeviceSpecialization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            system_type: CodeableConcept::default(),
            version: Default::default(),
            _version: Default::default(),
        }
    }
}

impl Default for DeviceProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_code: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Device {
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

impl crate::traits::resource::ResourceMutators for Device {
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

impl crate::traits::resource::ResourceExistence for Device {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Device {
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

impl crate::traits::domain_resource::DomainResourceMutators for Device {
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

impl crate::traits::domain_resource::DomainResourceExistence for Device {
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

impl crate::traits::device::DeviceAccessors for Device {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn definition(&self) -> Option<Reference> {
        self.definition.clone()
    }
    fn udi_carrier(&self) -> &[DeviceUdicarrier] {
        self.udi_carrier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Option<DeviceStatus> {
        self.status.clone()
    }
    fn status_reason(&self) -> &[CodeableConcept] {
        self.status_reason.as_deref().unwrap_or(&[])
    }
    fn distinct_identifier(&self) -> Option<StringType> {
        self.distinct_identifier.clone()
    }
    fn manufacturer(&self) -> Option<StringType> {
        self.manufacturer.clone()
    }
    fn manufacture_date(&self) -> Option<DateTimeType> {
        self.manufacture_date.clone()
    }
    fn expiration_date(&self) -> Option<DateTimeType> {
        self.expiration_date.clone()
    }
    fn lot_number(&self) -> Option<StringType> {
        self.lot_number.clone()
    }
    fn serial_number(&self) -> Option<StringType> {
        self.serial_number.clone()
    }
    fn device_name(&self) -> &[DeviceDevicename] {
        self.device_name.as_deref().unwrap_or(&[])
    }
    fn model_number(&self) -> Option<StringType> {
        self.model_number.clone()
    }
    fn part_number(&self) -> Option<StringType> {
        self.part_number.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn specialization(&self) -> &[DeviceSpecialization] {
        self.specialization.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> &[DeviceVersion] {
        self.version.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[DeviceProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn patient(&self) -> Option<Reference> {
        self.patient.clone()
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn safety(&self) -> &[CodeableConcept] {
        self.safety.as_deref().unwrap_or(&[])
    }
    fn parent(&self) -> Option<Reference> {
        self.parent.clone()
    }
}

impl crate::traits::device::DeviceMutators for Device {
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
    fn set_definition(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
    fn set_udi_carrier(self, value: Vec<DeviceUdicarrier>) -> Self {
        let mut resource = self.clone();
        resource.udi_carrier = Some(value);
        resource
    }
    fn add_udi_carrier(self, item: DeviceUdicarrier) -> Self {
        let mut resource = self.clone();
        resource.udi_carrier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: DeviceStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
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
    fn set_distinct_identifier(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.distinct_identifier = Some(value);
        resource
    }
    fn set_manufacturer(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn set_manufacture_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.manufacture_date = Some(value);
        resource
    }
    fn set_expiration_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expiration_date = Some(value);
        resource
    }
    fn set_lot_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.lot_number = Some(value);
        resource
    }
    fn set_serial_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.serial_number = Some(value);
        resource
    }
    fn set_device_name(self, value: Vec<DeviceDevicename>) -> Self {
        let mut resource = self.clone();
        resource.device_name = Some(value);
        resource
    }
    fn add_device_name(self, item: DeviceDevicename) -> Self {
        let mut resource = self.clone();
        resource.device_name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_model_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.model_number = Some(value);
        resource
    }
    fn set_part_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.part_number = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_specialization(self, value: Vec<DeviceSpecialization>) -> Self {
        let mut resource = self.clone();
        resource.specialization = Some(value);
        resource
    }
    fn add_specialization(self, item: DeviceSpecialization) -> Self {
        let mut resource = self.clone();
        resource
            .specialization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_version(self, value: Vec<DeviceVersion>) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn add_version(self, item: DeviceVersion) -> Self {
        let mut resource = self.clone();
        resource.version.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<DeviceProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: DeviceProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_patient(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.patient = Some(value);
        resource
    }
    fn set_owner(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.owner = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactPoint>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactPoint) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
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
    fn set_safety(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.safety = Some(value);
        resource
    }
    fn add_safety(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.safety.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_parent(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
}

impl crate::traits::device::DeviceExistence for Device {
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
    fn has_definition(&self) -> bool {
        self.definition.is_some()
    }
    fn has_udi_carrier(&self) -> bool {
        self.udi_carrier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_distinct_identifier(&self) -> bool {
        self.distinct_identifier.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.is_some()
    }
    fn has_manufacture_date(&self) -> bool {
        self.manufacture_date.is_some()
    }
    fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }
    fn has_lot_number(&self) -> bool {
        self.lot_number.is_some()
    }
    fn has_serial_number(&self) -> bool {
        self.serial_number.is_some()
    }
    fn has_device_name(&self) -> bool {
        self.device_name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_model_number(&self) -> bool {
        self.model_number.is_some()
    }
    fn has_part_number(&self) -> bool {
        self.part_number.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_specialization(&self) -> bool {
        self.specialization.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient(&self) -> bool {
        self.patient.is_some()
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_safety(&self) -> bool {
        self.safety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}
