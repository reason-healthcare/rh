use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::condition_definition::ConditionDefinitionMedication;
use crate::resources::condition_definition::ConditionDefinitionObservation;
use crate::resources::condition_definition::ConditionDefinitionPlan;
use crate::resources::condition_definition::ConditionDefinitionPrecondition;
use crate::resources::condition_definition::ConditionDefinitionQuestionnaire;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ConditionDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A definition of a condition and information relevant to managing it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConditionDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionDefinitionAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the subtitle field.
    fn subtitle(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
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
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the severity field.
    fn severity(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the stage field.
    fn stage(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the hasSeverity field.
    fn has_severity(&self) -> Option<BooleanType>;
    /// Returns a reference to the hasBodySite field.
    fn has_body_site(&self) -> Option<BooleanType>;
    /// Returns a reference to the hasStage field.
    fn has_stage(&self) -> Option<BooleanType>;
    /// Returns a reference to the definition field.
    fn definition(&self) -> &[StringType];
    /// Returns a reference to the observation field.
    fn observation(&self) -> &[ConditionDefinitionObservation];
    /// Returns a reference to the medication field.
    fn medication(&self) -> &[ConditionDefinitionMedication];
    /// Returns a reference to the precondition field.
    fn precondition(&self) -> &[ConditionDefinitionPrecondition];
    /// Returns a reference to the team field.
    fn team(&self) -> &[Reference];
    /// Returns a reference to the questionnaire field.
    fn questionnaire(&self) -> &[ConditionDefinitionQuestionnaire];
    /// Returns a reference to the plan field.
    fn plan(&self) -> &[ConditionDefinitionPlan];
}
/// ConditionDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A definition of a condition and information relevant to managing it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConditionDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionDefinitionMutators: DomainResourceMutators {
    /// Create a new ConditionDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::condition_definition::ConditionDefinition;
    /// use rh_hl7_fhir_r5_core::traits::condition_definition::ConditionDefinitionMutators;
    ///
    /// let resource = ConditionDefinition::new();
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
    /// Sets the subtitle field and returns self for chaining.
    fn set_subtitle(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
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
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the severity field and returns self for chaining.
    fn set_severity(self, value: CodeableConcept) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableConcept) -> Self;
    /// Sets the stage field and returns self for chaining.
    fn set_stage(self, value: CodeableConcept) -> Self;
    /// Sets the hasSeverity field and returns self for chaining.
    fn set_has_severity(self, value: bool) -> Self;
    /// Sets the hasBodySite field and returns self for chaining.
    fn set_has_body_site(self, value: bool) -> Self;
    /// Sets the hasStage field and returns self for chaining.
    fn set_has_stage(self, value: bool) -> Self;
    /// Sets the definition field and returns self for chaining.
    fn set_definition(self, value: Vec<String>) -> Self;
    /// Adds an item to the definition field and returns self for chaining.
    fn add_definition(self, item: String) -> Self;
    /// Sets the observation field and returns self for chaining.
    fn set_observation(self, value: Vec<ConditionDefinitionObservation>) -> Self;
    /// Adds an item to the observation field and returns self for chaining.
    fn add_observation(self, item: ConditionDefinitionObservation) -> Self;
    /// Sets the medication field and returns self for chaining.
    fn set_medication(self, value: Vec<ConditionDefinitionMedication>) -> Self;
    /// Adds an item to the medication field and returns self for chaining.
    fn add_medication(self, item: ConditionDefinitionMedication) -> Self;
    /// Sets the precondition field and returns self for chaining.
    fn set_precondition(self, value: Vec<ConditionDefinitionPrecondition>) -> Self;
    /// Adds an item to the precondition field and returns self for chaining.
    fn add_precondition(self, item: ConditionDefinitionPrecondition) -> Self;
    /// Sets the team field and returns self for chaining.
    fn set_team(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the team field and returns self for chaining.
    fn add_team(self, item: Reference) -> Self;
    /// Sets the questionnaire field and returns self for chaining.
    fn set_questionnaire(self, value: Vec<ConditionDefinitionQuestionnaire>) -> Self;
    /// Adds an item to the questionnaire field and returns self for chaining.
    fn add_questionnaire(self, item: ConditionDefinitionQuestionnaire) -> Self;
    /// Sets the plan field and returns self for chaining.
    fn set_plan(self, value: Vec<ConditionDefinitionPlan>) -> Self;
    /// Adds an item to the plan field and returns self for chaining.
    fn add_plan(self, item: ConditionDefinitionPlan) -> Self;
}
/// ConditionDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A definition of a condition and information relevant to managing it.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConditionDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ConditionDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the subtitle field is present (Some).
    fn has_subtitle(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
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
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the severity field is present (Some).
    fn has_severity(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the stage field is present (Some).
    fn has_stage(&self) -> bool;
    /// Returns true if the has_severity field is present (Some).
    fn has_has_severity(&self) -> bool;
    /// Returns true if the has_body_site field is present (Some).
    fn has_has_body_site(&self) -> bool;
    /// Returns true if the has_stage field is present (Some).
    fn has_has_stage(&self) -> bool;
    /// Returns true if the definition field is not empty.
    fn has_definition(&self) -> bool;
    /// Returns true if the observation field is not empty.
    fn has_observation(&self) -> bool;
    /// Returns true if the medication field is not empty.
    fn has_medication(&self) -> bool;
    /// Returns true if the precondition field is not empty.
    fn has_precondition(&self) -> bool;
    /// Returns true if the team field is not empty.
    fn has_team(&self) -> bool;
    /// Returns true if the questionnaire field is not empty.
    fn has_questionnaire(&self) -> bool;
    /// Returns true if the plan field is not empty.
    fn has_plan(&self) -> bool;
}
