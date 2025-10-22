use crate::datatypes::meta::Meta;
use crate::primitives::string::StringType;
/// Resource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This is the base resource type for everything.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Resource
pub trait ResourceAccessors {
    /// Returns a reference to the id field.
    fn id(&self) -> Option<StringType>;
    /// Returns a reference to the meta field.
    fn meta(&self) -> Option<Meta>;
    /// Returns a reference to the implicitRules field.
    fn implicit_rules(&self) -> Option<StringType>;
    /// Returns a reference to the language field.
    fn language(&self) -> Option<StringType>;
}
/// Resource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This is the base resource type for everything.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Resource
pub trait ResourceMutators {
    /// Create a new Resource with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::resource::Resource;
    /// use hl7_fhir_r4_core::traits::resource::ResourceMutators;
    ///
    /// let resource = Resource::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the id field and returns self for chaining.
    fn set_id(self, value: String) -> Self;
    /// Sets the meta field and returns self for chaining.
    fn set_meta(self, value: Meta) -> Self;
    /// Sets the implicitRules field and returns self for chaining.
    fn set_implicit_rules(self, value: String) -> Self;
    /// Sets the language field and returns self for chaining.
    fn set_language(self, value: String) -> Self;
}
/// Resource Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This is the base resource type for everything.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Resource
pub trait ResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
}
