use crate::bindings::substance_status::SubstanceStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::primitives::string::StringType;
use crate::resources::substance::SubstanceIngredient;
use crate::resources::substance::SubstanceInstance;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Substance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A homogeneous material with a definite composition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Substance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Substance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> Option<SubstanceStatus>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the instance field.
    fn instance(&self) -> &[SubstanceInstance];
    /// Returns a reference to the ingredient field.
    fn ingredient(&self) -> &[SubstanceIngredient];
}
/// Substance Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A homogeneous material with a definite composition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Substance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Substance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceMutators: DomainResourceMutators {
    /// Create a new Substance with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::substance::Substance;
    /// use hl7_fhir_r4_core::traits::substance::SubstanceMutators;
    ///
    /// let resource = Substance::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: SubstanceStatus) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the instance field and returns self for chaining.
    fn set_instance(self, value: Vec<SubstanceInstance>) -> Self;
    /// Adds an item to the instance field and returns self for chaining.
    fn add_instance(self, item: SubstanceInstance) -> Self;
    /// Sets the ingredient field and returns self for chaining.
    fn set_ingredient(self, value: Vec<SubstanceIngredient>) -> Self;
    /// Adds an item to the ingredient field and returns self for chaining.
    fn add_ingredient(self, item: SubstanceIngredient) -> Self;
}
/// Substance Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A homogeneous material with a definite composition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Substance
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Substance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait SubstanceExistence: DomainResourceExistence {
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
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the instance field is not empty.
    fn has_instance(&self) -> bool;
    /// Returns true if the ingredient field is not empty.
    fn has_ingredient(&self) -> bool;
}
