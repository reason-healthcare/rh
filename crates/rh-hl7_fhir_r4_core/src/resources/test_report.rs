use crate::bindings::report_action_result_codes::ReportActionResultCodes;
use crate::bindings::report_participant_type::ReportParticipantType;
use crate::bindings::report_result_codes::ReportResultCodes;
use crate::bindings::report_status_codes::ReportStatusCodes;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// TestReport
///
/// A summary of information based on the results of executing a TestScript.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestReport
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestReport
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External identifier
    pub identifier: Option<Identifier>,
    /// Informal name of the executed TestScript
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// completed | in-progress | waiting | stopped | entered-in-error
    pub status: ReportStatusCodes,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reference to the  version-specific TestScript that was executed to produce this TestReport
    #[serde(rename = "testScript")]
    pub test_script: Reference,
    /// pass | fail | pending
    pub result: ReportResultCodes,
    /// Extension element for the 'result' primitive field. Contains metadata and extensions.
    pub _result: Option<Element>,
    /// The final score (percentage of tests passed) resulting from the execution of the TestScript
    pub score: Option<DecimalType>,
    /// Extension element for the 'score' primitive field. Contains metadata and extensions.
    pub _score: Option<Element>,
    /// Name of the tester producing this report (Organization or individual)
    pub tester: Option<StringType>,
    /// Extension element for the 'tester' primitive field. Contains metadata and extensions.
    pub _tester: Option<Element>,
    /// When the TestScript was executed and this TestReport was generated
    pub issued: Option<DateTimeType>,
    /// Extension element for the 'issued' primitive field. Contains metadata and extensions.
    pub _issued: Option<Element>,
    /// A participant in the test execution, either the execution engine, a client, or a server
    pub participant: Option<Vec<TestReportParticipant>>,
    /// The results of the series of required setup operations before the tests were executed
    pub setup: Option<TestReportSetup>,
    /// A test executed from the test script
    pub test: Option<Vec<TestReportTest>>,
    /// The results of running the series of required clean up steps
    pub teardown: Option<TestReportTeardown>,
}
/// TestReport nested structure for the 'setup' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportSetup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A setup operation or assert that was executed
    pub action: Vec<TestReportSetupAction>,
}
/// TestReport nested structure for the 'teardown' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportTeardown {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// One or more teardown operations performed
    pub action: Vec<TestReportTeardownAction>,
}
/// TestReportTest nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportTestAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The operation performed
    pub operation: Option<StringType>,
    /// The assertion performed
    pub assert: Option<StringType>,
}
/// TestReport nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// test-engine | client | server
    #[serde(rename = "type")]
    pub type_: ReportParticipantType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The uri of the participant. An absolute URL is preferred
    pub uri: StringType,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// The display name of the participant
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
}
/// TestReportTeardown nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportTeardownAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The teardown operation performed
    pub operation: StringType,
}
/// TestReport nested structure for the 'test' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportTest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A test operation or assert that was performed
    pub action: Vec<TestReportTestAction>,
    /// Tracking/logging name of this test
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Tracking/reporting short description of the test
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// TestReportSetupAction nested structure for the 'assert' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportSetupActionAssert {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// pass | skip | fail | warning | error
    pub result: ReportActionResultCodes,
    /// Extension element for the 'result' primitive field. Contains metadata and extensions.
    pub _result: Option<Element>,
    /// A message associated with the result
    pub message: Option<StringType>,
    /// Extension element for the 'message' primitive field. Contains metadata and extensions.
    pub _message: Option<Element>,
    /// A link to further details on the result
    pub detail: Option<StringType>,
    /// Extension element for the 'detail' primitive field. Contains metadata and extensions.
    pub _detail: Option<Element>,
}
/// TestReportSetupAction nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportSetupActionOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// pass | skip | fail | warning | error
    pub result: ReportActionResultCodes,
    /// Extension element for the 'result' primitive field. Contains metadata and extensions.
    pub _result: Option<Element>,
    /// A message associated with the result
    pub message: Option<StringType>,
    /// Extension element for the 'message' primitive field. Contains metadata and extensions.
    pub _message: Option<Element>,
    /// A link to further details on the result
    pub detail: Option<StringType>,
    /// Extension element for the 'detail' primitive field. Contains metadata and extensions.
    pub _detail: Option<Element>,
}
/// TestReportSetup nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportSetupAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
}

impl Default for TestReport {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            status: ReportStatusCodes::default(),
            _status: Default::default(),
            test_script: Reference::default(),
            result: ReportResultCodes::default(),
            _result: Default::default(),
            score: Default::default(),
            _score: Default::default(),
            tester: Default::default(),
            _tester: Default::default(),
            issued: Default::default(),
            _issued: Default::default(),
            participant: Default::default(),
            setup: Default::default(),
            test: Default::default(),
            teardown: Default::default(),
        }
    }
}

impl Default for TestReportSetup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
        }
    }
}

impl Default for TestReportTeardown {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
        }
    }
}

impl Default for TestReportTestAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            operation: Default::default(),
            assert: Default::default(),
        }
    }
}

impl Default for TestReportParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            uri: StringType::default(),
            _uri: Default::default(),
            display: Default::default(),
            _display: Default::default(),
        }
    }
}

impl Default for TestReportTeardownAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            operation: Default::default(),
        }
    }
}

impl Default for TestReportTest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for TestReportSetupActionAssert {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            result: Default::default(),
            _result: Default::default(),
            message: Default::default(),
            _message: Default::default(),
            detail: Default::default(),
            _detail: Default::default(),
        }
    }
}

impl Default for TestReportSetupActionOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            result: Default::default(),
            _result: Default::default(),
            message: Default::default(),
            _message: Default::default(),
            detail: Default::default(),
            _detail: Default::default(),
        }
    }
}

impl Default for TestReportSetupAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("inv-1", rh_foundation::Severity::Error, "Setup action SHALL contain either an operation or assert but not both.", "operation.exists() xor assert.exists()").with_xpath("(f:operation or f:assert) and not(f:operation and f:assert)"),
    rh_foundation::Invariant::new("inv-2", rh_foundation::Severity::Error, "Test action SHALL contain either an operation or assert but not both.", "operation.exists() xor assert.exists()").with_xpath("(f:operation or f:assert) and not(f:operation and f:assert)"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "TestReport.participant.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-participant-type|4.0.1",
            )
            .with_description("The type of participant."),
            rh_foundation::ElementBinding::new(
                "TestReport.result",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-result-codes|4.0.1",
            )
            .with_description("The reported execution result."),
            rh_foundation::ElementBinding::new(
                "TestReport.setup.action.assert.result",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-action-result-codes|4.0.1",
            )
            .with_description("The results of executing an action."),
            rh_foundation::ElementBinding::new(
                "TestReport.setup.action.operation.result",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-action-result-codes|4.0.1",
            )
            .with_description("The results of executing an action."),
            rh_foundation::ElementBinding::new(
                "TestReport.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/report-status-codes|4.0.1",
            )
            .with_description("The current status of the test report."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("TestReport.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.contained", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.testScript", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.result", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.score", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.tester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.issued", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.participant", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestReport.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestReport.participant.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.participant.uri", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.participant.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.setup", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.setup.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.setup.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.setup.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.setup.action", 1, None),
            rh_foundation::ElementCardinality::new("TestReport.setup.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.setup.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestReport.setup.action.operation", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.result",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.message",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.operation.detail",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestReport.setup.action.assert", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.setup.action.assert.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.assert.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.assert.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.assert.result",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.assert.message",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.setup.action.assert.detail",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestReport.test", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.test.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.test.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.test.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestReport.test.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.test.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.test.action", 1, None),
            rh_foundation::ElementCardinality::new("TestReport.test.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.test.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestReport.test.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestReport.test.action.operation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.test.action.assert", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.teardown", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.teardown.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.teardown.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestReport.teardown.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestReport.teardown.action", 1, None),
            rh_foundation::ElementCardinality::new("TestReport.teardown.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestReport.teardown.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestReport.teardown.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestReport.teardown.action.operation",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for TestReport {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for TestReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for TestReport {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for TestReport {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for TestReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for TestReport {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::test_report::TestReportAccessors for TestReport {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn status(&self) -> ReportStatusCodes {
        self.status.clone()
    }
    fn test_script(&self) -> Reference {
        self.test_script.clone()
    }
    fn result(&self) -> ReportResultCodes {
        self.result.clone()
    }
    fn score(&self) -> Option<DecimalType> {
        self.score
    }
    fn tester(&self) -> Option<StringType> {
        self.tester.clone()
    }
    fn issued(&self) -> Option<DateTimeType> {
        self.issued.clone()
    }
    fn participant(&self) -> &[TestReportParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn setup(&self) -> Option<TestReportSetup> {
        self.setup.clone()
    }
    fn test(&self) -> &[TestReportTest] {
        self.test.as_deref().unwrap_or(&[])
    }
    fn teardown(&self) -> Option<TestReportTeardown> {
        self.teardown.clone()
    }
}

impl crate::traits::test_report::TestReportMutators for TestReport {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_status(self, value: ReportStatusCodes) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_test_script(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.test_script = value;
        resource
    }
    fn set_result(self, value: ReportResultCodes) -> Self {
        let mut resource = self.clone();
        resource.result = value;
        resource
    }
    fn set_score(self, value: f64) -> Self {
        let mut resource = self.clone();
        resource.score = Some(value);
        resource
    }
    fn set_tester(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.tester = Some(value);
        resource
    }
    fn set_issued(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.issued = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<TestReportParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: TestReportParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_setup(self, value: TestReportSetup) -> Self {
        let mut resource = self.clone();
        resource.setup = Some(value);
        resource
    }
    fn set_test(self, value: Vec<TestReportTest>) -> Self {
        let mut resource = self.clone();
        resource.test = Some(value);
        resource
    }
    fn add_test(self, item: TestReportTest) -> Self {
        let mut resource = self.clone();
        resource.test.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_teardown(self, value: TestReportTeardown) -> Self {
        let mut resource = self.clone();
        resource.teardown = Some(value);
        resource
    }
}

impl crate::traits::test_report::TestReportExistence for TestReport {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_test_script(&self) -> bool {
        true
    }
    fn has_result(&self) -> bool {
        true
    }
    fn has_score(&self) -> bool {
        self.score.is_some()
    }
    fn has_tester(&self) -> bool {
        self.tester.is_some()
    }
    fn has_issued(&self) -> bool {
        self.issued.is_some()
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_setup(&self) -> bool {
        self.setup.is_some()
    }
    fn has_test(&self) -> bool {
        self.test.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_teardown(&self) -> bool {
        self.teardown.is_some()
    }
}

impl crate::validation::ValidatableResource for TestReport {
    fn resource_type(&self) -> &'static str {
        "TestReport"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/TestReport")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::test_report::{
    TestReportAccessors, TestReportExistence, TestReportMutators,
};
