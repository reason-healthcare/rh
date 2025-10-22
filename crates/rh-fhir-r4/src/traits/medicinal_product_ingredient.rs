use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::resources::medicinal_product_ingredient::MedicinalProductIngredientSpecifiedsubstance;
use crate::resources::medicinal_product_ingredient::MedicinalProductIngredientSubstance;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// MedicinalProductIngredient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIngredient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIngredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIngredientAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the role field.
    fn role(&self) -> CodeableConcept;
    /// Returns a reference to the allergenicIndicator field.
    fn allergenic_indicator(&self) -> Option<BooleanType>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the specifiedSubstance field.
    fn specified_substance(&self) -> &[MedicinalProductIngredientSpecifiedsubstance];
    /// Returns a reference to the substance field.
    fn substance(&self) -> Option<MedicinalProductIngredientSubstance>;
}
/// MedicinalProductIngredient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIngredient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIngredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIngredientMutators: DomainResourceMutators {
    /// Create a new MedicinalProductIngredient with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::medicinal_product_ingredient::MedicinalProductIngredient;
    /// use hl7_fhir_r4_core::traits::medicinal_product_ingredient::MedicinalProductIngredientMutators;
    ///
    /// let resource = MedicinalProductIngredient::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the role field and returns self for chaining.
    fn set_role(self, value: CodeableConcept) -> Self;
    /// Sets the allergenicIndicator field and returns self for chaining.
    fn set_allergenic_indicator(self, value: bool) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the specifiedSubstance field and returns self for chaining.
    fn set_specified_substance(
        self,
        value: Vec<MedicinalProductIngredientSpecifiedsubstance>,
    ) -> Self;
    /// Adds an item to the specifiedSubstance field and returns self for chaining.
    fn add_specified_substance(self, item: MedicinalProductIngredientSpecifiedsubstance) -> Self;
    /// Sets the substance field and returns self for chaining.
    fn set_substance(self, value: MedicinalProductIngredientSubstance) -> Self;
}
/// MedicinalProductIngredient Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIngredient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: MedicinalProductIngredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait MedicinalProductIngredientExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the role field is present (Some).
    fn has_role(&self) -> bool;
    /// Returns true if the allergenic_indicator field is present (Some).
    fn has_allergenic_indicator(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the specified_substance field is not empty.
    fn has_specified_substance(&self) -> bool;
    /// Returns true if the substance field is present (Some).
    fn has_substance(&self) -> bool;
}
