use crate::bindings::inventoryitem_status::InventoryitemStatus;
use crate::datatypes::address::Address;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// InventoryItem
///
/// functional description of an inventory item used in inventory and supply-related workflows.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: InventoryItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for the inventory item
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error | unknown
    pub status: InventoryitemStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Category or class of the item
    pub category: Option<Vec<CodeableConcept>>,
    /// Code designating the specific type of item
    pub code: Option<Vec<CodeableConcept>>,
    /// The item name(s) - the brand name, or common name, functional name, generic name or others
    pub name: Option<Vec<InventoryItemName>>,
    /// Organization(s) responsible for the product
    #[serde(rename = "responsibleOrganization")]
    pub responsible_organization: Option<Vec<InventoryItemResponsibleorganization>>,
    /// Descriptive characteristics of the item
    pub description: Option<InventoryItemDescription>,
    /// The usage status like recalled, in use, discarded
    #[serde(rename = "inventoryStatus")]
    pub inventory_status: Option<Vec<CodeableConcept>>,
    /// The base unit of measure - the unit in which the product is used or counted
    #[serde(rename = "baseUnit")]
    pub base_unit: Option<CodeableConcept>,
    /// Net content or amount present in the item
    #[serde(rename = "netContent")]
    pub net_content: Option<Quantity>,
    /// Association with other items or products
    pub association: Option<Vec<InventoryItemAssociation>>,
    /// Characteristic of the item
    pub characteristic: Option<Vec<InventoryItemCharacteristic>>,
    /// Instances or occurrences of the product
    pub instance: Option<InventoryItemInstance>,
    /// Link to a product resource used in clinical workflows
    #[serde(rename = "productReference")]
    pub product_reference: Option<Reference>,
}
/// InventoryItem nested structure for the 'description' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemDescription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The language that is used in the item description
    pub language: Option<StringType>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Textual description of the item
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// InventoryItem nested structure for the 'responsibleOrganization' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemResponsibleorganization {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The role of the organization e.g. manufacturer, distributor, or other
    pub role: CodeableConcept,
    /// An organization that is associated with the item
    pub organization: Reference,
}
/// InventoryItem nested structure for the 'association' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemAssociation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of association between the device and the other item
    #[serde(rename = "associationType")]
    pub association_type: CodeableConcept,
    /// The related item or product
    #[serde(rename = "relatedItem")]
    pub related_item: Reference,
    /// The quantity of the product in this product
    pub quantity: Ratio,
}
/// InventoryItem nested structure for the 'name' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'
    ///
    /// Binding: preferred (Name types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/inventoryitem-nametype
    #[serde(rename = "nameType")]
    pub name_type: Coding,
    /// The language used to express the item name
    pub language: StringType,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// The name or designation of the item
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
}
/// InventoryItem nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The identifier for the physical instance, typically a serial number
    pub identifier: Option<Vec<Identifier>>,
    /// The lot or batch number of the item
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// The expiry date or date and time for the product
    pub expiry: Option<DateTimeType>,
    /// Extension element for the 'expiry' primitive field. Contains metadata and extensions.
    pub _expiry: Option<Element>,
    /// The subject that the item is associated with
    pub subject: Option<Reference>,
    /// The location that the item is associated with
    pub location: Option<Reference>,
}
/// InventoryItem nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The characteristic that is being defined
    #[serde(rename = "characteristicType")]
    pub characteristic_type: CodeableConcept,
    /// The value of the attribute (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// The value of the attribute (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// The value of the attribute (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// The value of the attribute (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// The value of the attribute (url)
    #[serde(rename = "valueUrl")]
    pub value_url: StringType,
    /// The value of the attribute (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// The value of the attribute (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// The value of the attribute (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// The value of the attribute (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// The value of the attribute (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,
    /// The value of the attribute (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Address,
    /// The value of the attribute (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,
    /// The value of the attribute (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
}

impl Default for InventoryItem {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: InventoryitemStatus::default(),
            _status: Default::default(),
            category: Default::default(),
            code: Default::default(),
            name: Default::default(),
            responsible_organization: Default::default(),
            description: Default::default(),
            inventory_status: Default::default(),
            base_unit: Default::default(),
            net_content: Default::default(),
            association: Default::default(),
            characteristic: Default::default(),
            instance: Default::default(),
            product_reference: Default::default(),
        }
    }
}

impl Default for InventoryItemDescription {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            language: Default::default(),
            _language: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for InventoryItemResponsibleorganization {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            role: Default::default(),
            organization: Default::default(),
        }
    }
}

impl Default for InventoryItemAssociation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            association_type: CodeableConcept::default(),
            related_item: Reference::default(),
            quantity: Ratio::default(),
        }
    }
}

impl Default for InventoryItemName {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name_type: Coding::default(),
            language: StringType::default(),
            _language: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
        }
    }
}

impl Default for InventoryItemInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            expiry: Default::default(),
            _expiry: Default::default(),
            subject: Default::default(),
            location: Default::default(),
        }
    }
}

impl Default for InventoryItemCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            characteristic_type: CodeableConcept::default(),
            value_string: Default::default(),
            value_integer: Default::default(),
            value_decimal: Default::default(),
            value_boolean: Default::default(),
            value_url: Default::default(),
            value_date_time: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_annotation: Default::default(),
            value_address: Default::default(),
            value_duration: Default::default(),
            value_codeable_concept: Default::default(),
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
                "InventoryItem.description.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/languages|5.0.0",
            )
            .with_description("Description languages."),
            rh_foundation::ElementBinding::new(
                "InventoryItem.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "InventoryItem.name.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/languages|5.0.0",
            )
            .with_description("Name languages."),
            rh_foundation::ElementBinding::new(
                "InventoryItem.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/inventoryitem-status|5.0.0",
            )
            .with_description("Status of the inventory item."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("InventoryItem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.contained", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.extension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.category", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.code", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.name", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.name.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.name.extension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.name.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.name.nameType", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.name.language", 1, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.name.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization.role",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.responsibleOrganization.organization",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryItem.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.description.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.description.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.description.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.description.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.description.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryItem.inventoryStatus", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.baseUnit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.netContent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.association", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.association.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.association.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.association.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.association.associationType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.association.relatedItem",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.association.quantity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryItem.characteristic", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.characteristic.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.characteristic.characteristicType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.characteristic.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("InventoryItem.instance", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "InventoryItem.instance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.identifier", 0, None),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.lotNumber", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.expiry", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.instance.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("InventoryItem.productReference", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for InventoryItem {
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

impl crate::traits::resource::ResourceMutators for InventoryItem {
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

impl crate::traits::resource::ResourceExistence for InventoryItem {
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

impl crate::traits::domain_resource::DomainResourceAccessors for InventoryItem {
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

impl crate::traits::domain_resource::DomainResourceMutators for InventoryItem {
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

impl crate::traits::domain_resource::DomainResourceExistence for InventoryItem {
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

impl crate::traits::inventory_item::InventoryItemAccessors for InventoryItem {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> InventoryitemStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn code(&self) -> &[CodeableConcept] {
        self.code.as_deref().unwrap_or(&[])
    }
    fn name(&self) -> &[InventoryItemName] {
        self.name.as_deref().unwrap_or(&[])
    }
    fn responsible_organization(&self) -> &[InventoryItemResponsibleorganization] {
        self.responsible_organization.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<InventoryItemDescription> {
        self.description.clone()
    }
    fn inventory_status(&self) -> &[CodeableConcept] {
        self.inventory_status.as_deref().unwrap_or(&[])
    }
    fn base_unit(&self) -> Option<CodeableConcept> {
        self.base_unit.clone()
    }
    fn net_content(&self) -> Option<Quantity> {
        self.net_content.clone()
    }
    fn association(&self) -> &[InventoryItemAssociation] {
        self.association.as_deref().unwrap_or(&[])
    }
    fn characteristic(&self) -> &[InventoryItemCharacteristic] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
    fn instance(&self) -> Option<InventoryItemInstance> {
        self.instance.clone()
    }
    fn product_reference(&self) -> Option<Reference> {
        self.product_reference.clone()
    }
}

impl crate::traits::inventory_item::InventoryItemMutators for InventoryItem {
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
    fn set_status(self, value: InventoryitemStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
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
    fn set_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn add_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_name(self, value: Vec<InventoryItemName>) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn add_name(self, item: InventoryItemName) -> Self {
        let mut resource = self.clone();
        resource.name.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_responsible_organization(
        self,
        value: Vec<InventoryItemResponsibleorganization>,
    ) -> Self {
        let mut resource = self.clone();
        resource.responsible_organization = Some(value);
        resource
    }
    fn add_responsible_organization(self, item: InventoryItemResponsibleorganization) -> Self {
        let mut resource = self.clone();
        resource
            .responsible_organization
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_description(self, value: InventoryItemDescription) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_inventory_status(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.inventory_status = Some(value);
        resource
    }
    fn add_inventory_status(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .inventory_status
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_base_unit(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.base_unit = Some(value);
        resource
    }
    fn set_net_content(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.net_content = Some(value);
        resource
    }
    fn set_association(self, value: Vec<InventoryItemAssociation>) -> Self {
        let mut resource = self.clone();
        resource.association = Some(value);
        resource
    }
    fn add_association(self, item: InventoryItemAssociation) -> Self {
        let mut resource = self.clone();
        resource.association.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_characteristic(self, value: Vec<InventoryItemCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: InventoryItemCharacteristic) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instance(self, value: InventoryItemInstance) -> Self {
        let mut resource = self.clone();
        resource.instance = Some(value);
        resource
    }
    fn set_product_reference(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.product_reference = Some(value);
        resource
    }
}

impl crate::traits::inventory_item::InventoryItemExistence for InventoryItem {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_code(&self) -> bool {
        self.code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_name(&self) -> bool {
        self.name.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_responsible_organization(&self) -> bool {
        self.responsible_organization
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_inventory_status(&self) -> bool {
        self.inventory_status
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_base_unit(&self) -> bool {
        self.base_unit.is_some()
    }
    fn has_net_content(&self) -> bool {
        self.net_content.is_some()
    }
    fn has_association(&self) -> bool {
        self.association.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instance(&self) -> bool {
        self.instance.is_some()
    }
    fn has_product_reference(&self) -> bool {
        self.product_reference.is_some()
    }
}

impl crate::validation::ValidatableResource for InventoryItem {
    fn resource_type(&self) -> &'static str {
        "InventoryItem"
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
        Some("http://hl7.org/fhir/StructureDefinition/InventoryItem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::inventory_item::{
    InventoryItemAccessors, InventoryItemExistence, InventoryItemMutators,
};
