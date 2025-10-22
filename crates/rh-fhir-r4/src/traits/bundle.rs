use crate::bindings::bundle_type::BundleType;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::signature::Signature;
use crate::primitives::instant::InstantType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::bundle::BundleEntry;
use crate::resources::bundle::BundleLink;
use crate::traits::resource::ResourceAccessors;
use crate::traits::resource::ResourceExistence;
use crate::traits::resource::ResourceMutators;
/// Bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A container for a collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Bundle
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BundleAccessors: ResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the type field.
    fn type_(&self) -> BundleType;
    /// Returns a reference to the timestamp field.
    fn timestamp(&self) -> Option<InstantType>;
    /// Returns a reference to the total field.
    fn total(&self) -> Option<UnsignedIntType>;
    /// Returns a reference to the link field.
    fn link(&self) -> &[BundleLink];
    /// Returns a reference to the entry field.
    fn entry(&self) -> &[BundleEntry];
    /// Returns a reference to the signature field.
    fn signature(&self) -> Option<Signature>;
}
/// Bundle Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A container for a collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Bundle
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BundleMutators: ResourceMutators {
    /// Create a new Bundle with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::bundle::Bundle;
    /// use hl7_fhir_r4_core::traits::bundle::BundleMutators;
    ///
    /// let resource = Bundle::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the type field and returns self for chaining.
    fn set_type_(self, value: BundleType) -> Self;
    /// Sets the timestamp field and returns self for chaining.
    fn set_timestamp(self, value: String) -> Self;
    /// Sets the total field and returns self for chaining.
    fn set_total(self, value: i32) -> Self;
    /// Sets the link field and returns self for chaining.
    fn set_link(self, value: Vec<BundleLink>) -> Self;
    /// Adds an item to the link field and returns self for chaining.
    fn add_link(self, item: BundleLink) -> Self;
    /// Sets the entry field and returns self for chaining.
    fn set_entry(self, value: Vec<BundleEntry>) -> Self;
    /// Adds an item to the entry field and returns self for chaining.
    fn add_entry(self, item: BundleEntry) -> Self;
    /// Sets the signature field and returns self for chaining.
    fn set_signature(self, value: Signature) -> Self;
}
/// Bundle Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A container for a collection of resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Bundle
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Bundle
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BundleExistence: ResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the type_ field is present (Some).
    fn has_type_(&self) -> bool;
    /// Returns true if the timestamp field is present (Some).
    fn has_timestamp(&self) -> bool;
    /// Returns true if the total field is present (Some).
    fn has_total(&self) -> bool;
    /// Returns true if the link field is not empty.
    fn has_link(&self) -> bool;
    /// Returns true if the entry field is not empty.
    fn has_entry(&self) -> bool;
    /// Returns true if the signature field is present (Some).
    fn has_signature(&self) -> bool;
}
