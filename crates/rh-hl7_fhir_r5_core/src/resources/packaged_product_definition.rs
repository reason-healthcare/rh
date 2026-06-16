use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::marketing_status::MarketingStatus;
use crate::datatypes::product_shelf_life::ProductShelfLife;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PackagedProductDefinition
///
/// A medically related item or items, in a container or package.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PackagedProductDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedProductDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A unique identifier for this package as whole - not for the content of the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// A name for this package. Typically as listed in a drug formulary, catalogue, inventory etc
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// A high level category e.g. medicinal product, raw material, shipping container etc
    ///
    /// Binding: example (A high level categorisation of a package.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/package-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The product that this is a pack for
    #[serde(rename = "packageFor")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_for: Vec<Reference>,
    /// The status within the lifecycle of this item. High level - not intended to duplicate details elsewhere e.g. legal status, or authorization/marketing status
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// The date at which the given status became applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateTimeType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// A total of the complete count of contained items of a particular type/form, independent of sub-packaging or organization. This can be considered as the pack size. See also packaging.containedItem.amount (especially the long definition)
    #[serde(rename = "containedItemQuantity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_item_quantity: Vec<Quantity>,
    /// Textual description. Note that this is not the name of the package or product
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The legal status of supply of the packaged item as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal_status_of_supply: Vec<PackagedProductDefinitionLegalstatusofsupply>,
    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[serde(rename = "marketingStatus")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<MarketingStatus>,
    /// Identifies if the drug product is supplied with another item such as a diluent or adjuvant
    #[serde(rename = "copackagedIndicator")]
    pub copackaged_indicator: Option<BooleanType>,
    /// Extension element for the 'copackagedIndicator' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copackagedIndicator")]
    pub _copackaged_indicator: Option<Element>,
    /// Manufacturer of this package type (multiple means these are all possible manufacturers)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<Reference>,
    /// Additional information or supporting documentation about the packaged product
    #[serde(rename = "attachedDocument")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<Reference>,
    /// A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap
    pub packaging: Option<PackagedProductDefinitionPackaging>,
    /// Allows the key features to be recorded, such as "hospital pack", "nurse prescribable"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<StringType>,
}
/// PackagedProductDefinition nested structure for the 'legalStatusOfSupply' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedProductDefinitionLegalstatusofsupply {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The actual status of supply. In what situation this package type may be supplied for use
    ///
    /// Binding: example (The prescription supply types appropriate to a medicinal product)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/legal-status-of-supply
    pub code: Option<CodeableConcept>,
    /// The place where the legal status of supply applies
    ///
    /// Binding: example (Jurisdiction codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<CodeableConcept>,
}
/// PackagedProductDefinition nested structure for the 'packaging' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedProductDefinitionPackaging {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The item(s) within the packaging
    #[serde(rename = "containedItem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_item: Vec<PackagedProductDefinitionPackagingContaineditem>,
    /// General characteristics of this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<PackagedProductDefinitionPackagingProperty>,
    /// An identifier that is specific to this particular part of the packaging. Including possibly a Data Carrier Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// The physical type of the container of the items
    ///
    /// Binding: example (A high level categorisation of a package.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/packaging-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than the packaging itself (e.g. a bottle or vial)
    #[serde(rename = "componentPart")]
    pub component_part: Option<BooleanType>,
    /// Extension element for the 'componentPart' primitive field. Contains metadata and extensions.
    #[serde(rename = "_componentPart")]
    pub _component_part: Option<Element>,
    /// The quantity of this level of packaging in the package that contains it (with the outermost level being 1)
    pub quantity: Option<IntegerType>,
    /// Extension element for the 'quantity' primitive field. Contains metadata and extensions.
    pub _quantity: Option<Element>,
    /// Material type of the package item
    ///
    /// Binding: example (A material used in the construction of packages and their components.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/package-material
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<CodeableConcept>,
    /// A possible alternate material for this part of the packaging, that is allowed to be used instead of the usual material
    ///
    /// Binding: example (A material used in the construction of packages and their components.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/package-material
    #[serde(rename = "alternateMaterial")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_material: Vec<CodeableConcept>,
    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<ProductShelfLife>,
    /// Manufacturer of this packaging item (multiple means these are all potential manufacturers)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<Reference>,
    /// Allows containers (and parts of containers) within containers, still as a part of single packaged product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<StringType>,
}
/// PackagedProductDefinitionPackaging nested structure for the 'containedItem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedProductDefinitionPackagingContaineditem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The actual item(s) of medication, as manufactured, or a device, or other medically related item (food, biologicals, raw materials, medical fluids, gases etc.), as contained in the package
    pub item: CodeableReference,
    /// The number of this type of item within this packaging or for continuous items such as liquids it is the quantity (for example 25ml). See also PackagedProductDefinition.containedItemQuantity (especially the long definition)
    pub amount: Option<Quantity>,
}
/// PackagedProductDefinitionPackaging nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagedProductDefinitionPackagingProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A code expressing the type of characteristic
    ///
    /// Binding: example (This value set includes all observable entity codes from SNOMED CT - provided as an exemplar value set.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-characteristic-codes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// A value for the characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,
    /// A value for the characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// A value for the characteristic (date)
    #[serde(rename = "valueDate")]
    pub value_date: Option<DateType>,
    /// A value for the characteristic (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// A value for the characteristic (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
}

impl Default for PackagedProductDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            package_for: Default::default(),
            status: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            contained_item_quantity: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            legal_status_of_supply: Default::default(),
            marketing_status: Default::default(),
            copackaged_indicator: Default::default(),
            _copackaged_indicator: Default::default(),
            manufacturer: Default::default(),
            attached_document: Default::default(),
            packaging: Default::default(),
            characteristic: Default::default(),
        }
    }
}

impl Default for PackagedProductDefinitionLegalstatusofsupply {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            jurisdiction: Default::default(),
        }
    }
}

impl Default for PackagedProductDefinitionPackaging {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            contained_item: Default::default(),
            property: Default::default(),
            identifier: Default::default(),
            type_: Default::default(),
            component_part: Default::default(),
            _component_part: Default::default(),
            quantity: Default::default(),
            _quantity: Default::default(),
            material: Default::default(),
            alternate_material: Default::default(),
            shelf_life_storage: Default::default(),
            manufacturer: Default::default(),
            packaging: Default::default(),
        }
    }
}

impl Default for PackagedProductDefinitionPackagingContaineditem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for PackagedProductDefinitionPackagingProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_date: Default::default(),
            value_boolean: Default::default(),
            value_attachment: Default::default(),
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
        vec![rh_foundation::ElementBinding::new(
            "PackagedProductDefinition.language",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
        )
        .with_description("IETF language tag for a human language")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.packageFor", 0, None),
            rh_foundation::ElementCardinality::new("PackagedProductDefinition.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.statusDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.containedItemQuantity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.legalStatusOfSupply.jurisdiction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.marketingStatus",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.copackagedIndicator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.manufacturer",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.attachedDocument",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.componentPart",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.material",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.alternateMaterial",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.shelfLifeStorage",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.manufacturer",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.property.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem.item",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.containedItem.amount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.packaging.packaging",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PackagedProductDefinition.characteristic",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PackagedProductDefinition {
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

impl crate::traits::resource::ResourceMutators for PackagedProductDefinition {
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

impl crate::traits::resource::ResourceExistence for PackagedProductDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PackagedProductDefinition {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for PackagedProductDefinition {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for PackagedProductDefinition {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::packaged_product_definition::PackagedProductDefinitionAccessors
    for PackagedProductDefinition
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn package_for(&self) -> &[Reference] {
        self.package_for.as_slice()
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn status_date(&self) -> Option<DateTimeType> {
        self.status_date.clone()
    }
    fn contained_item_quantity(&self) -> &[Quantity] {
        self.contained_item_quantity.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn legal_status_of_supply(&self) -> &[PackagedProductDefinitionLegalstatusofsupply] {
        self.legal_status_of_supply.as_slice()
    }
    fn marketing_status(&self) -> &[MarketingStatus] {
        self.marketing_status.as_slice()
    }
    fn copackaged_indicator(&self) -> Option<BooleanType> {
        self.copackaged_indicator
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_slice()
    }
    fn attached_document(&self) -> &[Reference] {
        self.attached_document.as_slice()
    }
    fn packaging(&self) -> Option<PackagedProductDefinitionPackaging> {
        self.packaging.clone()
    }
}

impl crate::traits::packaged_product_definition::PackagedProductDefinitionMutators
    for PackagedProductDefinition
{
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_package_for(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.package_for = value;
        resource
    }
    fn add_package_for(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.package_for.push(item);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_contained_item_quantity(self, value: Vec<Quantity>) -> Self {
        let mut resource = self.clone();
        resource.contained_item_quantity = value;
        resource
    }
    fn add_contained_item_quantity(self, item: Quantity) -> Self {
        let mut resource = self.clone();
        resource.contained_item_quantity.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_legal_status_of_supply(
        self,
        value: Vec<PackagedProductDefinitionLegalstatusofsupply>,
    ) -> Self {
        let mut resource = self.clone();
        resource.legal_status_of_supply = value;
        resource
    }
    fn add_legal_status_of_supply(
        self,
        item: PackagedProductDefinitionLegalstatusofsupply,
    ) -> Self {
        let mut resource = self.clone();
        resource.legal_status_of_supply.push(item);
        resource
    }
    fn set_marketing_status(self, value: Vec<MarketingStatus>) -> Self {
        let mut resource = self.clone();
        resource.marketing_status = value;
        resource
    }
    fn add_marketing_status(self, item: MarketingStatus) -> Self {
        let mut resource = self.clone();
        resource.marketing_status.push(item);
        resource
    }
    fn set_copackaged_indicator(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.copackaged_indicator = Some(value);
        resource
    }
    fn set_manufacturer(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.manufacturer = value;
        resource
    }
    fn add_manufacturer(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.manufacturer.push(item);
        resource
    }
    fn set_attached_document(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.attached_document = value;
        resource
    }
    fn add_attached_document(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.attached_document.push(item);
        resource
    }
    fn set_packaging(self, value: PackagedProductDefinitionPackaging) -> Self {
        let mut resource = self.clone();
        resource.packaging = Some(value);
        resource
    }
    fn set_characteristic(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = value;
        resource
    }
    fn add_characteristic(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.characteristic.push(item);
        resource
    }
}

impl crate::traits::packaged_product_definition::PackagedProductDefinitionExistence
    for PackagedProductDefinition
{
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_package_for(&self) -> bool {
        !self.package_for.is_empty()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_contained_item_quantity(&self) -> bool {
        !self.contained_item_quantity.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_legal_status_of_supply(&self) -> bool {
        !self.legal_status_of_supply.is_empty()
    }
    fn has_marketing_status(&self) -> bool {
        !self.marketing_status.is_empty()
    }
    fn has_copackaged_indicator(&self) -> bool {
        self.copackaged_indicator.is_some()
    }
    fn has_manufacturer(&self) -> bool {
        !self.manufacturer.is_empty()
    }
    fn has_attached_document(&self) -> bool {
        !self.attached_document.is_empty()
    }
    fn has_packaging(&self) -> bool {
        self.packaging.is_some()
    }
    fn has_characteristic(&self) -> bool {
        !self.characteristic.is_empty()
    }
}

impl crate::validation::ValidatableResource for PackagedProductDefinition {
    fn resource_type(&self) -> &'static str {
        "PackagedProductDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::packaged_product_definition::{
    PackagedProductDefinitionAccessors, PackagedProductDefinitionExistence,
    PackagedProductDefinitionMutators,
};
