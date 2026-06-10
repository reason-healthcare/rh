use crate::bindings::permitted_data_type::PermittedDataType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::observation_definition::ObservationDefinitionComponent;
use crate::resources::observation_definition::ObservationDefinitionQualifiedvalue;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// ObservationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
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
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the copyrightLabel field.
    fn copyright_label(&self) -> Option<StringType>;
    /// Returns a reference to the approvalDate field.
    fn approval_date(&self) -> Option<DateType>;
    /// Returns a reference to the lastReviewDate field.
    fn last_review_date(&self) -> Option<DateType>;
    /// Returns a reference to the effectivePeriod field.
    fn effective_period(&self) -> Option<Period>;
    /// Returns a reference to the derivedFromCanonical field.
    fn derived_from_canonical(&self) -> &[StringType];
    /// Returns a reference to the derivedFromUri field.
    fn derived_from_uri(&self) -> &[StringType];
    /// Returns a reference to the subject field.
    fn subject(&self) -> &[CodeableConcept];
    /// Returns a reference to the performerType field.
    fn performer_type(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the code field.
    fn code(&self) -> CodeableConcept;
    /// Returns a reference to the permittedDataType field.
    fn permitted_data_type(&self) -> &[PermittedDataType];
    /// Returns a reference to the multipleResultsAllowed field.
    fn multiple_results_allowed(&self) -> Option<BooleanType>;
    /// Returns a reference to the bodySite field.
    fn body_site(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the method field.
    fn method(&self) -> Option<CodeableConcept>;
    /// Returns a reference to the specimen field.
    fn specimen(&self) -> &[Reference];
    /// Returns a reference to the device field.
    fn device(&self) -> &[Reference];
    /// Returns a reference to the preferredReportName field.
    fn preferred_report_name(&self) -> Option<StringType>;
    /// Returns a reference to the permittedUnit field.
    fn permitted_unit(&self) -> &[Coding];
    /// Returns a reference to the qualifiedValue field.
    fn qualified_value(&self) -> &[ObservationDefinitionQualifiedvalue];
    /// Returns a reference to the hasMember field.
    fn has_member(&self) -> &[Reference];
    /// Returns a reference to the component field.
    fn component(&self) -> &[ObservationDefinitionComponent];
}
/// ObservationDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionMutators: DomainResourceMutators {
    /// Create a new ObservationDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::observation_definition::ObservationDefinition;
    /// use rh_hl7_fhir_r5_core::traits::observation_definition::ObservationDefinitionMutators;
    ///
    /// let resource = ObservationDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
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
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the copyrightLabel field and returns self for chaining.
    fn set_copyright_label(self, value: String) -> Self;
    /// Sets the approvalDate field and returns self for chaining.
    fn set_approval_date(self, value: String) -> Self;
    /// Sets the lastReviewDate field and returns self for chaining.
    fn set_last_review_date(self, value: String) -> Self;
    /// Sets the effectivePeriod field and returns self for chaining.
    fn set_effective_period(self, value: Period) -> Self;
    /// Sets the derivedFromCanonical field and returns self for chaining.
    fn set_derived_from_canonical(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFromCanonical field and returns self for chaining.
    fn add_derived_from_canonical(self, item: String) -> Self;
    /// Sets the derivedFromUri field and returns self for chaining.
    fn set_derived_from_uri(self, value: Vec<String>) -> Self;
    /// Adds an item to the derivedFromUri field and returns self for chaining.
    fn add_derived_from_uri(self, item: String) -> Self;
    /// Sets the subject field and returns self for chaining.
    fn set_subject(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the subject field and returns self for chaining.
    fn add_subject(self, item: CodeableConcept) -> Self;
    /// Sets the performerType field and returns self for chaining.
    fn set_performer_type(self, value: CodeableConcept) -> Self;
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the code field and returns self for chaining.
    fn set_code(self, value: CodeableConcept) -> Self;
    /// Sets the permittedDataType field and returns self for chaining.
    fn set_permitted_data_type(self, value: Vec<PermittedDataType>) -> Self;
    /// Adds an item to the permittedDataType field and returns self for chaining.
    fn add_permitted_data_type(self, item: PermittedDataType) -> Self;
    /// Sets the multipleResultsAllowed field and returns self for chaining.
    fn set_multiple_results_allowed(self, value: bool) -> Self;
    /// Sets the bodySite field and returns self for chaining.
    fn set_body_site(self, value: CodeableConcept) -> Self;
    /// Sets the method field and returns self for chaining.
    fn set_method(self, value: CodeableConcept) -> Self;
    /// Sets the specimen field and returns self for chaining.
    fn set_specimen(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the specimen field and returns self for chaining.
    fn add_specimen(self, item: Reference) -> Self;
    /// Sets the device field and returns self for chaining.
    fn set_device(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the device field and returns self for chaining.
    fn add_device(self, item: Reference) -> Self;
    /// Sets the preferredReportName field and returns self for chaining.
    fn set_preferred_report_name(self, value: String) -> Self;
    /// Sets the permittedUnit field and returns self for chaining.
    fn set_permitted_unit(self, value: Vec<Coding>) -> Self;
    /// Adds an item to the permittedUnit field and returns self for chaining.
    fn add_permitted_unit(self, item: Coding) -> Self;
    /// Sets the qualifiedValue field and returns self for chaining.
    fn set_qualified_value(self, value: Vec<ObservationDefinitionQualifiedvalue>) -> Self;
    /// Adds an item to the qualifiedValue field and returns self for chaining.
    fn add_qualified_value(self, item: ObservationDefinitionQualifiedvalue) -> Self;
    /// Sets the hasMember field and returns self for chaining.
    fn set_has_member(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the hasMember field and returns self for chaining.
    fn add_has_member(self, item: Reference) -> Self;
    /// Sets the component field and returns self for chaining.
    fn set_component(self, value: Vec<ObservationDefinitionComponent>) -> Self;
    /// Adds an item to the component field and returns self for chaining.
    fn add_component(self, item: ObservationDefinitionComponent) -> Self;
}
/// ObservationDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ObservationDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ObservationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait ObservationDefinitionExistence: DomainResourceExistence {
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
    /// Returns true if the identifier field is present (Some).
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
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
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the copyright_label field is present (Some).
    fn has_copyright_label(&self) -> bool;
    /// Returns true if the approval_date field is present (Some).
    fn has_approval_date(&self) -> bool;
    /// Returns true if the last_review_date field is present (Some).
    fn has_last_review_date(&self) -> bool;
    /// Returns true if the effective_period field is present (Some).
    fn has_effective_period(&self) -> bool;
    /// Returns true if the derived_from_canonical field is not empty.
    fn has_derived_from_canonical(&self) -> bool;
    /// Returns true if the derived_from_uri field is not empty.
    fn has_derived_from_uri(&self) -> bool;
    /// Returns true if the subject field is not empty.
    fn has_subject(&self) -> bool;
    /// Returns true if the performer_type field is present (Some).
    fn has_performer_type(&self) -> bool;
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the code field is present (Some).
    fn has_code(&self) -> bool;
    /// Returns true if the permitted_data_type field is not empty.
    fn has_permitted_data_type(&self) -> bool;
    /// Returns true if the multiple_results_allowed field is present (Some).
    fn has_multiple_results_allowed(&self) -> bool;
    /// Returns true if the body_site field is present (Some).
    fn has_body_site(&self) -> bool;
    /// Returns true if the method field is present (Some).
    fn has_method(&self) -> bool;
    /// Returns true if the specimen field is not empty.
    fn has_specimen(&self) -> bool;
    /// Returns true if the device field is not empty.
    fn has_device(&self) -> bool;
    /// Returns true if the preferred_report_name field is present (Some).
    fn has_preferred_report_name(&self) -> bool;
    /// Returns true if the permitted_unit field is not empty.
    fn has_permitted_unit(&self) -> bool;
    /// Returns true if the qualified_value field is not empty.
    fn has_qualified_value(&self) -> bool;
    /// Returns true if the has_member field is not empty.
    fn has_has_member(&self) -> bool;
    /// Returns true if the component field is not empty.
    fn has_component(&self) -> bool;
}
