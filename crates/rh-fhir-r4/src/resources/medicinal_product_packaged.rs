use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::prod_characteristic::ProdCharacteristic;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// MedicinalProductPackaged
///
/// A medicinal product in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPackaged
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductPackaged
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPackaged {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique identifier
    pub identifier: Option<Vec<Identifier>>,
    /// The product with this is a pack for
    pub subject: Option<Vec<Reference>>,
    /// Textual description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The legal status of supply of the medicinal product as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,
    /// Marketing information
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// Manufacturer of this Package Item
    #[serde(rename = "marketingAuthorization")]
    pub marketing_authorization: Option<Reference>,
    /// Manufacturer of this Package Item
    pub manufacturer: Option<Vec<Reference>>,
    /// Batch numbering
    #[serde(rename = "batchIdentifier")]
    pub batch_identifier: Option<Vec<MedicinalProductPackagedBatchidentifier>>,
    /// A packaging item, as a contained for medicine, possibly with other packaging items within
    #[serde(rename = "packageItem")]
    pub package_item: Vec<MedicinalProductPackagedPackageitem>,
}
/// MedicinalProductPackaged nested structure for the 'batchIdentifier' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPackagedBatchidentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A number appearing on the outer packaging of a specific batch
    #[serde(rename = "outerPackaging")]
    pub outer_packaging: Identifier,
    /// A number appearing on the immediate packaging (and not the outer packaging)
    #[serde(rename = "immediatePackaging")]
    pub immediate_packaging: Option<Identifier>,
}
/// MedicinalProductPackaged nested structure for the 'packageItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicinalProductPackagedPackageitem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Including possibly Data Carrier Identifier
    pub identifier: Option<Vec<Identifier>>,
    /// The physical type of the container of the medicine
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The quantity of this package in the medicinal product, at the current level of packaging. The outermost is always 1
    pub quantity: Quantity,
    /// Material type of the package item
    pub material: Option<Vec<CodeableConcept>>,
    /// A possible alternate material for the packaging
    #[serde(rename = "alternateMaterial")]
    pub alternate_material: Option<Vec<CodeableConcept>>,
    /// A device accompanying a medicinal product
    pub device: Option<Vec<Reference>>,
    /// The manufactured item as contained in the packaged medicinal product
    #[serde(rename = "manufacturedItem")]
    pub manufactured_item: Option<Vec<Reference>>,
    /// Allows containers within containers
    #[serde(rename = "packageItem")]
    pub package_item: Option<Vec<StringType>>,
    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,
    /// Other codeable characteristics
    #[serde(rename = "otherCharacteristics")]
    pub other_characteristics: Option<Vec<CodeableConcept>>,
    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,
    /// Manufacturer of this Package Item
    pub manufacturer: Option<Vec<Reference>>,
}

impl Default for MedicinalProductPackaged {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            subject: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            legal_status_of_supply: Default::default(),
            marketing_status: Default::default(),
            marketing_authorization: Default::default(),
            manufacturer: Default::default(),
            batch_identifier: Default::default(),
            package_item: Vec::new(),
        }
    }
}

impl Default for MedicinalProductPackagedBatchidentifier {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            outer_packaging: Default::default(),
            immediate_packaging: Default::default(),
        }
    }
}

impl Default for MedicinalProductPackagedPackageitem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            type_: Default::default(),
            quantity: Default::default(),
            material: Default::default(),
            alternate_material: Default::default(),
            device: Default::default(),
            manufactured_item: Default::default(),
            package_item: Default::default(),
            physical_characteristics: Default::default(),
            other_characteristics: Default::default(),
            shelf_life_storage: Default::default(),
            manufacturer: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for MedicinalProductPackaged {
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

impl crate::traits::resource::ResourceMutators for MedicinalProductPackaged {
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

impl crate::traits::resource::ResourceExistence for MedicinalProductPackaged {
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

impl crate::traits::domain_resource::DomainResourceAccessors for MedicinalProductPackaged {
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

impl crate::traits::domain_resource::DomainResourceMutators for MedicinalProductPackaged {
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

impl crate::traits::domain_resource::DomainResourceExistence for MedicinalProductPackaged {
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

impl crate::traits::medicinal_product_packaged::MedicinalProductPackagedAccessors
    for MedicinalProductPackaged
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn legal_status_of_supply(&self) -> Option<CodeableConcept> {
        self.legal_status_of_supply.clone()
    }
    fn marketing_status(&self) -> &[MarketingStatus] {
        self.marketing_status.as_deref().unwrap_or(&[])
    }
    fn marketing_authorization(&self) -> Option<Reference> {
        self.marketing_authorization.clone()
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn batch_identifier(&self) -> &[MedicinalProductPackagedBatchidentifier] {
        self.batch_identifier.as_deref().unwrap_or(&[])
    }
    fn package_item(&self) -> &[MedicinalProductPackagedPackageitem] {
        &self.package_item
    }
}

impl crate::traits::medicinal_product_packaged::MedicinalProductPackagedMutators
    for MedicinalProductPackaged
{
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
    fn set_subject(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn add_subject(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_legal_status_of_supply(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.legal_status_of_supply = Some(value);
        resource
    }
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self {
        let mut resource = self.clone();
        resource.marketing_status = Some(value);
        resource
    }
    fn add_marketing_status(self, item: MarketingStatus) -> Self {
        let mut resource = self.clone();
        resource
            .marketing_status
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_marketing_authorization(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.marketing_authorization = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = Some(value);
        resource
    }
    fn add_manufacturer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .manufacturer
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_batch_identifier(self, value: Vec<MedicinalProductPackagedBatchidentifier>) -> Self {
        let mut resource = self.clone();
        resource.batch_identifier = Some(value);
        resource
    }
    fn add_batch_identifier(self, item: MedicinalProductPackagedBatchidentifier) -> Self {
        let mut resource = self.clone();
        resource
            .batch_identifier
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_package_item(self, value: Vec<MedicinalProductPackagedPackageitem>) -> Self {
        let mut resource = self.clone();
        resource.package_item = value;
        resource
    }
    fn add_package_item(self, item: MedicinalProductPackagedPackageitem) -> Self {
        let mut resource = self.clone();
        resource.package_item.push(item);
        resource
    }
}

impl crate::traits::medicinal_product_packaged::MedicinalProductPackagedExistence
    for MedicinalProductPackaged
{
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
    fn has_subject(&self) -> bool {
        self.subject.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_legal_status_of_supply(&self) -> bool {
        self.legal_status_of_supply.is_some()
    }
    fn has_marketing_status(&self) -> bool {
        self.marketing_status
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_marketing_authorization(&self) -> bool {
        self.marketing_authorization.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_batch_identifier(&self) -> bool {
        self.batch_identifier
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_package_item(&self) -> bool {
        !self.package_item.is_empty()
    }
}
