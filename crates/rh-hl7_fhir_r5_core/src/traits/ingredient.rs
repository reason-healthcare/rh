use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::string::StringType;
use crate::resources::ingredient::IngredientManufacturer;
use crate::resources::ingredient::IngredientSubstance;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Ingredient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Ingredient
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Ingredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait IngredientAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the for field.
    fn for_(&self) -> &[Reference];
    /// Returns a reference to the role field.
    fn role(&self) -> CodeableConcept;
    /// Returns a reference to the function field.
    fn function(&self) -> &[CodeableConcept];
    /// Returns a reference to the group field.
    fn group(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the allergenicIndicator field.
    fn allergenic_indicator(&self) -> Option<BooleanType>;
    /// Returns a reference to the comment field.
    fn comment(&self) -> Option<StringType>;
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[IngredientManufacturer];
    /// Returns a reference to the substance field.
    fn substance(&self) -> IngredientSubstance;
}
/// Ingredient Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Ingredient
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Ingredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait IngredientMutators: DomainResourceMutators {
    /// Create a new Ingredient with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::ingredient::Ingredient;
    /// use rh_hl7_fhir_r5_core::traits::ingredient::IngredientMutators;
    ///
    /// let resource = Ingredient::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the for field and returns self for chaining.
    fn set_for_(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the for field and returns self for chaining.
    fn add_for_(self, item: Reference) -> Self;
    /// Sets the role field and returns self for chaining.
    fn set_role(self, value: CodeableConcept) -> Self;
    /// Sets the function field and returns self for chaining.
    fn set_function(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the function field and returns self for chaining.
    fn add_function(self, item: CodeableConcept) -> Self;
    /// Sets the group field and returns self for chaining.
    fn set_group(self, value: CodeableConcept) -> Self;
    /// Sets the allergenicIndicator field and returns self for chaining.
    fn set_allergenic_indicator(self, value: bool) -> Self;
    /// Sets the comment field and returns self for chaining.
    fn set_comment(self, value: String) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<IngredientManufacturer>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: IngredientManufacturer) -> Self;
    /// Sets the substance field and returns self for chaining.
    fn set_substance(self, value: IngredientSubstance) -> Self;
}
/// Ingredient Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// An ingredient of a manufactured item or pharmaceutical product.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Ingredient
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Ingredient
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait IngredientExistence: DomainResourceExistence {
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the for_ field is not empty.
    fn has_for_(&self) -> bool;
    /// Returns true if the role field is present (Some).
    fn has_role(&self) -> bool;
    /// Returns true if the function field is not empty.
    fn has_function(&self) -> bool;
    /// Returns true if the group field is present (Some).
    fn has_group(&self) -> bool;
    /// Returns true if the allergenic_indicator field is present (Some).
    fn has_allergenic_indicator(&self) -> bool;
    /// Returns true if the comment field is present (Some).
    fn has_comment(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the substance field is present (Some).
    fn has_substance(&self) -> bool;
}
