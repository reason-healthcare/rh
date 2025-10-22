use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::prod_characteristic::ProdCharacteristic;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::device_definition::DeviceDefinitionCapability;
use crate::resources::device_definition::DeviceDefinitionDevicename;
use crate::resources::device_definition::DeviceDefinitionMaterial;
use crate::resources::device_definition::DeviceDefinitionProperty;
use crate::resources::device_definition::DeviceDefinitionSpecialization;
use crate::resources::device_definition::DeviceDefinitionUdideviceidentifier;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The characteristics, operational status and capabilities of a medical-related component of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the udiDeviceIdentifier field.
    fn udi_device_identifier(&self) -> &[DeviceDefinitionUdideviceidentifier];
    /// Returns a reference to the deviceName field.
    fn device_name(&self) -> &[DeviceDefinitionDevicename];
    /// Returns a reference to the modelNumber field.
    fn model_number(&self) -> Option<StringType>;
    /// Returns a reference to the type field.
    fn type_(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specialization field.
    fn specialization(&self) -> &[DeviceDefinitionSpecialization];
    /// Returns a reference to the version field.
    fn version(&self) -> &[StringType];
    /// Returns a reference to the safety field.
    fn safety(&self) -> &[CodeableConcept];
    /// Returns a reference to the shelfLifeStorage field.
    fn shelf_life_storage(&self) -> &[ProductShelfLife];
    /// Returns a reference to the physicalCharacteristics field.
    fn physical_characteristics(&self) -> Option<ProdCharacteristic>;
    /// Returns a reference to the languageCode field.
    fn language_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the capability field.
    fn capability(&self) -> &[DeviceDefinitionCapability];
    /// Returns a reference to the property field.
    fn property(&self) -> &[DeviceDefinitionProperty];
    /// Returns a reference to the owner field.
    fn owner(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the onlineInformation field.
    fn online_information(&self) -> Option<StringType>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the parentDevice field.
    fn parent_device(&self) -> Option<Reference>;
    /// Returns a reference to the material field.
    fn material(&self) -> &[DeviceDefinitionMaterial];
}
/// DeviceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// The characteristics, operational status and capabilities of a medical-related component of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDefinitionMutators: DomainResourceMutators {
    /// Create a new DeviceDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::device_definition::DeviceDefinition;
    /// use hl7_fhir_r4_core::traits::device_definition::DeviceDefinitionMutators;
    ///
    /// let resource = DeviceDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the udiDeviceIdentifier field and returns self for chaining.
    fn set_udi_device_identifier(self, value: Vec<DeviceDefinitionUdideviceidentifier>) -> Self;
    /// Adds an item to the udiDeviceIdentifier field and returns self for chaining.
    fn add_udi_device_identifier(self, item: DeviceDefinitionUdideviceidentifier) -> Self;
    /// Sets the deviceName field and returns self for chaining.
    fn set_device_name(self, value: Vec<DeviceDefinitionDevicename>) -> Self;
    /// Adds an item to the deviceName field and returns self for chaining.
    fn add_device_name(self, item: DeviceDefinitionDevicename) -> Self;
    /// Sets the modelNumber field and returns self for chaining.
    fn set_model_number(self, value: String) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: CodeableConcept) -> Self;
    /// Sets the specialization field and returns self for chaining.
    fn set_specialization(self, value: Vec<DeviceDefinitionSpecialization>) -> Self;
    /// Adds an item to the specialization field and returns self for chaining.
    fn add_specialization(self, item: DeviceDefinitionSpecialization) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: Vec<String>) -> Self;
    /// Adds an item to the version field and returns self for chaining.
    fn add_version(self, item: String) -> Self;
    /// Sets the safety field and returns self for chaining.
    fn set_safety(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the safety field and returns self for chaining.
    fn add_safety(self, item: CodeableConcept) -> Self;
    /// Sets the shelfLifeStorage field and returns self for chaining.
    fn set_shelf_life_storage(self, value: Vec<ProductShelfLife>) -> Self;
    /// Adds an item to the shelfLifeStorage field and returns self for chaining.
    fn add_shelf_life_storage(self, item: ProductShelfLife) -> Self;
    /// Sets the physicalCharacteristics field and returns self for chaining.
    fn set_physical_characteristics(self, value: ProdCharacteristic) -> Self;
    /// Sets the languageCode field and returns self for chaining.
    fn set_language_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the languageCode field and returns self for chaining.
    fn add_language_code(self, item: CodeableConcept) -> Self;
    /// Sets the capability field and returns self for chaining.
    fn set_capability(self, value: Vec<DeviceDefinitionCapability>) -> Self;
    /// Adds an item to the capability field and returns self for chaining.
    fn add_capability(self, item: DeviceDefinitionCapability) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<DeviceDefinitionProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: DeviceDefinitionProperty) -> Self;
    /// Sets the owner field and returns self for chaining.
    fn set_owner(self, value: Reference) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactPoint>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactPoint) -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the onlineInformation field and returns self for chaining.
    fn set_online_information(self, value: String) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the parentDevice field and returns self for chaining.
    fn set_parent_device(self, value: Reference) -> Self;
    /// Sets the material field and returns self for chaining.
    fn set_material(self, value: Vec<DeviceDefinitionMaterial>) -> Self;
    /// Adds an item to the material field and returns self for chaining.
    fn add_material(self, item: DeviceDefinitionMaterial) -> Self;
}
/// DeviceDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// The characteristics, operational status and capabilities of a medical-related component of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the udi_device_identifier field is not empty.
    fn has_udi_device_identifier(&self) -> bool;
    /// Returns true if the manufacturer field is present (Some).
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the device_name field is not empty.
    fn has_device_name(&self) -> bool;
    /// Returns true if the model_number field is present (Some).
    fn has_model_number(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the specialization field is not empty.
    fn has_specialization(&self) -> bool;
    /// Returns true if the version field is not empty.
    fn has_version(&self) -> bool;
    /// Returns true if the safety field is not empty.
    fn has_safety(&self) -> bool;
    /// Returns true if the shelf_life_storage field is not empty.
    fn has_shelf_life_storage(&self) -> bool;
    /// Returns true if the physical_characteristics field is present (Some).
    fn has_physical_characteristics(&self) -> bool;
    /// Returns true if the language_code field is not empty.
    fn has_language_code(&self) -> bool;
    /// Returns true if the capability field is not empty.
    fn has_capability(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the owner field is present (Some).
    fn has_owner(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the online_information field is present (Some).
    fn has_online_information(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the parent_device field is present (Some).
    fn has_parent_device(&self) -> bool;
    /// Returns true if the material field is not empty.
    fn has_material(&self) -> bool;
}
