use crate::bindings::guidance_response_status::GuidanceResponseStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// GuidanceResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GuidanceResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the requestIdentifier field.
    fn request_identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the status field.
    fn status(&self) -> GuidanceResponseStatus;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the occurrenceDateTime field.
    fn occurrence_date_time(&self) -> Option<DateTimeType>;
    /// Returns a reference to the performer field.
    fn performer(&self) -> Option<Reference>;
    /// Returns a reference to the reasonCode field.
    fn reason_code(&self) -> &[CodeableConcept];
    /// Returns a reference to the reasonReference field.
    fn reason_reference(&self) -> &[Reference];
    /// Returns a reference to the note field.
    fn note(&self) -> &[Annotation];
    /// Returns a reference to the evaluationMessage field.
    fn evaluation_message(&self) -> &[Reference];
    /// Returns a reference to the outputParameters field.
    fn output_parameters(&self) -> Option<Reference>;
    /// Returns a reference to the result field.
    fn result(&self) -> Option<Reference>;
    /// Returns a reference to the dataRequirement field.
    fn data_requirement(&self) -> &[DataRequirement];
}
/// GuidanceResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GuidanceResponseMutators: DomainResourceMutators {
    /// Create a new GuidanceResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::guidance_response::GuidanceResponse;
    /// use hl7_fhir_r4_core::traits::guidance_response::GuidanceResponseMutators;
    ///
    /// let resource = GuidanceResponse::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the requestIdentifier field and returns self for chaining.
    fn set_request_identifier(self, value: Identifier) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: GuidanceResponseStatus) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the occurrenceDateTime field and returns self for chaining.
    fn set_occurrence_date_time(self, value: String) -> Self;
    /// Sets the performer field and returns self for chaining.
    fn set_performer(self, value: Reference) -> Self;
    /// Sets the reasonCode field and returns self for chaining.
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the reasonCode field and returns self for chaining.
    fn add_reason_code(self, item: CodeableConcept) -> Self;
    /// Sets the reasonReference field and returns self for chaining.
    fn set_reason_reference(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the reasonReference field and returns self for chaining.
    fn add_reason_reference(self, item: Reference) -> Self;
    /// Sets the note field and returns self for chaining.
    fn set_note(self, value: Vec<Annotation>) -> Self;
    /// Adds an item to the note field and returns self for chaining.
    fn add_note(self, item: Annotation) -> Self;
    /// Sets the evaluationMessage field and returns self for chaining.
    fn set_evaluation_message(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the evaluationMessage field and returns self for chaining.
    fn add_evaluation_message(self, item: Reference) -> Self;
    /// Sets the outputParameters field and returns self for chaining.
    fn set_output_parameters(self, value: Reference) -> Self;
    /// Sets the result field and returns self for chaining.
    fn set_result(self, value: Reference) -> Self;
    /// Sets the dataRequirement field and returns self for chaining.
    fn set_data_requirement(self, value: Vec<DataRequirement>) -> Self;
    /// Adds an item to the dataRequirement field and returns self for chaining.
    fn add_data_requirement(self, item: DataRequirement) -> Self;
}
/// GuidanceResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GuidanceResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GuidanceResponseExistence: DomainResourceExistence {
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
    /// Returns true if the request_identifier field is present (Some).
    fn has_request_identifier(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the module field is present (Some).
    fn has_module(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the occurrence_date_time field is present (Some).
    fn has_occurrence_date_time(&self) -> bool;
    /// Returns true if the performer field is present (Some).
    fn has_performer(&self) -> bool;
    /// Returns true if the reason_code field is not empty.
    fn has_reason_code(&self) -> bool;
    /// Returns true if the reason_reference field is not empty.
    fn has_reason_reference(&self) -> bool;
    /// Returns true if the note field is not empty.
    fn has_note(&self) -> bool;
    /// Returns true if the evaluation_message field is not empty.
    fn has_evaluation_message(&self) -> bool;
    /// Returns true if the output_parameters field is present (Some).
    fn has_output_parameters(&self) -> bool;
    /// Returns true if the result field is present (Some).
    fn has_result(&self) -> bool;
    /// Returns true if the data_requirement field is not empty.
    fn has_data_requirement(&self) -> bool;
}
