use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::resources::immunization_recommendation::ImmunizationRecommendationRecommendation;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ImmunizationRecommendation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationRecommendation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationRecommendationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the date field.
    fn date(&self) -> DateTimeType;
    /// Returns a reference to the authority field.
    fn authority(&self) -> Option<Reference>;
    /// Returns a reference to the recommendation field.
    fn recommendation(&self) -> &[ImmunizationRecommendationRecommendation];
}
/// ImmunizationRecommendation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationRecommendation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationRecommendationMutators: DomainResourceMutators {
    /// Create a new ImmunizationRecommendation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::immunization_recommendation::ImmunizationRecommendation;
    /// use hl7_fhir_r4_core::traits::immunization_recommendation::ImmunizationRecommendationMutators;
    ///
    /// let resource = ImmunizationRecommendation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the authority field and returns self for chaining.
    fn set_authority(self, value: Reference) -> Self;
    /// Sets the recommendation field and returns self for chaining.
    fn set_recommendation(self, value: Vec<ImmunizationRecommendationRecommendation>) -> Self;
    /// Adds an item to the recommendation field and returns self for chaining.
    fn add_recommendation(self, item: ImmunizationRecommendationRecommendation) -> Self;
}
/// ImmunizationRecommendation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationRecommendation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationRecommendationExistence: DomainResourceExistence {
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
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the authority field is present (Some).
    fn has_authority(&self) -> bool;
    /// Returns true if the recommendation field is not empty.
    fn has_recommendation(&self) -> bool;
}
