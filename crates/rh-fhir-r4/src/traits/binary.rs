use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::traits::resource::ResourceAccessors;
use crate::traits::resource::ResourceExistence;
use crate::traits::resource::ResourceMutators;
/// Binary Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Binary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Binary
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BinaryAccessors: ResourceAccessors {
    /// Returns a reference to the contentType field.
    fn content_type(&self) -> Mimetypes;
    /// Returns a reference to the securityContext field.
    fn security_context(&self) -> Option<Reference>;
    /// Returns a reference to the data field.
    fn data(&self) -> Option<Base64BinaryType>;
}
/// Binary Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Binary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Binary
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BinaryMutators: ResourceMutators {
    /// Create a new Binary with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::binary::Binary;
    /// use hl7_fhir_r4_core::traits::binary::BinaryMutators;
    ///
    /// let resource = Binary::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the contentType field and returns self for chaining.
    fn set_content_type(self, value: Mimetypes) -> Self;
    /// Sets the securityContext field and returns self for chaining.
    fn set_security_context(self, value: Reference) -> Self;
    /// Sets the data field and returns self for chaining.
    fn set_data(self, value: String) -> Self;
}
/// Binary Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Binary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Binary
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait BinaryExistence: ResourceExistence {
    /// Returns true if the id field is present (Some).
    fn has_id(&self) -> bool;
    /// Returns true if the meta field is present (Some).
    fn has_meta(&self) -> bool;
    /// Returns true if the implicit_rules field is present (Some).
    fn has_implicit_rules(&self) -> bool;
    /// Returns true if the language field is present (Some).
    fn has_language(&self) -> bool;
    /// Returns true if the content_type field is present (Some).
    fn has_content_type(&self) -> bool;
    /// Returns true if the security_context field is present (Some).
    fn has_security_context(&self) -> bool;
    /// Returns true if the data field is present (Some).
    fn has_data(&self) -> bool;
}
