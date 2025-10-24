use crate::bindings::product_category::ProductCategory;
use crate::bindings::product_status::ProductStatus;
use crate::bindings::product_storage_scale::ProductStorageScale;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// BiologicallyDerivedProduct
///
/// A material substance originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProduct {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// organ | tissue | fluid | cells | biologicalAgent
    #[serde(rename = "productCategory")]
    pub product_category: Option<ProductCategory>,
    /// Extension element for the 'productCategory' primitive field. Contains metadata and extensions.
    #[serde(rename = "_productCategory")]
    pub _product_category: Option<Element>,
    /// What this biologically derived product is
    ///
    /// Binding: example (Biologically Derived Product Code.)
    #[serde(rename = "productCode")]
    pub product_code: Option<CodeableConcept>,
    /// available | unavailable
    pub status: Option<ProductStatus>,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Procedure request
    pub request: Option<Vec<Reference>>,
    /// The amount of this biologically derived product
    pub quantity: Option<IntegerType>,
    /// Extension element for the 'quantity' primitive field. Contains metadata and extensions.
    pub _quantity: Option<Element>,
    /// BiologicallyDerivedProduct parent
    pub parent: Option<Vec<Reference>>,
    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,
    /// Any processing of the product during collection
    pub processing: Option<Vec<BiologicallyDerivedProductProcessing>>,
    /// Any manipulation of product post-collection
    pub manipulation: Option<BiologicallyDerivedProductManipulation>,
    /// Product storage
    pub storage: Option<Vec<BiologicallyDerivedProductStorage>>,
}
/// BiologicallyDerivedProduct nested structure for the 'manipulation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductManipulation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of manipulation
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Time of manipulation (dateTime)
    #[serde(rename = "timeDateTime")]
    pub time_date_time: Option<DateTimeType>,
    /// Time of manipulation (Period)
    #[serde(rename = "timePeriod")]
    pub time_period: Option<Period>,
}
/// BiologicallyDerivedProduct nested structure for the 'processing' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductProcessing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of of processing
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Procesing code
    ///
    /// Binding: example (Biologically Derived Product Procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub procedure: Option<CodeableConcept>,
    /// Substance added during processing
    pub additive: Option<Reference>,
    /// Time of processing (dateTime)
    #[serde(rename = "timeDateTime")]
    pub time_date_time: Option<DateTimeType>,
    /// Time of processing (Period)
    #[serde(rename = "timePeriod")]
    pub time_period: Option<Period>,
}
/// BiologicallyDerivedProduct nested structure for the 'collection' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductCollection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Individual performing collection
    pub collector: Option<Reference>,
    /// Who is product from
    pub source: Option<Reference>,
    /// Time of product collection (dateTime)
    #[serde(rename = "collectedDateTime")]
    pub collected_date_time: Option<DateTimeType>,
    /// Time of product collection (Period)
    #[serde(rename = "collectedPeriod")]
    pub collected_period: Option<Period>,
}
/// BiologicallyDerivedProduct nested structure for the 'storage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductStorage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of storage
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Storage temperature
    pub temperature: Option<DecimalType>,
    /// Extension element for the 'temperature' primitive field. Contains metadata and extensions.
    pub _temperature: Option<Element>,
    /// farenheit | celsius | kelvin
    pub scale: Option<ProductStorageScale>,
    /// Extension element for the 'scale' primitive field. Contains metadata and extensions.
    pub _scale: Option<Element>,
    /// Storage timeperiod
    pub duration: Option<Period>,
}

impl Default for BiologicallyDerivedProduct {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            product_category: Default::default(),
            _product_category: Default::default(),
            product_code: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            request: Default::default(),
            quantity: Default::default(),
            _quantity: Default::default(),
            parent: Default::default(),
            collection: Default::default(),
            processing: Default::default(),
            manipulation: Default::default(),
            storage: Default::default(),
        }
    }
}

impl Default for BiologicallyDerivedProductManipulation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            time_date_time: Default::default(),
            time_period: Default::default(),
        }
    }
}

impl Default for BiologicallyDerivedProductProcessing {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            procedure: Default::default(),
            additive: Default::default(),
            time_date_time: Default::default(),
            time_period: Default::default(),
        }
    }
}

impl Default for BiologicallyDerivedProductCollection {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            collector: Default::default(),
            source: Default::default(),
            collected_date_time: Default::default(),
            collected_period: Default::default(),
        }
    }
}

impl Default for BiologicallyDerivedProductStorage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            temperature: Default::default(),
            _temperature: Default::default(),
            scale: Default::default(),
            _scale: Default::default(),
            duration: Default::default(),
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
impl crate::traits::resource::ResourceAccessors for BiologicallyDerivedProduct {
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

impl crate::traits::resource::ResourceMutators for BiologicallyDerivedProduct {
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

impl crate::traits::resource::ResourceExistence for BiologicallyDerivedProduct {
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

impl crate::traits::domain_resource::DomainResourceAccessors for BiologicallyDerivedProduct {
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

impl crate::traits::domain_resource::DomainResourceMutators for BiologicallyDerivedProduct {
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

impl crate::traits::domain_resource::DomainResourceExistence for BiologicallyDerivedProduct {
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

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductAccessors
    for BiologicallyDerivedProduct
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn product_category(&self) -> Option<ProductCategory> {
        self.product_category.clone()
    }
    fn product_code(&self) -> Option<CodeableConcept> {
        self.product_code.clone()
    }
    fn status(&self) -> Option<ProductStatus> {
        self.status.clone()
    }
    fn request(&self) -> &[Reference] {
        self.request.as_deref().unwrap_or(&[])
    }
    fn quantity(&self) -> Option<IntegerType> {
        self.quantity
    }
    fn parent(&self) -> &[Reference] {
        self.parent.as_deref().unwrap_or(&[])
    }
    fn collection(&self) -> Option<BiologicallyDerivedProductCollection> {
        self.collection.clone()
    }
    fn processing(&self) -> &[BiologicallyDerivedProductProcessing] {
        self.processing.as_deref().unwrap_or(&[])
    }
    fn manipulation(&self) -> Option<BiologicallyDerivedProductManipulation> {
        self.manipulation.clone()
    }
    fn storage(&self) -> &[BiologicallyDerivedProductStorage] {
        self.storage.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductMutators
    for BiologicallyDerivedProduct
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
    fn set_product_category(self, value: ProductCategory) -> Self {
        let mut resource = self.clone();
        resource.product_category = Some(value);
        resource
    }
    fn set_product_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.product_code = Some(value);
        resource
    }
    fn set_status(self, value: ProductStatus) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.request = Some(value);
        resource
    }
    fn add_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.request.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_quantity(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_parent(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.parent = Some(value);
        resource
    }
    fn add_parent(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_collection(self, value: BiologicallyDerivedProductCollection) -> Self {
        let mut resource = self.clone();
        resource.collection = Some(value);
        resource
    }
    fn set_processing(self, value: Vec<BiologicallyDerivedProductProcessing>) -> Self {
        let mut resource = self.clone();
        resource.processing = Some(value);
        resource
    }
    fn add_processing(self, item: BiologicallyDerivedProductProcessing) -> Self {
        let mut resource = self.clone();
        resource.processing.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_manipulation(self, value: BiologicallyDerivedProductManipulation) -> Self {
        let mut resource = self.clone();
        resource.manipulation = Some(value);
        resource
    }
    fn set_storage(self, value: Vec<BiologicallyDerivedProductStorage>) -> Self {
        let mut resource = self.clone();
        resource.storage = Some(value);
        resource
    }
    fn add_storage(self, item: BiologicallyDerivedProductStorage) -> Self {
        let mut resource = self.clone();
        resource.storage.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductExistence
    for BiologicallyDerivedProduct
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
    fn has_product_category(&self) -> bool {
        self.product_category.is_some()
    }
    fn has_product_code(&self) -> bool {
        self.product_code.is_some()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_request(&self) -> bool {
        self.request.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_parent(&self) -> bool {
        self.parent.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_collection(&self) -> bool {
        self.collection.is_some()
    }
    fn has_processing(&self) -> bool {
        self.processing.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_manipulation(&self) -> bool {
        self.manipulation.is_some()
    }
    fn has_storage(&self) -> bool {
        self.storage.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for BiologicallyDerivedProduct {
    fn resource_type(&self) -> &'static str {
        "BiologicallyDerivedProduct"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct")
    }
}
