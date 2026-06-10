use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductCollection;
use crate::resources::biologically_derived_product::BiologicallyDerivedProductProperty;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// BiologicallyDerivedProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: BiologicallyDerivedProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait BiologicallyDerivedProductAccessors: DomainResourceAccessors {
    /// Returns a reference to the productCategory field.
    fn product_category(&self) -> Option<Coding>;
    /// Returns a reference to the productCode field.
    fn product_code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the parent field.
    fn parent(&self) -> &[Reference];
    /// Returns a reference to the request field.
    fn request(&self) -> &[Reference];
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the biologicalSourceEvent field.
    fn biological_source_event(&self) -> Option<Identifier>;
    /// Returns a reference to the processingFacility field.
    fn processing_facility(&self) -> &[Reference];
    /// Returns a reference to the division field.
    fn division(&self) -> Option<StringType>;
    /// Returns a reference to the productStatus field.
    fn product_status(&self) -> Option<Coding>;
    /// Returns a reference to the expirationDate field.
    fn expiration_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the collection field.
    fn collection(&self) -> Option<BiologicallyDerivedProductCollection>;
    /// Returns a reference to the storageTempRequirements field.
    fn storage_temp_requirements(&self) -> Option<Range>;
    /// Returns a reference to the property field.
    fn property(&self) -> &[BiologicallyDerivedProductProperty];
}
/// BiologicallyDerivedProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 5.0.0
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
    /// use rh_hl7_fhir_r5_core::resources::biologically_derived_product::BiologicallyDerivedProduct;
    /// use rh_hl7_fhir_r5_core::traits::biologically_derived_product::BiologicallyDerivedProductMutators;
    ///
    /// let resource = BiologicallyDerivedProduct::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the productCategory field and returns self for chaining.
    fn set_product_category(self, value: Coding) -> Self;
    /// Sets the productCode field and returns self for chaining.
    fn set_product_code(self, value: CodeableConcept) -> Self;
    /// Sets the parent field and returns self for chaining.
    fn set_parent(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the parent field and returns self for chaining.
    fn add_parent(self, item: Reference) -> Self;
    /// Sets the request field and returns self for chaining.
    fn set_request(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the request field and returns self for chaining.
    fn add_request(self, item: Reference) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the biologicalSourceEvent field and returns self for chaining.
    fn set_biological_source_event(self, value: Identifier) -> Self;
    /// Sets the processingFacility field and returns self for chaining.
    fn set_processing_facility(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the processingFacility field and returns self for chaining.
    fn add_processing_facility(self, item: Reference) -> Self;
    /// Sets the division field and returns self for chaining.
    fn set_division(self, value: String) -> Self;
    /// Sets the productStatus field and returns self for chaining.
    fn set_product_status(self, value: Coding) -> Self;
    /// Sets the expirationDate field and returns self for chaining.
    fn set_expiration_date(self, value: String) -> Self;
    /// Sets the collection field and returns self for chaining.
    fn set_collection(self, value: BiologicallyDerivedProductCollection) -> Self;
    /// Sets the storageTempRequirements field and returns self for chaining.
    fn set_storage_temp_requirements(self, value: Range) -> Self;
    /// Sets the property field and returns self for chaining.
    fn set_property(self, value: Vec<BiologicallyDerivedProductProperty>) -> Self;
    /// Adds an item to the property field and returns self for chaining.
    fn add_property(self, item: BiologicallyDerivedProductProperty) -> Self;
}
/// BiologicallyDerivedProduct Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
/// - Version: 5.0.0
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
    /// Returns true if the product_category field is present (Some).
    fn has_product_category(&self) -> bool;
    /// Returns true if the product_code field is present (Some).
    fn has_product_code(&self) -> bool;
    /// Returns true if the parent field is not empty.
    fn has_parent(&self) -> bool;
    /// Returns true if the request field is not empty.
    fn has_request(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the biological_source_event field is present (Some).
    fn has_biological_source_event(&self) -> bool;
    /// Returns true if the processing_facility field is not empty.
    fn has_processing_facility(&self) -> bool;
    /// Returns true if the division field is present (Some).
    fn has_division(&self) -> bool;
    /// Returns true if the product_status field is present (Some).
    fn has_product_status(&self) -> bool;
    /// Returns true if the expiration_date field is present (Some).
    fn has_expiration_date(&self) -> bool;
    /// Returns true if the collection field is present (Some).
    fn has_collection(&self) -> bool;
    /// Returns true if the storage_temp_requirements field is present (Some).
    fn has_storage_temp_requirements(&self) -> bool;
    /// Returns true if the property field is not empty.
    fn has_property(&self) -> bool;
}
