use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// BiologicallyDerivedProduct
///
/// A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProduct {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// organ | tissue | fluid | cells | biologicalAgent
    ///
    /// Binding: example (Biologically Derived Product Category.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-category
    #[serde(rename = "productCategory")]
    pub product_category: Option<Coding>,
    /// A code that identifies the kind of this biologically derived product
    ///
    /// Binding: example (Biologically-derived Product Codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderived-productcodes
    #[serde(rename = "productCode")]
    pub product_code: Option<CodeableConcept>,
    /// The parent biologically-derived product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<Reference>,
    /// Request to obtain and/or infuse this product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<Reference>,
    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Identifier>,
    /// Processing facilities responsible for the labeling and distribution of this biologically derived product
    #[serde(rename = "processingFacility")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processing_facility: Vec<Reference>,
    /// A unique identifier for an aliquot of a product
    pub division: Option<StringType>,
    /// Extension element for the 'division' primitive field. Contains metadata and extensions.
    pub _division: Option<Element>,
    /// available | unavailable
    ///
    /// Binding: example (Biologically Derived Product Status.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderived-product-status
    #[serde(rename = "productStatus")]
    pub product_status: Option<Coding>,
    /// Date, and where relevant time, of expiration
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTimeType>,
    /// Extension element for the 'expirationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_expirationDate")]
    pub _expiration_date: Option<Element>,
    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,
    /// Product storage temperature requirements
    #[serde(rename = "storageTempRequirements")]
    pub storage_temp_requirements: Option<Range>,
    /// A property that is specific to this BiologicallyDerviedProduct instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<BiologicallyDerivedProductProperty>,
}
/// BiologicallyDerivedProduct nested structure for the 'collection' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductCollection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Individual performing collection
    pub collector: Option<Reference>,
    /// The patient who underwent the medical procedure to collect the product or the organization that facilitated the collection
    pub source: Option<Reference>,
    /// Time of product collection (dateTime)
    #[serde(rename = "collectedDateTime")]
    pub collected_date_time: Option<DateTimeType>,
    /// Time of product collection (Period)
    #[serde(rename = "collectedPeriod")]
    pub collected_period: Option<Period>,
}
/// BiologicallyDerivedProduct nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicallyDerivedProductProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code that specifies the property
    ///
    /// Binding: example (Biologically Derived Product Property Type Codes)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/biologicallyderived-product-property-type-codes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Property values (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Property values (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Property values (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Property values (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
    /// Property values (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Property values (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Property values (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// Property values (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Property values (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
}

impl Default for BiologicallyDerivedProduct {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            product_category: Default::default(),
            product_code: Default::default(),
            parent: Default::default(),
            request: Default::default(),
            identifier: Default::default(),
            biological_source_event: Default::default(),
            processing_facility: Default::default(),
            division: Default::default(),
            _division: Default::default(),
            product_status: Default::default(),
            expiration_date: Default::default(),
            _expiration_date: Default::default(),
            collection: Default::default(),
            storage_temp_requirements: Default::default(),
            property: Default::default(),
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

impl Default for BiologicallyDerivedProductProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_codeable_concept: Default::default(),
            value_period: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_string: Default::default(),
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
            "BiologicallyDerivedProduct.language",
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
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.contained", 0, None),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.productCategory",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.productCode",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.parent", 0, None),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.request", 0, None),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.identifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.biologicalSourceEvent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.processingFacility",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.division",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.productStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.expirationDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.collector",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.source",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.collection.collected[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.storageTempRequirements",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("BiologicallyDerivedProduct.property", 0, None),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.property.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "BiologicallyDerivedProduct.property.value[x]",
                1,
                Some(1),
            ),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for BiologicallyDerivedProduct {
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

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductAccessors
    for BiologicallyDerivedProduct
{
    fn product_category(&self) -> Option<Coding> {
        self.product_category.clone()
    }
    fn product_code(&self) -> Option<CodeableConcept> {
        self.product_code.clone()
    }
    fn parent(&self) -> &[Reference] {
        self.parent.as_slice()
    }
    fn request(&self) -> &[Reference] {
        self.request.as_slice()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn biological_source_event(&self) -> Option<Identifier> {
        self.biological_source_event.clone()
    }
    fn processing_facility(&self) -> &[Reference] {
        self.processing_facility.as_slice()
    }
    fn division(&self) -> Option<StringType> {
        self.division.clone()
    }
    fn product_status(&self) -> Option<Coding> {
        self.product_status.clone()
    }
    fn expiration_date(&self) -> Option<DateTimeType> {
        self.expiration_date.clone()
    }
    fn collection(&self) -> Option<BiologicallyDerivedProductCollection> {
        self.collection.clone()
    }
    fn storage_temp_requirements(&self) -> Option<Range> {
        self.storage_temp_requirements.clone()
    }
    fn property(&self) -> &[BiologicallyDerivedProductProperty] {
        self.property.as_slice()
    }
}

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductMutators
    for BiologicallyDerivedProduct
{
    fn new() -> Self {
        Self::default()
    }
    fn set_product_category(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.product_category = Some(value);
        resource
    }
    fn set_product_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.product_code = Some(value);
        resource
    }
    fn set_parent(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.parent = value;
        resource
    }
    fn add_parent(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.parent.push(item);
        resource
    }
    fn set_request(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.request = value;
        resource
    }
    fn add_request(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.request.push(item);
        resource
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
    fn set_biological_source_event(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.biological_source_event = Some(value);
        resource
    }
    fn set_processing_facility(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.processing_facility = value;
        resource
    }
    fn add_processing_facility(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.processing_facility.push(item);
        resource
    }
    fn set_division(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.division = Some(value);
        resource
    }
    fn set_product_status(self, value: Coding) -> Self {
        let mut resource = self.clone();
        resource.product_status = Some(value);
        resource
    }
    fn set_expiration_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expiration_date = Some(value);
        resource
    }
    fn set_collection(self, value: BiologicallyDerivedProductCollection) -> Self {
        let mut resource = self.clone();
        resource.collection = Some(value);
        resource
    }
    fn set_storage_temp_requirements(self, value: Range) -> Self {
        let mut resource = self.clone();
        resource.storage_temp_requirements = Some(value);
        resource
    }
    fn set_property(self, value: Vec<BiologicallyDerivedProductProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = value;
        resource
    }
    fn add_property(self, item: BiologicallyDerivedProductProperty) -> Self {
        let mut resource = self.clone();
        resource.property.push(item);
        resource
    }
}

impl crate::traits::biologically_derived_product::BiologicallyDerivedProductExistence
    for BiologicallyDerivedProduct
{
    fn has_product_category(&self) -> bool {
        self.product_category.is_some()
    }
    fn has_product_code(&self) -> bool {
        self.product_code.is_some()
    }
    fn has_parent(&self) -> bool {
        !self.parent.is_empty()
    }
    fn has_request(&self) -> bool {
        !self.request.is_empty()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_biological_source_event(&self) -> bool {
        self.biological_source_event.is_some()
    }
    fn has_processing_facility(&self) -> bool {
        !self.processing_facility.is_empty()
    }
    fn has_division(&self) -> bool {
        self.division.is_some()
    }
    fn has_product_status(&self) -> bool {
        self.product_status.is_some()
    }
    fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }
    fn has_collection(&self) -> bool {
        self.collection.is_some()
    }
    fn has_storage_temp_requirements(&self) -> bool {
        self.storage_temp_requirements.is_some()
    }
    fn has_property(&self) -> bool {
        !self.property.is_empty()
    }
}

impl crate::validation::ValidatableResource for BiologicallyDerivedProduct {
    fn resource_type(&self) -> &'static str {
        "BiologicallyDerivedProduct"
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
        Some("http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::biologically_derived_product::{
    BiologicallyDerivedProductAccessors, BiologicallyDerivedProductExistence,
    BiologicallyDerivedProductMutators,
};
