use crate::bindings::immunization_evaluation_status::ImmunizationEvaluationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ImmunizationEvaluation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those  recommendations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationEvaluation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationEvaluationAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> ImmunizationEvaluationStatus;
    /// Returns a reference to the patient field.
    fn patient(&self) -> Reference;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the authority field.
    fn authority(&self) -> Option<Reference>;
    /// Returns a reference to the targetDisease field.
    fn target_disease(&self) -> CodeableConcept;
    /// Returns a reference to the immunizationEvent field.
    fn immunization_event(&self) -> Reference;
    /// Returns a reference to the doseStatus field.
    fn dose_status(&self) -> CodeableConcept;
    /// Returns a reference to the doseStatusReason field.
    fn dose_status_reason(&self) -> &[CodeableConcept];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the series field.
    fn series(&self) -> Option<StringType>;
}
/// ImmunizationEvaluation Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those  recommendations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationEvaluation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationEvaluationMutators: DomainResourceMutators {
    /// Create a new ImmunizationEvaluation with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::immunization_evaluation::ImmunizationEvaluation;
    /// use hl7_fhir_r4_core::traits::immunization_evaluation::ImmunizationEvaluationMutators;
    ///
    /// let resource = ImmunizationEvaluation::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ImmunizationEvaluationStatus) -> Self;
    /// Sets the patient field and returns self for chaining.
    fn set_patient(self, value: Reference) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the authority field and returns self for chaining.
    fn set_authority(self, value: Reference) -> Self;
    /// Sets the targetDisease field and returns self for chaining.
    fn set_target_disease(self, value: CodeableConcept) -> Self;
    /// Sets the immunizationEvent field and returns self for chaining.
    fn set_immunization_event(self, value: Reference) -> Self;
    /// Sets the doseStatus field and returns self for chaining.
    fn set_dose_status(self, value: CodeableConcept) -> Self;
    /// Sets the doseStatusReason field and returns self for chaining.
    fn set_dose_status_reason(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the doseStatusReason field and returns self for chaining.
    fn add_dose_status_reason(self, item: CodeableConcept) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the series field and returns self for chaining.
    fn set_series(self, value: String) -> Self;
}
/// ImmunizationEvaluation Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those  recommendations.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImmunizationEvaluation
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ImmunizationEvaluationExistence: DomainResourceExistence {
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
    /// Returns true if the patient field is present (Some).
    fn has_patient(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the authority field is present (Some).
    fn has_authority(&self) -> bool;
    /// Returns true if the target_disease field is present (Some).
    fn has_target_disease(&self) -> bool;
    /// Returns true if the immunization_event field is present (Some).
    fn has_immunization_event(&self) -> bool;
    /// Returns true if the dose_status field is present (Some).
    fn has_dose_status(&self) -> bool;
    /// Returns true if the dose_status_reason field is not empty.
    fn has_dose_status_reason(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the series field is present (Some).
    fn has_series(&self) -> bool;
    /// Returns true if the dose_number field is present (Some).
    fn has_dose_number(&self) -> bool;
    /// Returns true if the series_doses field is present (Some).
    fn has_series_doses(&self) -> bool;
}
