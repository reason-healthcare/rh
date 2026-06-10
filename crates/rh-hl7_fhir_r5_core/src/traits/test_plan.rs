use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::test_plan::TestPlanDependency;
use crate::resources::test_plan::TestPlanTestcase;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// TestPlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A plan for executing testing on an artifact or specifications
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestPlan
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestPlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestPlanAccessors: DomainResourceAccessors {
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
    /// Returns a reference to the category field.
    fn category(&self) -> &[CodeableConcept];
    /// Returns a reference to the scope field.
    fn scope(&self) -> &[Reference];
    /// Returns a reference to the testTools field.
    fn test_tools(&self) -> Option<StringType>;
    /// Returns a reference to the dependency field.
    fn dependency(&self) -> &[TestPlanDependency];
    /// Returns a reference to the exitCriteria field.
    fn exit_criteria(&self) -> Option<StringType>;
    /// Returns a reference to the testCase field.
    fn test_case(&self) -> &[TestPlanTestcase];
}
/// TestPlan Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A plan for executing testing on an artifact or specifications
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestPlan
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestPlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestPlanMutators: DomainResourceMutators {
    /// Create a new TestPlan with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::test_plan::TestPlan;
    /// use rh_hl7_fhir_r5_core::traits::test_plan::TestPlanMutators;
    ///
    /// let resource = TestPlan::new();
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
    /// Sets the category field and returns self for chaining.
    fn set_category(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the category field and returns self for chaining.
    fn add_category(self, item: CodeableConcept) -> Self;
    /// Sets the scope field and returns self for chaining.
    fn set_scope(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the scope field and returns self for chaining.
    fn add_scope(self, item: Reference) -> Self;
    /// Sets the testTools field and returns self for chaining.
    fn set_test_tools(self, value: String) -> Self;
    /// Sets the dependency field and returns self for chaining.
    fn set_dependency(self, value: Vec<TestPlanDependency>) -> Self;
    /// Adds an item to the dependency field and returns self for chaining.
    fn add_dependency(self, item: TestPlanDependency) -> Self;
    /// Sets the exitCriteria field and returns self for chaining.
    fn set_exit_criteria(self, value: String) -> Self;
    /// Sets the testCase field and returns self for chaining.
    fn set_test_case(self, value: Vec<TestPlanTestcase>) -> Self;
    /// Adds an item to the testCase field and returns self for chaining.
    fn add_test_case(self, item: TestPlanTestcase) -> Self;
}
/// TestPlan Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A plan for executing testing on an artifact or specifications
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestPlan
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestPlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestPlanExistence: DomainResourceExistence {
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
    /// Returns true if the category field is not empty.
    fn has_category(&self) -> bool;
    /// Returns true if the scope field is not empty.
    fn has_scope(&self) -> bool;
    /// Returns true if the test_tools field is present (Some).
    fn has_test_tools(&self) -> bool;
    /// Returns true if the dependency field is not empty.
    fn has_dependency(&self) -> bool;
    /// Returns true if the exit_criteria field is present (Some).
    fn has_exit_criteria(&self) -> bool;
    /// Returns true if the test_case field is not empty.
    fn has_test_case(&self) -> bool;
}
