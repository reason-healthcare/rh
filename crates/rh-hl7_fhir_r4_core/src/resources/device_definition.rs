use crate::bindings::device_nametype::DeviceNametype;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::prod_characteristic::ProdCharacteristic;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DeviceDefinition
///
/// The characteristics, operational status and capabilities of a medical-related component of a medical device.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DeviceDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Instance identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdideviceidentifier>>,
    /// Name of device manufacturer (string)
    #[serde(rename = "manufacturerString")]
    pub manufacturer_string: Option<StringType>,
    /// Name of device manufacturer (Reference)
    #[serde(rename = "manufacturerReference")]
    pub manufacturer_reference: Option<Reference>,
    /// A name given to the device to identify it
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDefinitionDevicename>>,
    /// The model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<StringType>,
    /// Extension element for the 'modelNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_modelNumber")]
    pub _model_number: Option<Element>,
    /// What kind of device or device system this is
    ///
    /// Binding: example (Type of device e.g. according to official classification.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/device-kind
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The capabilities supported on a  device, the standards to which the device conforms for a particular purpose, and used for the communication
    pub specialization: Option<Vec<DeviceDefinitionSpecialization>>,
    /// Available versions
    pub version: Option<Vec<StringType>>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Safety characteristics of the device
    ///
    /// Binding: example (No description)
    ///
    /// Available values:
    /// - `C106046`: Magnetic Resonance Conditional
    /// - `C106045`: Magnetic Resonance Safe
    /// - `C106047`: Magnetic Resonance Unsafe
    /// - `C113844`: Labeling does not Contain MRI Safety Information
    /// - `C101673`: Labeled as Containing Natural Rubber Latex
    /// - `C106038`: Not Made with Natural Rubber Latex
    pub safety: Option<Vec<CodeableConcept>>,
    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,
    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,
    /// Language code for the human-readable text strings produced by the device (all supported)
    #[serde(rename = "languageCode")]
    pub language_code: Option<Vec<CodeableConcept>>,
    /// Device capabilities
    pub capability: Option<Vec<DeviceDefinitionCapability>>,
    /// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
    pub property: Option<Vec<DeviceDefinitionProperty>>,
    /// Organization responsible for device
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,
    /// Network address to contact device
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Access to on-line information
    #[serde(rename = "onlineInformation")]
    pub online_information: Option<StringType>,
    /// Extension element for the 'onlineInformation' primitive field. Contains metadata and extensions.
    #[serde(rename = "_onlineInformation")]
    pub _online_information: Option<Element>,
    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,
    /// The quantity of the device present in the packaging (e.g. the number of devices present in a pack, or the number of devices in the same package of the medicinal product)
    pub quantity: Option<Quantity>,
    /// The parent device it can be part of
    #[serde(rename = "parentDevice")]
    pub parent_device: Option<Reference>,
    /// A substance used to create the material(s) of which the device is made
    pub material: Option<Vec<DeviceDefinitionMaterial>>,
}
/// DeviceDefinition nested structure for the 'material' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionMaterial {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The substance
    pub substance: CodeableConcept,
    /// Indicates an alternative material of the device
    pub alternate: Option<BooleanType>,
    /// Extension element for the 'alternate' primitive field. Contains metadata and extensions.
    pub _alternate: Option<Element>,
    /// Whether the substance is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<BooleanType>,
    /// Extension element for the 'allergenicIndicator' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allergenicIndicator")]
    pub _allergenic_indicator: Option<Element>,
}
/// DeviceDefinition nested structure for the 'capability' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionCapability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of capability
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Description of capability
    pub description: Option<Vec<CodeableConcept>>,
}
/// DeviceDefinition nested structure for the 'deviceName' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionDevicename {
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
/// DeviceDefinition nested structure for the 'specialization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionSpecialization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The standard that is used to operate and communicate
    #[serde(rename = "systemType")]
    pub system_type: StringType,
    /// Extension element for the 'systemType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_systemType")]
    pub _system_type: Option<Element>,
    /// The version of the standard that is used to operate and communicate
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
}
/// DeviceDefinition nested structure for the 'udiDeviceIdentifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionUdideviceidentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdication porvided in the DeviceDefinition.udiDeviceIdentifier
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: StringType,
    /// Extension element for the 'deviceIdentifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_deviceIdentifier")]
    pub _device_identifier: Option<Element>,
    /// The organization that assigns the identifier algorithm
    pub issuer: StringType,
    /// Extension element for the 'issuer' primitive field. Contains metadata and extensions.
    pub _issuer: Option<Element>,
    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: StringType,
    /// Extension element for the 'jurisdiction' primitive field. Contains metadata and extensions.
    pub _jurisdiction: Option<Element>,
}
/// DeviceDefinition nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDefinitionProperty {
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

impl Default for DeviceDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            udi_device_identifier: Default::default(),
            manufacturer_string: Default::default(),
            manufacturer_reference: Default::default(),
            device_name: Default::default(),
            model_number: Default::default(),
            _model_number: Default::default(),
            type_: Default::default(),
            specialization: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            safety: Default::default(),
            shelf_life_storage: Default::default(),
            physical_characteristics: Default::default(),
            language_code: Default::default(),
            capability: Default::default(),
            property: Default::default(),
            owner: Default::default(),
            contact: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            online_information: Default::default(),
            _online_information: Default::default(),
            note: Default::default(),
            quantity: Default::default(),
            parent_device: Default::default(),
            material: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionMaterial {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            substance: CodeableConcept::default(),
            alternate: Default::default(),
            _alternate: Default::default(),
            allergenic_indicator: Default::default(),
            _allergenic_indicator: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionCapability {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            description: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionDevicename {
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

impl Default for DeviceDefinitionSpecialization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            system_type: StringType::default(),
            _system_type: Default::default(),
            version: Default::default(),
            _version: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionUdideviceidentifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            device_identifier: Default::default(),
            _device_identifier: Default::default(),
            issuer: Default::default(),
            _issuer: Default::default(),
            jurisdiction: Default::default(),
            _jurisdiction: Default::default(),
        }
    }
}

impl Default for DeviceDefinitionProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_quantity: Default::default(),
            value_code: Default::default(),
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DeviceDefinition {
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

impl crate::traits::resource::ResourceMutators for DeviceDefinition {
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

impl crate::traits::resource::ResourceExistence for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for DeviceDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for DeviceDefinition {
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

impl crate::traits::device_definition::DeviceDefinitionAccessors for DeviceDefinition {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn udi_device_identifier(&self) -> &[DeviceDefinitionUdideviceidentifier] {
        self.udi_device_identifier.as_deref().unwrap_or(&[])
    }
    fn device_name(&self) -> &[DeviceDefinitionDevicename] {
        self.device_name.as_deref().unwrap_or(&[])
    }
    fn model_number(&self) -> Option<StringType> {
        self.model_number.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn specialization(&self) -> &[DeviceDefinitionSpecialization] {
        self.specialization.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> &[StringType] {
        self.version.as_deref().unwrap_or(&[])
    }
    fn safety(&self) -> &[CodeableConcept] {
        self.safety.as_deref().unwrap_or(&[])
    }
    fn shelf_life_storage(&self) -> &[ProductShelfLife] {
        self.shelf_life_storage.as_deref().unwrap_or(&[])
    }
    fn physical_characteristics(&self) -> Option<ProdCharacteristic> {
        self.physical_characteristics.clone()
    }
    fn language_code(&self) -> &[CodeableConcept] {
        self.language_code.as_deref().unwrap_or(&[])
    }
    fn capability(&self) -> &[DeviceDefinitionCapability] {
        self.capability.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[DeviceDefinitionProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn contact(&self) -> &[ContactPoint] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn online_information(&self) -> Option<StringType> {
        self.online_information.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn parent_device(&self) -> Option<Reference> {
        self.parent_device.clone()
    }
    fn material(&self) -> &[DeviceDefinitionMaterial] {
        self.material.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::device_definition::DeviceDefinitionMutators for DeviceDefinition {
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
    fn set_udi_device_identifier(self, value: Vec<DeviceDefinitionUdideviceidentifier>) -> Self {
        let mut resource = self.clone();
        resource.udi_device_identifier = Some(value);
        resource
    }
    fn add_udi_device_identifier(self, item: DeviceDefinitionUdideviceidentifier) -> Self {
        let mut resource = self.clone();
        resource
            .udi_device_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_device_name(self, value: Vec<DeviceDefinitionDevicename>) -> Self {
        let mut resource = self.clone();
        resource.device_name = Some(value);
        resource
    }
    fn add_device_name(self, item: DeviceDefinitionDevicename) -> Self {
        let mut resource = self.clone();
        resource.device_name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_model_number(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.model_number = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_specialization(self, value: Vec<DeviceDefinitionSpecialization>) -> Self {
        let mut resource = self.clone();
        resource.specialization = Some(value);
        resource
    }
    fn add_specialization(self, item: DeviceDefinitionSpecialization) -> Self {
        let mut resource = self.clone();
        resource
            .specialization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_version(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn add_version(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.version.get_or_insert_with(Vec::new).push(item);
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
    fn set_shelf_life_storage(self, value: Vec<ProductShelfLife>) -> Self {
        let mut resource = self.clone();
        resource.shelf_life_storage = Some(value);
        resource
    }
    fn add_shelf_life_storage(self, item: ProductShelfLife) -> Self {
        let mut resource = self.clone();
        resource
            .shelf_life_storage
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_physical_characteristics(self, value: ProdCharacteristic) -> Self {
        let mut resource = self.clone();
        resource.physical_characteristics = Some(value);
        resource
    }
    fn set_language_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.language_code = Some(value);
        resource
    }
    fn add_language_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .language_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_capability(self, value: Vec<DeviceDefinitionCapability>) -> Self {
        let mut resource = self.clone();
        resource.capability = Some(value);
        resource
    }
    fn add_capability(self, item: DeviceDefinitionCapability) -> Self {
        let mut resource = self.clone();
        resource.capability.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<DeviceDefinitionProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: DeviceDefinitionProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
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
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_online_information(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.online_information = Some(value);
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
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_parent_device(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent_device = Some(value);
        resource
    }
    fn set_material(self, value: Vec<DeviceDefinitionMaterial>) -> Self {
        let mut resource = self.clone();
        resource.material = Some(value);
        resource
    }
    fn add_material(self, item: DeviceDefinitionMaterial) -> Self {
        let mut resource = self.clone();
        resource.material.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::device_definition::DeviceDefinitionExistence for DeviceDefinition {
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
    fn has_manufacturer(&self) -> bool {
        self.manufacturer_string.is_some() || self.manufacturer_reference.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_udi_device_identifier(&self) -> bool {
        self.udi_device_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_device_name(&self) -> bool {
        self.device_name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_model_number(&self) -> bool {
        self.model_number.is_some()
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
    fn has_safety(&self) -> bool {
        self.safety.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_shelf_life_storage(&self) -> bool {
        self.shelf_life_storage
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_physical_characteristics(&self) -> bool {
        self.physical_characteristics.is_some()
    }
    fn has_language_code(&self) -> bool {
        self.language_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_capability(&self) -> bool {
        self.capability.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_online_information(&self) -> bool {
        self.online_information.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_parent_device(&self) -> bool {
        self.parent_device.is_some()
    }
    fn has_material(&self) -> bool {
        self.material.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DeviceDefinition {
    fn resource_type(&self) -> &'static str {
        "DeviceDefinition"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/DeviceDefinition")
    }
}
