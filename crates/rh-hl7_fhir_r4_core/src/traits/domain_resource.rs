use crate::datatypes::extension::Extension;
use crate::datatypes::narrative::Narrative;
use crate::resources::resource::Resource;
use crate::traits::resource::ResourceAccessors;
use crate::traits::resource::ResourceExistence;
use crate::traits::resource::ResourceMutators;
/// DomainResource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource that includes narrative, extensions, and contained resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DomainResource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait DomainResourceAccessors: ResourceAccessors {
    /// Returns a reference to the text field.
    fn text(&self) -> Option<Narrative>;
    /// Returns a reference to the contained field.
    fn contained(&self) -> &[Resource];
    /// Returns a reference to the extension field.
    fn extension(&self) -> &[Extension];
    /// Returns a reference to the modifierExtension field.
    fn modifier_extension(&self) -> &[Extension];
}
/// DomainResource Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A resource that includes narrative, extensions, and contained resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DomainResource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait DomainResourceMutators: ResourceMutators {
    /// Create a new DomainResource with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::domain_resource::DomainResource;
    /// use hl7_fhir_r4_core::traits::domain_resource::DomainResourceMutators;
    ///
    /// let resource = DomainResource::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the text field and returns self for chaining.
    fn set_text(self, value: Narrative) -> Self;
    /// Sets the contained field and returns self for chaining.
    fn set_contained(self, value: Vec<Resource>) -> Self;
    /// Adds an item to the contained field and returns self for chaining.
    fn add_contained(self, item: Resource) -> Self;
    /// Sets the extension field and returns self for chaining.
    fn set_extension(self, value: Vec<Extension>) -> Self;
    /// Adds an item to the extension field and returns self for chaining.
    fn add_extension(self, item: Extension) -> Self;
    /// Sets the modifierExtension field and returns self for chaining.
    fn set_modifier_extension(self, value: Vec<Extension>) -> Self;
    /// Adds an item to the modifierExtension field and returns self for chaining.
    fn add_modifier_extension(self, item: Extension) -> Self;
}
/// DomainResource Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A resource that includes narrative, extensions, and contained resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DomainResource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
pub trait DomainResourceExistence: ResourceExistence {
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
}
