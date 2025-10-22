use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::test_script::TestScriptDestination;
use crate::resources::test_script::TestScriptFixture;
use crate::resources::test_script::TestScriptMetadata;
use crate::resources::test_script::TestScriptOrigin;
use crate::resources::test_script::TestScriptSetup;
use crate::resources::test_script::TestScriptTeardown;
use crate::resources::test_script::TestScriptTest;
use crate::resources::test_script::TestScriptVariable;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// TestScript Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestScript
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestScriptAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> StringType;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
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
    /// Returns a reference to the origin field.
    fn origin(&self) -> &[TestScriptOrigin];
    /// Returns a reference to the destination field.
    fn destination(&self) -> &[TestScriptDestination];
    /// Returns a reference to the metadata field.
    fn metadata(&self) -> Option<TestScriptMetadata>;
    /// Returns a reference to the fixture field.
    fn fixture(&self) -> &[TestScriptFixture];
    /// Returns a reference to the profile field.
    fn profile(&self) -> &[Reference];
    /// Returns a reference to the variable field.
    fn variable(&self) -> &[TestScriptVariable];
    /// Returns a reference to the setup field.
    fn setup(&self) -> Option<TestScriptSetup>;
    /// Returns a reference to the test field.
    fn test(&self) -> &[TestScriptTest];
    /// Returns a reference to the teardown field.
    fn teardown(&self) -> Option<TestScriptTeardown>;
}
/// TestScript Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestScript
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestScriptMutators: DomainResourceMutators {
    /// Create a new TestScript with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::test_script::TestScript;
    /// use hl7_fhir_r4_core::traits::test_script::TestScriptMutators;
    ///
    /// let resource = TestScript::new();
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
    /// Sets the origin field and returns self for chaining.
    fn set_origin(self, value: Vec<TestScriptOrigin>) -> Self;
    /// Adds an item to the origin field and returns self for chaining.
    fn add_origin(self, item: TestScriptOrigin) -> Self;
    /// Sets the destination field and returns self for chaining.
    fn set_destination(self, value: Vec<TestScriptDestination>) -> Self;
    /// Adds an item to the destination field and returns self for chaining.
    fn add_destination(self, item: TestScriptDestination) -> Self;
    /// Sets the metadata field and returns self for chaining.
    fn set_metadata(self, value: TestScriptMetadata) -> Self;
    /// Sets the fixture field and returns self for chaining.
    fn set_fixture(self, value: Vec<TestScriptFixture>) -> Self;
    /// Adds an item to the fixture field and returns self for chaining.
    fn add_fixture(self, item: TestScriptFixture) -> Self;
    /// Sets the profile field and returns self for chaining.
    fn set_profile(self, value: Vec<Reference>) -> Self;
    /// Adds an item to the profile field and returns self for chaining.
    fn add_profile(self, item: Reference) -> Self;
    /// Sets the variable field and returns self for chaining.
    fn set_variable(self, value: Vec<TestScriptVariable>) -> Self;
    /// Adds an item to the variable field and returns self for chaining.
    fn add_variable(self, item: TestScriptVariable) -> Self;
    /// Sets the setup field and returns self for chaining.
    fn set_setup(self, value: TestScriptSetup) -> Self;
    /// Sets the test field and returns self for chaining.
    fn set_test(self, value: Vec<TestScriptTest>) -> Self;
    /// Adds an item to the test field and returns self for chaining.
    fn add_test(self, item: TestScriptTest) -> Self;
    /// Sets the teardown field and returns self for chaining.
    fn set_teardown(self, value: TestScriptTeardown) -> Self;
}
/// TestScript Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestScript
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestScriptExistence: DomainResourceExistence {
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
    /// Returns true if the origin field is not empty.
    fn has_origin(&self) -> bool;
    /// Returns true if the destination field is not empty.
    fn has_destination(&self) -> bool;
    /// Returns true if the metadata field is present (Some).
    fn has_metadata(&self) -> bool;
    /// Returns true if the fixture field is not empty.
    fn has_fixture(&self) -> bool;
    /// Returns true if the profile field is not empty.
    fn has_profile(&self) -> bool;
    /// Returns true if the variable field is not empty.
    fn has_variable(&self) -> bool;
    /// Returns true if the setup field is present (Some).
    fn has_setup(&self) -> bool;
    /// Returns true if the test field is not empty.
    fn has_test(&self) -> bool;
    /// Returns true if the teardown field is present (Some).
    fn has_teardown(&self) -> bool;
}
