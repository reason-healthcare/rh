use crate::bindings::formularyitem_status::FormularyitemStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// FormularyItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource describes a product or service that is available through a program and includes the conditions and constraints of availability.  All of the information in this resource is specific to the inclusion of the item in the formulary and is not inherent to the item itself.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FormularyItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: FormularyItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FormularyItemAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> Option<FormularyitemStatus>;
}
/// FormularyItem Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource describes a product or service that is available through a program and includes the conditions and constraints of availability.  All of the information in this resource is specific to the inclusion of the item in the formulary and is not inherent to the item itself.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FormularyItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: FormularyItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FormularyItemMutators: DomainResourceMutators {
    /// Create a new FormularyItem with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::formulary_item::FormularyItem;
    /// use rh_hl7_fhir_r5_core::traits::formulary_item::FormularyItemMutators;
    ///
    /// let resource = FormularyItem::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: FormularyitemStatus) -> Self;
}
/// FormularyItem Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource describes a product or service that is available through a program and includes the conditions and constraints of availability.  All of the information in this resource is specific to the inclusion of the item in the formulary and is not inherent to the item itself.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/FormularyItem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: FormularyItem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait FormularyItemExistence: DomainResourceExistence {
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
}
