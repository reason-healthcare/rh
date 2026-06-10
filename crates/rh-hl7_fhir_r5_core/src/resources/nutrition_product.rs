use crate::bindings::nutritionproduct_status::NutritionproductStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// NutritionProduct
///
/// A food or supplement that is consumed by patients.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProduct {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// A code that can identify the detailed nutrients and ingredients in a specific food product
    ///
    /// Binding: example (Codes identifying specific types of nutrition products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/edible-substance-type
    pub code: Option<CodeableConcept>,
    /// active | inactive | entered-in-error
    pub status: NutritionproductStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Broad product groups or categories used to classify the product, such as Legume and Legume Products, Beverages, or Beef Products
    ///
    /// Binding: example (Codes identifying classes of nutrition products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/nutrition-product-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Manufacturer, representative or officially responsible for the product
    pub manufacturer: Option<Vec<Reference>>,
    /// The product's nutritional information expressed by the nutrients
    pub nutrient: Option<Vec<NutritionProductNutrient>>,
    /// Ingredients contained in this product
    pub ingredient: Option<Vec<NutritionProductIngredient>>,
    /// Known or suspected allergens that are a part of this product
    ///
    /// Binding: example (Codes that identify substances that can be an allergen.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/allergen-class
    #[serde(rename = "knownAllergen")]
    pub known_allergen: Option<Vec<CodeableReference>>,
    /// Specifies descriptive properties of the nutrition product
    pub characteristic: Option<Vec<NutritionProductCharacteristic>>,
    /// One or several physical instances or occurrences of the nutrition product
    pub instance: Option<Vec<NutritionProductInstance>>,
    /// Comments made about the product
    pub note: Option<Vec<Annotation>>,
}
/// NutritionProduct nested structure for the 'nutrient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProductNutrient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The (relevant) nutrients in the product
    ///
    /// Binding: example (Codes that identify nutrients that could be parts of nutrition products.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/nutrition-product-nutrient
    pub item: Option<CodeableReference>,
    /// The amount of nutrient expressed in one or more units: X per pack / per serving / per dose
    pub amount: Option<Vec<Ratio>>,
}
/// NutritionProduct nested structure for the 'characteristic' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProductCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code specifying the type of characteristic
    ///
    /// Binding: example (Codes that identify properties that can be measured.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/measurement-property
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// The value of the characteristic (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// The value of the characteristic (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// The value of the characteristic (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// The value of the characteristic (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
    /// The value of the characteristic (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// The value of the characteristic (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
}
/// NutritionProduct nested structure for the 'ingredient' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProductIngredient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The ingredient contained in the product
    pub item: CodeableReference,
    /// The amount of ingredient that is in the product
    pub amount: Option<Vec<Ratio>>,
}
/// NutritionProduct nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProductInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The amount of items or instances
    pub quantity: Option<Quantity>,
    /// The identifier for the physical instance, typically a serial number or manufacturer number
    pub identifier: Option<Vec<Identifier>>,
    /// The name for the specific product
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// The identification of the batch or lot of the product
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<StringType>,
    /// Extension element for the 'lotNumber' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lotNumber")]
    pub _lot_number: Option<Element>,
    /// The expiry date or date and time for the product
    pub expiry: Option<DateTimeType>,
    /// Extension element for the 'expiry' primitive field. Contains metadata and extensions.
    pub _expiry: Option<Element>,
    /// The date until which the product is expected to be good for consumption
    #[serde(rename = "useBy")]
    pub use_by: Option<DateTimeType>,
    /// Extension element for the 'useBy' primitive field. Contains metadata and extensions.
    #[serde(rename = "_useBy")]
    pub _use_by: Option<Element>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Identifier>,
}

impl Default for NutritionProduct {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            code: Default::default(),
            status: NutritionproductStatus::default(),
            _status: Default::default(),
            category: Default::default(),
            manufacturer: Default::default(),
            nutrient: Default::default(),
            ingredient: Default::default(),
            known_allergen: Default::default(),
            characteristic: Default::default(),
            instance: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for NutritionProductNutrient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for NutritionProductCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_codeable_concept: Default::default(),
            value_string: Default::default(),
            value_quantity: Default::default(),
            value_base64_binary: Default::default(),
            value_attachment: Default::default(),
            value_boolean: Default::default(),
        }
    }
}

impl Default for NutritionProductIngredient {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item: CodeableReference::default(),
            amount: Default::default(),
        }
    }
}

impl Default for NutritionProductInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            quantity: Default::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            lot_number: Default::default(),
            _lot_number: Default::default(),
            expiry: Default::default(),
            _expiry: Default::default(),
            use_by: Default::default(),
            _use_by: Default::default(),
            biological_source_event: Default::default(),
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
                "NutritionProduct.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "NutritionProduct.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/nutritionproduct-status|5.0.0",
            )
            .with_description("Codes identifying the lifecycle stage of a product."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NutritionProduct.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.contained", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.extension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.category", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.manufacturer", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.nutrient", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.nutrient.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.nutrient.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.nutrient.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.nutrient.item", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.nutrient.amount", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.ingredient", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.ingredient.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.ingredient.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.ingredient.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.ingredient.item", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.ingredient.amount", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.knownAllergen", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.characteristic", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.characteristic.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.characteristic.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.characteristic.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.characteristic.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.characteristic.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.instance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.instance.quantity",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.identifier", 0, None),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.instance.lotNumber",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.expiry", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NutritionProduct.instance.useBy", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NutritionProduct.instance.biologicalSourceEvent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("NutritionProduct.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for NutritionProduct {
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

impl crate::traits::resource::ResourceMutators for NutritionProduct {
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

impl crate::traits::resource::ResourceExistence for NutritionProduct {
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

impl crate::traits::domain_resource::DomainResourceAccessors for NutritionProduct {
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

impl crate::traits::domain_resource::DomainResourceMutators for NutritionProduct {
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

impl crate::traits::domain_resource::DomainResourceExistence for NutritionProduct {
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

impl crate::traits::nutrition_product::NutritionProductAccessors for NutritionProduct {
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn status(&self) -> NutritionproductStatus {
        self.status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn manufacturer(&self) -> &[Reference] {
        self.manufacturer.as_deref().unwrap_or(&[])
    }
    fn nutrient(&self) -> &[NutritionProductNutrient] {
        self.nutrient.as_deref().unwrap_or(&[])
    }
    fn ingredient(&self) -> &[NutritionProductIngredient] {
        self.ingredient.as_deref().unwrap_or(&[])
    }
    fn known_allergen(&self) -> &[CodeableReference] {
        self.known_allergen.as_deref().unwrap_or(&[])
    }
    fn characteristic(&self) -> &[NutritionProductCharacteristic] {
        self.characteristic.as_deref().unwrap_or(&[])
    }
    fn instance(&self) -> &[NutritionProductInstance] {
        self.instance.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::nutrition_product::NutritionProductMutators for NutritionProduct {
    fn new() -> Self {
        Self::default()
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_status(self, value: NutritionproductStatus) -> Self {
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
    fn set_nutrient(self, value: Vec<NutritionProductNutrient>) -> Self {
        let mut resource = self.clone();
        resource.nutrient = Some(value);
        resource
    }
    fn add_nutrient(self, item: NutritionProductNutrient) -> Self {
        let mut resource = self.clone();
        resource.nutrient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_ingredient(self, value: Vec<NutritionProductIngredient>) -> Self {
        let mut resource = self.clone();
        resource.ingredient = Some(value);
        resource
    }
    fn add_ingredient(self, item: NutritionProductIngredient) -> Self {
        let mut resource = self.clone();
        resource.ingredient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_known_allergen(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.known_allergen = Some(value);
        resource
    }
    fn add_known_allergen(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource
            .known_allergen
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_characteristic(self, value: Vec<NutritionProductCharacteristic>) -> Self {
        let mut resource = self.clone();
        resource.characteristic = Some(value);
        resource
    }
    fn add_characteristic(self, item: NutritionProductCharacteristic) -> Self {
        let mut resource = self.clone();
        resource
            .characteristic
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instance(self, value: Vec<NutritionProductInstance>) -> Self {
        let mut resource = self.clone();
        resource.instance = Some(value);
        resource
    }
    fn add_instance(self, item: NutritionProductInstance) -> Self {
        let mut resource = self.clone();
        resource.instance.get_or_insert_with(Vec::new).push(item);
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
}

impl crate::traits::nutrition_product::NutritionProductExistence for NutritionProduct {
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_manufacturer(&self) -> bool {
        self.manufacturer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_nutrient(&self) -> bool {
        self.nutrient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_ingredient(&self) -> bool {
        self.ingredient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_known_allergen(&self) -> bool {
        self.known_allergen.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_characteristic(&self) -> bool {
        self.characteristic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instance(&self) -> bool {
        self.instance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for NutritionProduct {
    fn resource_type(&self) -> &'static str {
        "NutritionProduct"
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
        Some("http://hl7.org/fhir/StructureDefinition/NutritionProduct")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::nutrition_product::{
    NutritionProductAccessors, NutritionProductExistence, NutritionProductMutators,
};
