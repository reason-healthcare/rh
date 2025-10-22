use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::nutrition_order::NutritionOrderEnteralformula;
use crate::resources::nutrition_order::NutritionOrderOraldiet;
use crate::resources::nutrition_order::NutritionOrderSupplement;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// NutritionOrder Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NutritionOrder
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionOrderAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the instantiatesCanonical field.
    fn instantiates_canonical(&self) -> &[StringType];
    /// Returns a reference to the instantiatesUri field.
    fn instantiates_uri(&self) -> &[StringType];
    /// Returns a reference to the instantiates field.
    fn instantiates(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> RequestStatus;
    /// Returns a reference to the intent field.
    fn intent(&self) -> RequestIntent;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the dateTime field.
    fn date_time(&self) -> DateTimeType;
    /// Returns a reference to the orderer field.
    fn orderer(&self) -> Option<Reference>;
    /// Returns a reference to the allergyIntolerance field.
    fn allergy_intolerance(&self) -> &[Reference];
    /// Returns a reference to the foodPreferenceModifier field.
    fn food_preference_modifier(&self) -> &[CodeableConcept];
    /// Returns a reference to the excludeFoodModifier field.
    fn exclude_food_modifier(&self) -> &[CodeableConcept];
    /// Returns a reference to the oralDiet field.
    fn oral_diet(&self) -> Option<NutritionOrderOraldiet>;
    /// Returns a reference to the supplement field.
    fn supplement(&self) -> &[NutritionOrderSupplement];
    /// Returns a reference to the enteralFormula field.
    fn enteral_formula(&self) -> Option<NutritionOrderEnteralformula>;
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
}
/// NutritionOrder Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NutritionOrder
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionOrderMutators: DomainResourceMutators {
    /// Create a new NutritionOrder with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::nutrition_order::NutritionOrder;
    /// use hl7_fhir_r4_core::traits::nutrition_order::NutritionOrderMutators;
    ///
    /// let resource = NutritionOrder::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the instantiatesCanonical field and returns self for chaining.
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesCanonical field and returns self for chaining.
    fn add_instantiates_canonical(self, item: String) -> Self;
    /// Sets the instantiatesUri field and returns self for chaining.
    fn set_instantiates_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiatesUri field and returns self for chaining.
    fn add_instantiates_uri(self, item: String) -> Self;
    /// Sets the instantiates field and returns self for chaining.
    fn set_instantiates(self, value: Vec<String>) -> Self;
    /// Adds an item to the instantiates field and returns self for chaining.
    fn add_instantiates(self, item: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: RequestStatus) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: RequestIntent) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the dateTime field and returns self for chaining.
    fn set_date_time(self, value: String) -> Self;
    /// Sets the orderer field and returns self for chaining.
    fn set_orderer(self, value: Reference) -> Self;
    /// Sets the allergyIntolerance field and returns self for chaining.
    fn set_allergy_intolerance(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the allergyIntolerance field and returns self for chaining.
    fn add_allergy_intolerance(self, item: Reference) -> Self;
    /// Sets the foodPreferenceModifier field and returns self for chaining.
    fn set_food_preference_modifier(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the foodPreferenceModifier field and returns self for chaining.
    fn add_food_preference_modifier(self, item: CodeableConcept) -> Self;
    /// Sets the excludeFoodModifier field and returns self for chaining.
    fn set_exclude_food_modifier(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the excludeFoodModifier field and returns self for chaining.
    fn add_exclude_food_modifier(self, item: CodeableConcept) -> Self;
    /// Sets the oralDiet field and returns self for chaining.
    fn set_oral_diet(self, value: NutritionOrderOraldiet) -> Self;
    /// Sets the supplement field and returns self for chaining.
    fn set_supplement(self, value: Vec<NutritionOrderSupplement>) -> Self;
    /// Adds an item to the supplement field and returns self for chaining.
    fn add_supplement(self, item: NutritionOrderSupplement) -> Self;
    /// Sets the enteralFormula field and returns self for chaining.
    fn set_enteral_formula(self, value: NutritionOrderEnteralformula) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
}
/// NutritionOrder Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NutritionOrder
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait NutritionOrderExistence: DomainResourceExistence {
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
    /// Returns true if the instantiates_canonical field is not empty.
    fn has_instantiates_canonical(&self) -> bool;
    /// Returns true if the instantiates_uri field is not empty.
    fn has_instantiates_uri(&self) -> bool;
    /// Returns true if the instantiates field is not empty.
    fn has_instantiates(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the date_time field is present (Some).
    fn has_date_time(&self) -> bool;
    /// Returns true if the orderer field is present (Some).
    fn has_orderer(&self) -> bool;
    /// Returns true if the allergy_intolerance field is not empty.
    fn has_allergy_intolerance(&self) -> bool;
    /// Returns true if the food_preference_modifier field is not empty.
    fn has_food_preference_modifier(&self) -> bool;
    /// Returns true if the exclude_food_modifier field is not empty.
    fn has_exclude_food_modifier(&self) -> bool;
    /// Returns true if the oral_diet field is present (Some).
    fn has_oral_diet(&self) -> bool;
    /// Returns true if the supplement field is not empty.
    fn has_supplement(&self) -> bool;
    /// Returns true if the enteral_formula field is present (Some).
    fn has_enteral_formula(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
}
