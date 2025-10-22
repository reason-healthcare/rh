use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_resource_types::RequestResourceTypes;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::activity_definition::ActivityDefinitionDynamicvalue;
use crate::resources::activity_definition::ActivityDefinitionParticipant;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActivityDefinitionAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the usage field.
    fn usage(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the topic field.
    fn topic(&self) -> &[CodeableConcept];
    /// Returns a reference to the author field.
    fn author(&self) -> &[ContactDetail];
    /// Returns a reference to the editor field.
    fn editor(&self) -> &[ContactDetail];
    /// Returns a reference to the reviewer field.
    fn reviewer(&self) -> &[ContactDetail];
    /// Returns a reference to the endorser field.
    fn endorser(&self) -> &[ContactDetail];
    /// Returns a reference to the relatedArtifact field.
    fn related_artifact(&self) -> &[RelatedArtifact];
    /// Returns a reference to the library field.
    fn library(&self) -> &[StringType];
    /// Returns a reference to the kind field.
    fn kind(&self) -> Option<RequestResourceTypes>;
    /// Returns a reference to the profile field.
    fn profile(&self) -> Option<StringType>;
    /// Returns a reference to the code field.
    fn code(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the intent field.
    fn intent(&self) -> Option<RequestIntent>;
    /// Returns a reference to the priority field.
    fn priority(&self) -> Option<RequestPriority>;
    /// Returns a reference to the doNotPerform field.
    fn do_not_perform(&self) -> Option<BooleanType>;
    /// Returns a reference to the location field.
    fn location(&self) -> Option<Reference>;
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[ActivityDefinitionParticipant];
    /// Returns a reference to the quantity field.
    fn quantity(&self) -> Option<Quantity>;
    /// Returns a reference to the dosage field.
    fn dosage(&self) -> &[Dosage];
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> &[CodeableConcept];
    /// Returns a reference to the specimenRequirement field.
    fn specimen_requirement(&self) -> &[Reference];
    /// Returns a reference to the observationRequirement field.
    fn observation_requirement(&self) -> &[Reference];
    /// Returns a reference to the observationResultRequirement field.
    fn observation_result_requirement(&self) -> &[Reference];
    /// Returns a reference to the transform field.
    fn transform(&self) -> Option<StringType>;
    /// Returns a reference to the dynamicValue field.
    fn dynamic_value(&self) -> &[ActivityDefinitionDynamicvalue];
}
/// ActivityDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActivityDefinitionMutators: DomainResourceMutators {
    /// Create a new ActivityDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::activity_definition::ActivityDefinition;
    /// use hl7_fhir_r4_core::traits::activity_definition::ActivityDefinitionMutators;
    ///
    /// let resource = ActivityDefinition::new();
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
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the usage field and returns self for chaining.
    fn set_usage(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the topic field and returns self for chaining.
    fn set_topic(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the topic field and returns self for chaining.
    fn add_topic(self, item: CodeableConcept) -> Self;
    /// Sets the author field and returns self for chaining.
    fn set_author(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the author field and returns self for chaining.
    fn add_author(self, item: ContactDetail) -> Self;
    /// Sets the editor field and returns self for chaining.
    fn set_editor(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the editor field and returns self for chaining.
    fn add_editor(self, item: ContactDetail) -> Self;
    /// Sets the reviewer field and returns self for chaining.
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the reviewer field and returns self for chaining.
    fn add_reviewer(self, item: ContactDetail) -> Self;
    /// Sets the endorser field and returns self for chaining.
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the endorser field and returns self for chaining.
    fn add_endorser(self, item: ContactDetail) -> Self;
    /// Sets the relatedArtifact field and returns self for chaining.
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self;
    /// Adds an item to the relatedArtifact field and returns self for chaining.
    fn add_related_artifact(self, item: RelatedArtifact) -> Self;
    /// Sets the library field and returns self for chaining.
    fn set_library(self, value: Vec<String>) -> Self;
    /// Adds an item to the library field and returns self for chaining.
    fn add_library(self, item: String) -> Self;
    /// Sets the kind field and returns self for chaining.
    fn set_kind(self, value: RequestResourceTypes) -> Self;
    /// Sets the profile field and returns self for chaining.
    fn set_profile(self, value: String) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the intent field and returns self for chaining.
    fn set_intent(self, value: RequestIntent) -> Self;
    /// Sets the priority field and returns self for chaining.
    fn set_priority(self, value: RequestPriority) -> Self;
    /// Sets the doNotPerform field and returns self for chaining.
    fn set_do_not_perform(self, value: bool) -> Self;
    /// Sets the location field and returns self for chaining.
    fn set_location(self, value: Reference) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<ActivityDefinitionParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: ActivityDefinitionParticipant) -> Self;
    /// Sets the quantity field and returns self for chaining.
    fn set_quantity(self, value: Quantity) -> Self;
    /// Sets the dosage field and returns self for chaining.
    fn set_dosage(self, value: Vec<Dosage>) -> Self;
    /// Adds an item to the dosage field and returns self for chaining.
    fn add_dosage(self, item: Dosage) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the bodySite field and returns self for chaining.
    fn add_body_site(self, item: CodeableConcept) -> Self;
    /// Sets the specimenRequirement field and returns self for chaining.
    fn set_specimen_requirement(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the specimenRequirement field and returns self for chaining.
    fn add_specimen_requirement(self, item: Reference) -> Self;
    /// Sets the observationRequirement field and returns self for chaining.
    fn set_observation_requirement(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the observationRequirement field and returns self for chaining.
    fn add_observation_requirement(self, item: Reference) -> Self;
    /// Sets the observationResultRequirement field and returns self for chaining.
    fn set_observation_result_requirement(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the observationResultRequirement field and returns self for chaining.
    fn add_observation_result_requirement(self, item: Reference) -> Self;
    /// Sets the transform field and returns self for chaining.
    fn set_transform(self, value: String) -> Self;
    /// Sets the dynamicValue field and returns self for chaining.
    fn set_dynamic_value(self, value: Vec<ActivityDefinitionDynamicvalue>) -> Self;
    /// Adds an item to the dynamicValue field and returns self for chaining.
    fn add_dynamic_value(self, item: ActivityDefinitionDynamicvalue) -> Self;
}
/// ActivityDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ActivityDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the subtitle field is present (Some).
    fn has_subtitle(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the subject field is present (Some).
    fn has_subject(&self) -> bool;
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
    /// Returns true if the usage field is present (Some).
    fn has_usage(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the topic field is not empty.
    fn has_topic(&self) -> bool;
    /// Returns true if the author field is not empty.
    fn has_author(&self) -> bool;
    /// Returns true if the editor field is not empty.
    fn has_editor(&self) -> bool;
    /// Returns true if the reviewer field is not empty.
    fn has_reviewer(&self) -> bool;
    /// Returns true if the endorser field is not empty.
    fn has_endorser(&self) -> bool;
    /// Returns true if the related_artifact field is not empty.
    fn has_related_artifact(&self) -> bool;
    /// Returns true if the library field is not empty.
    fn has_library(&self) -> bool;
    /// Returns true if the kind field is present (Some).
    fn has_kind(&self) -> bool;
    /// Returns true if the profile field is present (Some).
    fn has_profile(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the intent field is present (Some).
    fn has_intent(&self) -> bool;
    /// Returns true if the priority field is present (Some).
    fn has_priority(&self) -> bool;
    /// Returns true if the do_not_perform field is present (Some).
    fn has_do_not_perform(&self) -> bool;
    /// Returns true if the timing field is present (Some).
    fn has_timing(&self) -> bool;
    /// Returns true if the location field is present (Some).
    fn has_location(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the product field is present (Some).
    fn has_product(&self) -> bool;
    /// Returns true if the quantity field is present (Some).
    fn has_quantity(&self) -> bool;
    /// Returns true if the dosage field is not empty.
    fn has_dosage(&self) -> bool;
    /// Returns true if the body_site field is not empty.
    fn has_body_site(&self) -> bool;
    /// Returns true if the specimen_requirement field is not empty.
    fn has_specimen_requirement(&self) -> bool;
    /// Returns true if the observation_requirement field is not empty.
    fn has_observation_requirement(&self) -> bool;
    /// Returns true if the observation_result_requirement field is not empty.
    fn has_observation_result_requirement(&self) -> bool;
    /// Returns true if the transform field is present (Some).
    fn has_transform(&self) -> bool;
    /// Returns true if the dynamic_value field is not empty.
    fn has_dynamic_value(&self) -> bool;
}
