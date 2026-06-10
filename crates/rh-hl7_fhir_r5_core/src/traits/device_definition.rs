use crate::bindings::device_productidentifierinudi::DeviceProductidentifierinudi;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::device_definition::DeviceDefinitionChargeitem;
use crate::resources::device_definition::DeviceDefinitionClassification;
use crate::resources::device_definition::DeviceDefinitionConformsto;
use crate::resources::device_definition::DeviceDefinitionCorrectiveaction;
use crate::resources::device_definition::DeviceDefinitionDevicename;
use crate::resources::device_definition::DeviceDefinitionGuideline;
use crate::resources::device_definition::DeviceDefinitionHaspart;
use crate::resources::device_definition::DeviceDefinitionLink;
use crate::resources::device_definition::DeviceDefinitionMaterial;
use crate::resources::device_definition::DeviceDefinitionPackaging;
use crate::resources::device_definition::DeviceDefinitionProperty;
use crate::resources::device_definition::DeviceDefinitionRegulatoryidentifier;
use crate::resources::device_definition::DeviceDefinitionUdideviceidentifier;
use crate::resources::device_definition::DeviceDefinitionVersion;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// DeviceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This is a specialized resource that defines the characteristics and capabilities of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the udiDeviceIdentifier field.
    fn udi_device_identifier(&self) -> &[DeviceDefinitionUdideviceidentifier];
    /// Returns a reference to the regulatoryIdentifier field.
    fn regulatory_identifier(&self) -> &[DeviceDefinitionRegulatoryidentifier];
    /// Returns a reference to the partNumber field.
    fn part_number(&self) -> Option<StringType>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> Option<Reference>;
    /// Returns a reference to the deviceName field.
    fn device_name(&self) -> &[DeviceDefinitionDevicename];
    /// Returns a reference to the modelNumber field.
    fn model_number(&self) -> Option<StringType>;
    /// Returns a reference to the classification field.
    fn classification(&self) -> &[DeviceDefinitionClassification];
    /// Returns a reference to the conformsTo field.
    fn conforms_to(&self) -> &[DeviceDefinitionConformsto];
    /// Returns a reference to the hasPart field.
    fn has_part(&self) -> &[DeviceDefinitionHaspart];
    /// Returns a reference to the packaging field.
    fn packaging(&self) -> &[DeviceDefinitionPackaging];
    /// Returns a reference to the version field.
    fn version(&self) -> &[DeviceDefinitionVersion];
    /// Returns a reference to the safety field.
    fn safety(&self) -> &[CodeableConcept];
    /// Returns a reference to the shelfLifeStorage field.
    fn shelf_life_storage(&self) -> &[ProductShelfLife];
    /// Returns a reference to the languageCode field.
    fn language_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the property field.
    fn property(&self) -> &[DeviceDefinitionProperty];
    /// Returns a reference to the owner field.
    fn owner(&self) -> Option<Reference>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactPoint];
    /// Returns a reference to the link field.
    fn link(&self) -> &[DeviceDefinitionLink];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the material field.
    fn material(&self) -> &[DeviceDefinitionMaterial];
    /// Returns a reference to the productionIdentifierInUDI field.
    fn production_identifier_in_u_d_i(&self) -> &[DeviceProductidentifierinudi];
    /// Returns a reference to the guideline field.
    fn guideline(&self) -> Option<DeviceDefinitionGuideline>;
    /// Returns a reference to the correctiveAction field.
    fn corrective_action(&self) -> Option<DeviceDefinitionCorrectiveaction>;
    /// Returns a reference to the chargeItem field.
    fn charge_item(&self) -> &[DeviceDefinitionChargeitem];
}
/// DeviceDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This is a specialized resource that defines the characteristics and capabilities of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::device_definition::DeviceDefinition;
    /// use rh_hl7_fhir_r5_core::traits::device_definition::DeviceDefinitionMutators;
    ///
    /// let resource = DeviceDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the udiDeviceIdentifier field and returns self for chaining.
    fn set_udi_device_identifier(self, value: Vec<DeviceDefinitionUdideviceidentifier>) -> Self;
    /// Adds an item to the udiDeviceIdentifier field and returns self for chaining.
    fn add_udi_device_identifier(self, item: DeviceDefinitionUdideviceidentifier) -> Self;
    /// Sets the regulatoryIdentifier field and returns self for chaining.
    fn set_regulatory_identifier(self, value: Vec<DeviceDefinitionRegulatoryidentifier>) -> Self;
    /// Adds an item to the regulatoryIdentifier field and returns self for chaining.
    fn add_regulatory_identifier(self, item: DeviceDefinitionRegulatoryidentifier) -> Self;
    /// Sets the partNumber field and returns self for chaining.
    fn set_part_number(self, value: String) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Reference) -> Self;
    /// Sets the deviceName field and returns self for chaining.
    fn set_device_name(self, value: Vec<DeviceDefinitionDevicename>) -> Self;
    /// Adds an item to the deviceName field and returns self for chaining.
    fn add_device_name(self, item: DeviceDefinitionDevicename) -> Self;
    /// Sets the modelNumber field and returns self for chaining.
    fn set_model_number(self, value: String) -> Self;
    /// Sets the classification field and returns self for chaining.
    fn set_classification(self, value: Vec<DeviceDefinitionClassification>) -> Self;
    /// Adds an item to the classification field and returns self for chaining.
    fn add_classification(self, item: DeviceDefinitionClassification) -> Self;
    /// Sets the conformsTo field and returns self for chaining.
    fn set_conforms_to(self, value: Vec<DeviceDefinitionConformsto>) -> Self;
    /// Adds an item to the conformsTo field and returns self for chaining.
    fn add_conforms_to(self, item: DeviceDefinitionConformsto) -> Self;
    /// Sets the hasPart field and returns self for chaining.
    fn set_has_part(self, value: Vec<DeviceDefinitionHaspart>) -> Self;
    /// Adds an item to the hasPart field and returns self for chaining.
    fn add_has_part(self, item: DeviceDefinitionHaspart) -> Self;
    /// Sets the packaging field and returns self for chaining.
    fn set_packaging(self, value: Vec<DeviceDefinitionPackaging>) -> Self;
    /// Adds an item to the packaging field and returns self for chaining.
    fn add_packaging(self, item: DeviceDefinitionPackaging) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: Vec<DeviceDefinitionVersion>) -> Self;
    /// Adds an item to the version field and returns self for chaining.
    fn add_version(self, item: DeviceDefinitionVersion) -> Self;
    /// Sets the safety field and returns self for chaining.
    fn set_safety(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the safety field and returns self for chaining.
    fn add_safety(self, item: CodeableConcept) -> Self;
    /// Sets the shelfLifeStorage field and returns self for chaining.
    fn set_shelf_life_storage(self, value: Vec<ProductShelfLife>) -> Self;
    /// Adds an item to the shelfLifeStorage field and returns self for chaining.
    fn add_shelf_life_storage(self, item: ProductShelfLife) -> Self;
    /// Sets the languageCode field and returns self for chaining.
    fn set_language_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the languageCode field and returns self for chaining.
    fn add_language_code(self, item: CodeableConcept) -> Self;
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
    /// Sets the link field and returns self for chaining.
    fn set_link(self, value: Vec<DeviceDefinitionLink>) -> Self;
    /// Adds an item to the link field and returns self for chaining.
    fn add_link(self, item: DeviceDefinitionLink) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the material field and returns self for chaining.
    fn set_material(self, value: Vec<DeviceDefinitionMaterial>) -> Self;
    /// Adds an item to the material field and returns self for chaining.
    fn add_material(self, item: DeviceDefinitionMaterial) -> Self;
    /// Sets the productionIdentifierInUDI field and returns self for chaining.
    fn set_production_identifier_in_u_d_i(self, value: Vec<DeviceProductidentifierinudi>) -> Self;
    /// Adds an item to the productionIdentifierInUDI field and returns self for chaining.
    fn add_production_identifier_in_u_d_i(self, item: DeviceProductidentifierinudi) -> Self;
    /// Sets the guideline field and returns self for chaining.
    fn set_guideline(self, value: DeviceDefinitionGuideline) -> Self;
    /// Sets the correctiveAction field and returns self for chaining.
    fn set_corrective_action(self, value: DeviceDefinitionCorrectiveaction) -> Self;
    /// Sets the chargeItem field and returns self for chaining.
    fn set_charge_item(self, value: Vec<DeviceDefinitionChargeitem>) -> Self;
    /// Adds an item to the chargeItem field and returns self for chaining.
    fn add_charge_item(self, item: DeviceDefinitionChargeitem) -> Self;
}
/// DeviceDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This is a specialized resource that defines the characteristics and capabilities of a device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait DeviceDefinitionExistence: DomainResourceExistence {
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the udi_device_identifier field is not empty.
    fn has_udi_device_identifier(&self) -> bool;
    /// Returns true if the regulatory_identifier field is not empty.
    fn has_regulatory_identifier(&self) -> bool;
    /// Returns true if the part_number field is present (Some).
    fn has_part_number(&self) -> bool;
    /// Returns true if the manufacturer field is present (Some).
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the device_name field is not empty.
    fn has_device_name(&self) -> bool;
    /// Returns true if the model_number field is present (Some).
    fn has_model_number(&self) -> bool;
    /// Returns true if the classification field is not empty.
    fn has_classification(&self) -> bool;
    /// Returns true if the conforms_to field is not empty.
    fn has_conforms_to(&self) -> bool;
    /// Returns true if the has_part field is not empty.
    fn has_has_part(&self) -> bool;
    /// Returns true if the packaging field is not empty.
    fn has_packaging(&self) -> bool;
    /// Returns true if the version field is not empty.
    fn has_version(&self) -> bool;
    /// Returns true if the safety field is not empty.
    fn has_safety(&self) -> bool;
    /// Returns true if the shelf_life_storage field is not empty.
    fn has_shelf_life_storage(&self) -> bool;
    /// Returns true if the language_code field is not empty.
    fn has_language_code(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
    /// Returns true if the owner field is present (Some).
    fn has_owner(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the link field is not empty.
    fn has_link(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the material field is not empty.
    fn has_material(&self) -> bool;
    /// Returns true if the production_identifier_in_u_d_i field is not empty.
    fn has_production_identifier_in_u_d_i(&self) -> bool;
    /// Returns true if the guideline field is present (Some).
    fn has_guideline(&self) -> bool;
    /// Returns true if the corrective_action field is present (Some).
    fn has_corrective_action(&self) -> bool;
    /// Returns true if the charge_item field is not empty.
    fn has_charge_item(&self) -> bool;
}
