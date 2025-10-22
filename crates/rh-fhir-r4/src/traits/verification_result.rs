use crate::bindings::verificationresult_status::VerificationresultStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::verification_result::VerificationResultAttestation;
use crate::resources::verification_result::VerificationResultPrimarysource;
use crate::resources::verification_result::VerificationResultValidator;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// VerificationResult Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes validation requirements, source(s), status and dates for one or more elements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VerificationResult
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VerificationResultAccessors: DomainResourceAccessors {
    /// Returns a reference to the target field.
    fn target(&self) -> &[Reference];
    /// Returns a reference to the targetLocation field.
    fn target_location(&self) -> &[StringType];
    /// Returns a reference to the need field.
    fn need(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the status field.
    fn status(&self) -> VerificationresultStatus;
    /// Returns a reference to the statusDate field.
    fn status_date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the validationType field.
    fn validation_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the validationProcess field.
    fn validation_process(&self) -> &[CodeableConcept];
    /// Returns a reference to the frequency field.
    fn frequency(&self) -> Option<Timing>;
    /// Returns a reference to the lastPerformed field.
    fn last_performed(&self) -> Option<DateTimeType>;
    /// Returns a reference to the nextScheduled field.
    fn next_scheduled(&self) -> Option<StringType>;
    /// Returns a reference to the failureAction field.
    fn failure_action(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the primarySource field.
    fn primary_source(&self) -> &[VerificationResultPrimarysource];
    /// Returns a reference to the attestation field.
    fn attestation(&self) -> Option<VerificationResultAttestation>;
    /// Returns a reference to the validator field.
    fn validator(&self) -> &[VerificationResultValidator];
}
/// VerificationResult Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Describes validation requirements, source(s), status and dates for one or more elements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VerificationResult
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VerificationResultMutators: DomainResourceMutators {
    /// Create a new VerificationResult with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::verification_result::VerificationResult;
    /// use hl7_fhir_r4_core::traits::verification_result::VerificationResultMutators;
    ///
    /// let resource = VerificationResult::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the target field and returns self for chaining.
    fn set_target(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the target field and returns self for chaining.
    fn add_target(self, item: Reference) -> Self;
    /// Sets the targetLocation field and returns self for chaining.
    fn set_target_location(self, value: Vec<String>) -> Self;
    /// Adds an item to the targetLocation field and returns self for chaining.
    fn add_target_location(self, item: String) -> Self;
    /// Sets the need field and returns self for chaining.
    fn set_need(self, value: CodeableConcept) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: VerificationresultStatus) -> Self;
    /// Sets the statusDate field and returns self for chaining.
    fn set_status_date(self, value: String) -> Self;
    /// Sets the validationType field and returns self for chaining.
    fn set_validation_type(self, value: CodeableConcept) -> Self;
    /// Sets the validationProcess field and returns self for chaining.
    fn set_validation_process(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the validationProcess field and returns self for chaining.
    fn add_validation_process(self, item: CodeableConcept) -> Self;
    /// Sets the frequency field and returns self for chaining.
    fn set_frequency(self, value: Timing) -> Self;
    /// Sets the lastPerformed field and returns self for chaining.
    fn set_last_performed(self, value: String) -> Self;
    /// Sets the nextScheduled field and returns self for chaining.
    fn set_next_scheduled(self, value: String) -> Self;
    /// Sets the failureAction field and returns self for chaining.
    fn set_failure_action(self, value: CodeableConcept) -> Self;
    /// Sets the primarySource field and returns self for chaining.
    fn set_primary_source(self, value: Vec<VerificationResultPrimarysource>) -> Self;
    /// Adds an item to the primarySource field and returns self for chaining.
    fn add_primary_source(self, item: VerificationResultPrimarysource) -> Self;
    /// Sets the attestation field and returns self for chaining.
    fn set_attestation(self, value: VerificationResultAttestation) -> Self;
    /// Sets the validator field and returns self for chaining.
    fn set_validator(self, value: Vec<VerificationResultValidator>) -> Self;
    /// Adds an item to the validator field and returns self for chaining.
    fn add_validator(self, item: VerificationResultValidator) -> Self;
}
/// VerificationResult Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Describes validation requirements, source(s), status and dates for one or more elements.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/VerificationResult
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: VerificationResult
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait VerificationResultExistence: DomainResourceExistence {
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
    /// Returns true if the target field is not empty.
    fn has_target(&self) -> bool;
    /// Returns true if the target_location field is not empty.
    fn has_target_location(&self) -> bool;
    /// Returns true if the need field is present (Some).
    fn has_need(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the status_date field is present (Some).
    fn has_status_date(&self) -> bool;
    /// Returns true if the validation_type field is present (Some).
    fn has_validation_type(&self) -> bool;
    /// Returns true if the validation_process field is not empty.
    fn has_validation_process(&self) -> bool;
    /// Returns true if the frequency field is present (Some).
    fn has_frequency(&self) -> bool;
    /// Returns true if the last_performed field is present (Some).
    fn has_last_performed(&self) -> bool;
    /// Returns true if the next_scheduled field is present (Some).
    fn has_next_scheduled(&self) -> bool;
    /// Returns true if the failure_action field is present (Some).
    fn has_failure_action(&self) -> bool;
    /// Returns true if the primary_source field is not empty.
    fn has_primary_source(&self) -> bool;
    /// Returns true if the attestation field is present (Some).
    fn has_attestation(&self) -> bool;
    /// Returns true if the validator field is not empty.
    fn has_validator(&self) -> bool;
}
