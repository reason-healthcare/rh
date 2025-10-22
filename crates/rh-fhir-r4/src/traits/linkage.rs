use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::resources::linkage::LinkageItem;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Linkage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Identifies two or more records (resource instances) that refer to the same real-world "occurrence".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Linkage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Linkage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LinkageAccessors: DomainResourceAccessors {
    /// Returns a reference to the active field.
    fn active(&self) -> Option<BooleanType>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the item field.
    fn item(&self) -> &[LinkageItem];
}
/// Linkage Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Identifies two or more records (resource instances) that refer to the same real-world "occurrence".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Linkage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Linkage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LinkageMutators: DomainResourceMutators {
    /// Create a new Linkage with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::linkage::Linkage;
    /// use hl7_fhir_r4_core::traits::linkage::LinkageMutators;
    ///
    /// let resource = Linkage::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the active field and returns self for chaining.
    fn set_active(self, value: bool) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<LinkageItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: LinkageItem) -> Self;
}
/// Linkage Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Identifies two or more records (resource instances) that refer to the same real-world "occurrence".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Linkage
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Linkage
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait LinkageExistence: DomainResourceExistence {
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
    /// Returns true if the active field is present (Some).
    fn has_active(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
}
