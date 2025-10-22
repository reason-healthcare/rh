use crate::bindings::device_status::DeviceStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::device::DeviceDevicename;
use crate::resources::device::DeviceProperty;
use crate::resources::device::DeviceSpecialization;
use crate::resources::device::DeviceUdicarrier;
use crate::resources::device::DeviceVersion;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Device Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Device
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Device
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the definition field.
    fn definition(&self) -> Option<Reference>;
    /// Returns a reference to the udiCarrier field.
    fn udi_carrier(&self) -> &[DeviceUdicarrier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<DeviceStatus>;
    /// Returns a reference to the statusReason field.
    fn status_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the distinctIdentifier field.
    fn distinct_identifier(&self) -> Option<StringType>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> Option<StringType>;
    /// Returns a reference to the manufactureDate field.
    fn manufacture_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the expirationDate field.
    fn expiration_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the lotNumber field.
    fn lot_number(&self) -> Option<StringType>;
    /// Returns a reference to the serialNumber field.
    fn serial_number(&self) -> Option<StringType>;
    /// Returns a reference to the deviceName field.
    fn device_name(&self) -> &[DeviceDevicename];
    /// Returns a reference to the modelNumber field.
    fn model_number(&self) -> Option<StringType>;
    /// Returns a reference to the partNumber field.
    fn part_number(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specialization field.
    fn specialization(&self) -> &[DeviceSpecialization];
    /// Returns a reference to the version field.
    fn version(&self) -> &[DeviceVersion];
    /// Returns a reference to the property field.
    fn property(&self) -> &[DeviceProperty];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Option<Reference>;
    /// Returns a reference to the owner field.
    fn owner(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the safety field.
    fn safety(&self) -> &[CodeableConcept];
    /// Returns a reference to the parent field.
    fn parent(&self) -> Option<Reference>;
}
/// Device Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Device
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Device
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceMutators: DomainResourceMutators {
    /// Create a new Device with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::device::Device;
    /// use hl7_fhir_r4_core::traits::device::DeviceMutators;
    ///
    /// let resource = Device::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: Reference) -> Self;
    /// Sets the udiCarrier field and returns self for chaining.
    fn set_udi_carrier(self, value: Vec<DeviceUdicarrier>) -> Self;
    /// Adds an item to the udiCarrier field and returns self for chaining.
    fn add_udi_carrier(self, item: DeviceUdicarrier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: DeviceStatus) -> Self;
    /// Sets the statusReason field and returns self for chaining.
    fn set_status_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the statusReason field and returns self for chaining.
    fn add_status_reason(self, item: CodeableConcept) -> Self;
    /// Sets the distinctIdentifier field and returns self for chaining.
    fn set_distinct_identifier(self, value: String) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: String) -> Self;
    /// Sets the manufactureDate field and returns self for chaining.
    fn set_manufacture_date(self, value: String) -> Self;
    /// Sets the expirationDate field and returns self for chaining.
    fn set_expiration_date(self, value: String) -> Self;
    /// Sets the lotNumber field and returns self for chaining.
    fn set_lot_number(self, value: String) -> Self;
    /// Sets the serialNumber field and returns self for chaining.
    fn set_serial_number(self, value: String) -> Self;
    /// Sets the deviceName field and returns self for chaining.
    fn set_device_name(self, value: Vec<DeviceDevicename>) -> Self;
    /// Adds an item to the deviceName field and returns self for chaining.
    fn add_device_name(self, item: DeviceDevicename) -> Self;
    /// Sets the modelNumber field and returns self for chaining.
    fn set_model_number(self, value: String) -> Self;
    /// Sets the partNumber field and returns self for chaining.
    fn set_part_number(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the specialization field and returns self for chaining.
    fn set_specialization(self, value: Vec<DeviceSpecialization>) -> Self;
    /// Adds an item to the specialization field and returns self for chaining.
    fn add_specialization(self, item: DeviceSpecialization) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: Vec<DeviceVersion>) -> Self;
    /// Adds an item to the version field and returns self for chaining.
    fn add_version(self, item: DeviceVersion) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<DeviceProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: DeviceProperty) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the owner field and returns self for chaining.
    fn set_owner(self, value: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the safety field and returns self for chaining.
    fn set_safety(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the safety field and returns self for chaining.
    fn add_safety(self, item: CodeableConcept) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Reference) -> Self;
}
/// Device Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Device
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Device
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceExistence: DomainResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the text field is present (Some).
    fn has_text(&self) -> bool;
    /// Returns true if the contained field is not empty.
    fn has_contained(&self) -> bool;
    /// Returns true if the extension field is not empty.
    fn has_extension(&self) -> bool;
    /// Returns true if the modifier_extension field is not empty.
    fn has_modifier_extension(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the definition field is present (Some).
    fn has_definition(&self) -> bool;
    /// Returns true if the udi_carrier field is not empty.
    fn has_udi_carrier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_reason field is not empty.
    fn has_status_reason(&self) -> bool;
    /// Returns true if the distinct_identifier field is present (Some).
    fn has_distinct_identifier(&self) -> bool;
    /// Returns true if the manufacturer field is present (Some).
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the manufacture_date field is present (Some).
    fn has_manufacture_date(&self) -> bool;
    /// Returns true if the expiration_date field is present (Some).
    fn has_expiration_date(&self) -> bool;
    /// Returns true if the lot_number field is present (Some).
    fn has_lot_number(&self) -> bool;
    /// Returns true if the serial_number field is present (Some).
    fn has_serial_number(&self) -> bool;
    /// Returns true if the device_name field is not empty.
    fn has_device_name(&self) -> bool;
    /// Returns true if the model_number field is present (Some).
    fn has_model_number(&self) -> bool;
    /// Returns true if the part_number field is present (Some).
    fn has_part_number(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the specialization field is not empty.
    fn has_specialization(&self) -> bool;
    /// Returns true if the version field is not empty.
    fn has_version(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the owner field is present (Some).
    fn has_owner(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the safety field is not empty.
    fn has_safety(&self) -> bool;
    /// Returns true if the parent field is present (Some).
    fn has_parent(&self) -> bool;
}
