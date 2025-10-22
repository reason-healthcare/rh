use crate::traits::resource::ResourceExistence;
/// CQF-Questionnaire Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A questionnaire with the ability to specify behavior associated with questions or groups of questions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Questionnaire
pub trait CQFQuestionnaireAccessors {}
/// CQF-Questionnaire Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A questionnaire with the ability to specify behavior associated with questions or groups of questions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Questionnaire
pub trait CQFQuestionnaireMutators {
    /// Create a new CQFQuestionnaire with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::profiles::cqf_questionnaire::CQFQuestionnaire;
    /// use hl7_fhir_r4_core::traits::cqf_questionnaire::CQFQuestionnaireMutators;
    ///
    /// let resource = CQFQuestionnaire::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
}
/// cqf-questionnaire Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A questionnaire with the ability to specify behavior associated with questions or groups of questions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Questionnaire
pub trait CQFQuestionnaireExistence: ResourceExistence {}
