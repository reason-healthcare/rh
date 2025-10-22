use crate::bindings::report_result_codes::ReportResultCodes;
use crate::bindings::report_status_codes::ReportStatusCodes;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::test_report::TestReportParticipant;
use crate::resources::test_report::TestReportSetup;
use crate::resources::test_report::TestReportTeardown;
use crate::resources::test_report::TestReportTest;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// TestReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A summary of information based on the results of executing a TestScript.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestReportAccessors: DomainResourceAccessors {
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> Option<Identifier>;
    /// Returns a reference to the name field.
    fn name(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> ReportStatusCodes;
    /// Returns a reference to the testScript field.
    fn test_script(&self) -> Reference;
    /// Returns a reference to the result field.
    fn result(&self) -> ReportResultCodes;
    /// Returns a reference to the score field.
    fn score(&self) -> Option<DecimalType>;
    /// Returns a reference to the tester field.
    fn tester(&self) -> Option<StringType>;
    /// Returns a reference to the issued field.
    fn issued(&self) -> Option<DateTimeType>;
    /// Returns a reference to the participant field.
    fn participant(&self) -> &[TestReportParticipant];
    /// Returns a reference to the setup field.
    fn setup(&self) -> Option<TestReportSetup>;
    /// Returns a reference to the test field.
    fn test(&self) -> &[TestReportTest];
    /// Returns a reference to the teardown field.
    fn teardown(&self) -> Option<TestReportTeardown>;
}
/// TestReport Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A summary of information based on the results of executing a TestScript.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestReportMutators: DomainResourceMutators {
    /// Create a new TestReport with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use hl7_fhir_r4_core::resources::test_report::TestReport;
    /// use hl7_fhir_r4_core::traits::test_report::TestReportMutators;
    ///
    /// let resource = TestReport::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Identifier) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: ReportStatusCodes) -> Self;
    /// Sets the testScript field and returns self for chaining.
    fn set_test_script(self, value: Reference) -> Self;
    /// Sets the result field and returns self for chaining.
    fn set_result(self, value: ReportResultCodes) -> Self;
    /// Sets the score field and returns self for chaining.
    fn set_score(self, value: f64) -> Self;
    /// Sets the tester field and returns self for chaining.
    fn set_tester(self, value: String) -> Self;
    /// Sets the issued field and returns self for chaining.
    fn set_issued(self, value: String) -> Self;
    /// Sets the participant field and returns self for chaining.
    fn set_participant(self, value: Vec<TestReportParticipant>) -> Self;
    /// Adds an item to the participant field and returns self for chaining.
    fn add_participant(self, item: TestReportParticipant) -> Self;
    /// Sets the setup field and returns self for chaining.
    fn set_setup(self, value: TestReportSetup) -> Self;
    /// Sets the test field and returns self for chaining.
    fn set_test(self, value: Vec<TestReportTest>) -> Self;
    /// Adds an item to the test field and returns self for chaining.
    fn add_test(self, item: TestReportTest) -> Self;
    /// Sets the teardown field and returns self for chaining.
    fn set_teardown(self, value: TestReportTeardown) -> Self;
}
/// TestReport Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A summary of information based on the results of executing a TestScript.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait TestReportExistence: DomainResourceExistence {
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
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the test_script field is present (Some).
    fn has_test_script(&self) -> bool;
    /// Returns true if the result field is present (Some).
    fn has_result(&self) -> bool;
    /// Returns true if the score field is present (Some).
    fn has_score(&self) -> bool;
    /// Returns true if the tester field is present (Some).
    fn has_tester(&self) -> bool;
    /// Returns true if the issued field is present (Some).
    fn has_issued(&self) -> bool;
    /// Returns true if the participant field is not empty.
    fn has_participant(&self) -> bool;
    /// Returns true if the setup field is present (Some).
    fn has_setup(&self) -> bool;
    /// Returns true if the test field is not empty.
    fn has_test(&self) -> bool;
    /// Returns true if the teardown field is present (Some).
    fn has_teardown(&self) -> bool;
}
