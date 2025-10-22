use crate::bindings::product_category::ProductCategory;
use crate::bindings::product_status::ProductStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::integer::IntegerType;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductCollection;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductManipulation;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductProcessing;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductStorage;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// BiologicallyDerivedProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A material substance originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the productCategory field.
    fn product_category(&self) -> Option<ProductCategory>;
    /// Returns a reference to the productCode field.
    fn product_code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<ProductStatus>;
    /// Returns a reference to the request field.
    fn request(&self) -> &[Reference];
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<IntegerType>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> &[Reference];
    /// Returns a reference to the collection field.
    fn collection(&self) -> Option<BiologicallyDerivedProductCollection>;
    /// Returns a reference to the processing field.
    fn processing(&self) -> &[BiologicallyDerivedProductProcessing];
    /// Returns a reference to the manipulation field.
    fn manipulation(&self) -> Option<BiologicallyDerivedProductManipulation>;
    /// Returns a reference to the storage field.
    fn storage(&self) -> &[BiologicallyDerivedProductStorage];
}
/// BiologicallyDerivedProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A material substance originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductMutators: DomainResourceMutators {
    /// Create a new BiologicallyDerivedProduct with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::biologically_derived_product::BiologicallyDerivedProduct;
    /// use hl7_fhir_r4_core::traits::biologically_derived_product::BiologicallyDerivedProductMutators;
    ///
    /// let resource = BiologicallyDerivedProduct::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the productCategory field and returns self for chaining.
    fn set_product_category(self, value: ProductCategory) -> Self;
    /// Sets the productCode field and returns self for chaining.
    fn set_product_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ProductStatus) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the request field and returns self for chaining.
    fn add_request(self, item: Reference) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: i32) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the parent field and returns self for chaining.
    fn add_parent(self, item: Reference) -> Self;
    /// Sets the collection field and returns self for chaining.
    fn set_collection(self, value: BiologicallyDerivedProductCollection) -> Self;
    /// Sets the processing field and returns self for chaining.
    fn set_processing(self, value: Vec<BiologicallyDerivedProductProcessing>) -> Self;
    /// Adds an item to the processing field and returns self for chaining.
    fn add_processing(self, item: BiologicallyDerivedProductProcessing) -> Self;
    /// Sets the manipulation field and returns self for chaining.
    fn set_manipulation(self, value: BiologicallyDerivedProductManipulation) -> Self;
    /// Sets the storage field and returns self for chaining.
    fn set_storage(self, value: Vec<BiologicallyDerivedProductStorage>) -> Self;
    /// Adds an item to the storage field and returns self for chaining.
    fn add_storage(self, item: BiologicallyDerivedProductStorage) -> Self;
}
/// BiologicallyDerivedProduct Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A material substance originating from a biological entity intended to be transplanted or infused
/// into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductExistence: DomainResourceExistence {
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
    /// Returns true if the product_category field is present (Some).
    fn has_product_category(&self) -> bool;
    /// Returns true if the product_code field is present (Some).
    fn has_product_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the request field is not empty.
    fn has_request(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the parent field is not empty.
    fn has_parent(&self) -> bool;
    /// Returns true if the collection field is present (Some).
    fn has_collection(&self) -> bool;
    /// Returns true if the processing field is not empty.
    fn has_processing(&self) -> bool;
    /// Returns true if the manipulation field is present (Some).
    fn has_manipulation(&self) -> bool;
    /// Returns true if the storage field is not empty.
    fn has_storage(&self) -> bool;
}
