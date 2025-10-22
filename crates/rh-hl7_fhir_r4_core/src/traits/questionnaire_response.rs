use crate::bindings::questionnaire_answers_status::QuestionnaireAnswersStatus;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::questionnaire_response::QuestionnaireResponseItem;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// QuestionnaireResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: QuestionnaireResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireResponseAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the basedOn field.
    fn based_on(&self) -> &[Reference];
    /// Returns a reference to the partOf field.
    fn part_of(&self) -> &[Reference];
    /// Returns a reference to the questionnaire field.
    fn questionnaire(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> QuestionnaireAnswersStatus;
    /// Returns a reference to the subject field.
    fn subject(&self) -> Option<Reference>;
    /// Returns a reference to the encounter field.
    fn encounter(&self) -> Option<Reference>;
    /// Returns a reference to the authored field.
    fn authored(&self) -> Option<DateTimeType>;
    /// Returns a reference to the author field.
    fn author(&self) -> Option<Reference>;
    /// Returns a reference to the source field.
    fn source(&self) -> Option<Reference>;
    /// Returns a reference to the item field.
    fn item(&self) -> &[QuestionnaireResponseItem];
}
/// QuestionnaireResponse Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: QuestionnaireResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireResponseMutators: DomainResourceMutators {
    /// Create a new QuestionnaireResponse with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::questionnaire_response::QuestionnaireResponse;
    /// use hl7_fhir_r4_core::traits::questionnaire_response::QuestionnaireResponseMutators;
    ///
    /// let resource = QuestionnaireResponse::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the basedOn field and returns self for chaining.
    fn set_based_on(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the basedOn field and returns self for chaining.
    fn add_based_on(self, item: Reference) -> Self;
    /// Sets the partOf field and returns self for chaining.
    fn set_part_of(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the partOf field and returns self for chaining.
    fn add_part_of(self, item: Reference) -> Self;
    /// Sets the questionnaire field and returns self for chaining.
    fn set_questionnaire(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: QuestionnaireAnswersStatus) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Reference) -> Self;
    /// Sets the encounter field and returns self for chaining.
    fn set_encounter(self, value: Reference) -> Self;
    /// Sets the authored field and returns self for chaining.
    fn set_authored(self, value: String) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Reference) -> Self;
    /// Sets the source field and returns self for chaining.
    fn set_source(self, value: Reference) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<QuestionnaireResponseItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: QuestionnaireResponseItem) -> Self;
}
/// QuestionnaireResponse Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: QuestionnaireResponse
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireResponseExistence: DomainResourceExistence {
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
    /// Returns true if the based_on field is not empty.
    fn has_based_on(&self) -> bool;
    /// Returns true if the part_of field is not empty.
    fn has_part_of(&self) -> bool;
    /// Returns true if the questionnaire field is present (Some).
    fn has_questionnaire(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
    /// Returns true if the encounter field is present (Some).
    fn has_encounter(&self) -> bool;
    /// Returns true if the authored field is present (Some).
    fn has_authored(&self) -> bool;
    /// Returns true if the author field is present (Some).
    fn has_author(&self) -> bool;
    /// Returns true if the source field is present (Some).
    fn has_source(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
}
