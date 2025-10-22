use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::questionnaire::QuestionnaireItem;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// Questionnaire Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the derivedFrom field.
    fn derived_from(&self) -> &[StringType];
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the subjectType field.
    fn subject_type(&self) -> &[ResourceTypes];
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the code field.
    fn code(&self) -> &[Coding];
    /// Returns a reference to the item field.
    fn item(&self) -> &[QuestionnaireItem];
}
/// Questionnaire Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireMutators: DomainResourceMutators {
    /// Create a new Questionnaire with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::questionnaire::Questionnaire;
    /// use hl7_fhir_r4_core::traits::questionnaire::QuestionnaireMutators;
    ///
    /// let resource = Questionnaire::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the derivedFrom field and returns self for chaining.
    fn set_derived_from(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFrom field and returns self for chaining.
    fn add_derived_from(self, item: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
    /// Sets the subjectType field and returns self for chaining.
    fn set_subject_type(self, value: Vec<ResourceTypes>) -> Self;
    /// Adds an item to the subjectType field and returns self for chaining.
    fn add_subject_type(self, item: ResourceTypes) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the code field and returns self for chaining.
    fn add_code(self, item: Coding) -> Self;
    /// Sets the item field and returns self for chaining.
    fn set_item(self, value: Vec<QuestionnaireItem>) -> Self;
    /// Adds an item to the item field and returns self for chaining.
    fn add_item(self, item: QuestionnaireItem) -> Self;
}
/// Questionnaire Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait QuestionnaireExistence: DomainResourceExistence {
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
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the derived_from field is not empty.
    fn has_derived_from(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the subject_type field is not empty.
    fn has_subject_type(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the code field is not empty.
    fn has_code(&self) -> bool;
    /// Returns true if the item field is not empty.
    fn has_item(&self) -> bool;
}
