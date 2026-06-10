use crate::bindings::nutritionproduct_status::NutritionproductStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::reference::Reference;
use crate::resources::nutrition_product::NutritionProductCharacteristic;
use crate::resources::nutrition_product::NutritionProductIngredient;
use crate::resources::nutrition_product::NutritionProductInstance;
use crate::resources::nutrition_product::NutritionProductNutrient;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// NutritionProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A food or supplement that is consumed by patients.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionProductAccessors: DomainResourceAccessors {
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> NutritionproductStatus;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the manufacturer field.
    fn manufacturer(&self) -> &[Reference];
    /// Returns a reference to the nutrient field.
    fn nutrient(&self) -> &[NutritionProductNutrient];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[NutritionProductIngredient];
    /// Returns a reference to the knownAllergen field.
    fn known_allergen(&self) -> &[CodeableReference];
    /// Returns a reference to the characteristic field.
    fn characteristic(&self) -> &[NutritionProductCharacteristic];
    /// Returns a reference to the instance field.
    fn instance(&self) -> &[NutritionProductInstance];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// NutritionProduct Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A food or supplement that is consumed by patients.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionProductMutators: DomainResourceMutators {
    /// Create a new NutritionProduct with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::nutrition_product::NutritionProduct;
    /// use rh_hl7_fhir_r5_core::traits::nutrition_product::NutritionProductMutators;
    ///
    /// let resource = NutritionProduct::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: NutritionproductStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the manufacturer field and returns self for chaining.
    fn set_manufacturer(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the manufacturer field and returns self for chaining.
    fn add_manufacturer(self, item: Reference) -> Self;
    /// Sets the nutrient field and returns self for chaining.
    fn set_nutrient(self, value: Vec<NutritionProductNutrient>) -> Self;
    /// Adds an item to the nutrient field and returns self for chaining.
    fn add_nutrient(self, item: NutritionProductNutrient) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<NutritionProductIngredient>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: NutritionProductIngredient) -> Self;
    /// Sets the knownAllergen field and returns self for chaining.
    fn set_known_allergen(self, value: Vec<CodeableReference>) -> Self;
    /// Adds an item to the knownAllergen field and returns self for chaining.
    fn add_known_allergen(self, item: CodeableReference) -> Self;
    /// Sets the characteristic field and returns self for chaining.
    fn set_characteristic(self, value: Vec<NutritionProductCharacteristic>) -> Self;
    /// Adds an item to the characteristic field and returns self for chaining.
    fn add_characteristic(self, item: NutritionProductCharacteristic) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: Vec<NutritionProductInstance>) -> Self;
    /// Adds an item to the instance field and returns self for chaining.
    fn add_instance(self, item: NutritionProductInstance) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// NutritionProduct Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A food or supplement that is consumed by patients.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NutritionProduct
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionProductExistence: DomainResourceExistence {
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the manufacturer field is not empty.
    fn has_manufacturer(&self) -> bool;
    /// Returns true if the nutrient field is not empty.
    fn has_nutrient(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
    /// Returns true if the known_allergen field is not empty.
    fn has_known_allergen(&self) -> bool;
    /// Returns true if the characteristic field is not empty.
    fn has_characteristic(&self) -> bool;
    /// Returns true if the instance field is not empty.
    fn has_instance(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
