use crate::bindings::assert_direction_codes::AssertDirectionCodes;
use crate::bindings::assert_operator_codes::AssertOperatorCodes;
use crate::bindings::assert_response_code_types::AssertResponseCodeTypes;
use crate::bindings::defined_types::DefinedTypes;
use crate::bindings::http_operations::HttpOperations;
use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// TestScript
///
/// A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TestScript
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScript {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this test script, represented as a URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the test script
    pub identifier: Option<Identifier>,
    /// Business version of the test script
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this test script (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this test script (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the test script
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for test script (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this test script is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// An abstract server representing a client or sender in a message exchange
    pub origin: Option<Vec<TestScriptOrigin>>,
    /// An abstract server representing a destination or receiver in a message exchange
    pub destination: Option<Vec<TestScriptDestination>>,
    /// Required capability that is assumed to function correctly on the FHIR server being tested
    pub metadata: Option<TestScriptMetadata>,
    /// Fixture in the test script - by reference (uri)
    pub fixture: Option<Vec<TestScriptFixture>>,
    /// Reference of the validation profile
    pub profile: Option<Vec<Reference>>,
    /// Placeholder for evaluated elements
    pub variable: Option<Vec<TestScriptVariable>>,
    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,
    /// A test in this script
    pub test: Option<Vec<TestScriptTest>>,
    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
}
/// TestScriptTest nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptTestAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The setup operation to perform
    pub operation: Option<StringType>,
    /// The setup assertion to perform
    pub assert: Option<StringType>,
}
/// TestScript nested structure for the 'variable' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptVariable {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Descriptive name for this variable
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Default, hard-coded, or user-defined value for this variable
    #[serde(rename = "defaultValue")]
    pub default_value: Option<StringType>,
    /// Extension element for the 'defaultValue' primitive field. Contains metadata and extensions.
    #[serde(rename = "_defaultValue")]
    pub _default_value: Option<Element>,
    /// Natural language description of the variable
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The FHIRPath expression against the fixture body
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// HTTP header field name for source
    #[serde(rename = "headerField")]
    pub header_field: Option<StringType>,
    /// Extension element for the 'headerField' primitive field. Contains metadata and extensions.
    #[serde(rename = "_headerField")]
    pub _header_field: Option<Element>,
    /// Hint help text for default value to enter
    pub hint: Option<StringType>,
    /// Extension element for the 'hint' primitive field. Contains metadata and extensions.
    pub _hint: Option<Element>,
    /// XPath or JSONPath against the fixture body
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// Fixture Id of source expression or headerField within this variable
    #[serde(rename = "sourceId")]
    pub source_id: Option<StringType>,
    /// Extension element for the 'sourceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sourceId")]
    pub _source_id: Option<Element>,
}
/// TestScript nested structure for the 'origin' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptOrigin {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The index of the abstract origin server starting at 1
    pub index: IntegerType,
    /// Extension element for the 'index' primitive field. Contains metadata and extensions.
    pub _index: Option<Element>,
    /// FHIR-Client | FHIR-SDC-FormFiller
    ///
    /// Binding: extensible (The type of origin profile the test system supports.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-profile-origin-types
    pub profile: Coding,
}
/// TestScript nested structure for the 'destination' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptDestination {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The index of the abstract destination server starting at 1
    pub index: IntegerType,
    /// Extension element for the 'index' primitive field. Contains metadata and extensions.
    pub _index: Option<Element>,
    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver | FHIR-SDC-FormProcessor
    ///
    /// Binding: extensible (The type of destination profile the test system supports.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-profile-destination-types
    pub profile: Coding,
}
/// TestScript nested structure for the 'fixture' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptFixture {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether or not to implicitly create the fixture during setup
    pub autocreate: BooleanType,
    /// Extension element for the 'autocreate' primitive field. Contains metadata and extensions.
    pub _autocreate: Option<Element>,
    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: BooleanType,
    /// Extension element for the 'autodelete' primitive field. Contains metadata and extensions.
    pub _autodelete: Option<Element>,
    /// Reference of the resource
    pub resource: Option<Reference>,
}
/// TestScript nested structure for the 'teardown' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptTeardown {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// One or more teardown operations to perform
    pub action: Vec<TestScriptTeardownAction>,
}
/// TestScriptMetadata nested structure for the 'capability' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptMetadataCapability {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Are the capabilities required?
    pub required: BooleanType,
    /// Extension element for the 'required' primitive field. Contains metadata and extensions.
    pub _required: Option<Element>,
    /// Are the capabilities validated?
    pub validated: BooleanType,
    /// Extension element for the 'validated' primitive field. Contains metadata and extensions.
    pub _validated: Option<Element>,
    /// The expected capabilities of the server
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Which origin server these requirements apply to
    pub origin: Option<Vec<IntegerType>>,
    /// Extension element for the 'origin' primitive field. Contains metadata and extensions.
    pub _origin: Option<Element>,
    /// Which server these requirements apply to
    pub destination: Option<IntegerType>,
    /// Extension element for the 'destination' primitive field. Contains metadata and extensions.
    pub _destination: Option<Element>,
    /// Links to the FHIR specification
    pub link: Option<Vec<StringType>>,
    /// Extension element for the 'link' primitive field. Contains metadata and extensions.
    pub _link: Option<Element>,
    /// Required Capability Statement
    pub capabilities: StringType,
    /// Extension element for the 'capabilities' primitive field. Contains metadata and extensions.
    pub _capabilities: Option<Element>,
}
/// TestScriptSetupAction nested structure for the 'assert' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupActionAssert {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Tracking/logging assertion label
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Tracking/reporting assertion description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// response | request
    pub direction: Option<AssertDirectionCodes>,
    /// Extension element for the 'direction' primitive field. Contains metadata and extensions.
    pub _direction: Option<Element>,
    /// Id of the source fixture to be evaluated
    #[serde(rename = "compareToSourceId")]
    pub compare_to_source_id: Option<StringType>,
    /// Extension element for the 'compareToSourceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_compareToSourceId")]
    pub _compare_to_source_id: Option<Element>,
    /// The FHIRPath expression to evaluate against the source fixture
    #[serde(rename = "compareToSourceExpression")]
    pub compare_to_source_expression: Option<StringType>,
    /// Extension element for the 'compareToSourceExpression' primitive field. Contains metadata and extensions.
    #[serde(rename = "_compareToSourceExpression")]
    pub _compare_to_source_expression: Option<Element>,
    /// XPath or JSONPath expression to evaluate against the source fixture
    #[serde(rename = "compareToSourcePath")]
    pub compare_to_source_path: Option<StringType>,
    /// Extension element for the 'compareToSourcePath' primitive field. Contains metadata and extensions.
    #[serde(rename = "_compareToSourcePath")]
    pub _compare_to_source_path: Option<Element>,
    /// Mime type to compare against the 'Content-Type' header
    #[serde(rename = "contentType")]
    pub content_type: Option<Mimetypes>,
    /// Extension element for the 'contentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentType")]
    pub _content_type: Option<Element>,
    /// The FHIRPath expression to be evaluated
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// HTTP header field name
    #[serde(rename = "headerField")]
    pub header_field: Option<StringType>,
    /// Extension element for the 'headerField' primitive field. Contains metadata and extensions.
    #[serde(rename = "_headerField")]
    pub _header_field: Option<Element>,
    /// Fixture Id of minimum content resource
    #[serde(rename = "minimumId")]
    pub minimum_id: Option<StringType>,
    /// Extension element for the 'minimumId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_minimumId")]
    pub _minimum_id: Option<Element>,
    /// Perform validation on navigation links?
    #[serde(rename = "navigationLinks")]
    pub navigation_links: Option<BooleanType>,
    /// Extension element for the 'navigationLinks' primitive field. Contains metadata and extensions.
    #[serde(rename = "_navigationLinks")]
    pub _navigation_links: Option<Element>,
    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval
    pub operator: Option<AssertOperatorCodes>,
    /// Extension element for the 'operator' primitive field. Contains metadata and extensions.
    pub _operator: Option<Element>,
    /// XPath or JSONPath expression
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// delete | get | options | patch | post | put | head
    #[serde(rename = "requestMethod")]
    pub request_method: Option<HttpOperations>,
    /// Extension element for the 'requestMethod' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requestMethod")]
    pub _request_method: Option<Element>,
    /// Request URL comparison value
    #[serde(rename = "requestURL")]
    pub request_u_r_l: Option<StringType>,
    /// Extension element for the 'requestURL' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requestURL")]
    pub _request_u_r_l: Option<Element>,
    /// Resource type
    pub resource: Option<DefinedTypes>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// okay | created | noContent | notModified | bad | forbidden | notFound | methodNotAllowed | conflict | gone | preconditionFailed | unprocessable
    pub response: Option<AssertResponseCodeTypes>,
    /// Extension element for the 'response' primitive field. Contains metadata and extensions.
    pub _response: Option<Element>,
    /// HTTP response code to test
    #[serde(rename = "responseCode")]
    pub response_code: Option<StringType>,
    /// Extension element for the 'responseCode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_responseCode")]
    pub _response_code: Option<Element>,
    /// Fixture Id of source expression or headerField
    #[serde(rename = "sourceId")]
    pub source_id: Option<StringType>,
    /// Extension element for the 'sourceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sourceId")]
    pub _source_id: Option<Element>,
    /// Profile Id of validation profile reference
    #[serde(rename = "validateProfileId")]
    pub validate_profile_id: Option<StringType>,
    /// Extension element for the 'validateProfileId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_validateProfileId")]
    pub _validate_profile_id: Option<Element>,
    /// The value to compare to
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Will this assert produce a warning only on error?
    #[serde(rename = "warningOnly")]
    pub warning_only: BooleanType,
    /// Extension element for the 'warningOnly' primitive field. Contains metadata and extensions.
    #[serde(rename = "_warningOnly")]
    pub _warning_only: Option<Element>,
}
/// TestScriptSetupActionOperation nested structure for the 'requestHeader' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupActionOperationRequestheader {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// HTTP header field name
    pub field: StringType,
    /// Extension element for the 'field' primitive field. Contains metadata and extensions.
    pub _field: Option<Element>,
    /// HTTP headerfield value
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// TestScript nested structure for the 'setup' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A setup operation or assert to perform
    pub action: Vec<TestScriptSetupAction>,
}
/// TestScriptSetupAction nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupActionOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The operation code type that will be executed
    ///
    /// Binding: extensible (The allowable operation code types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-operation-codes
    #[serde(rename = "type")]
    pub type_: Option<Coding>,
    /// Resource type
    pub resource: Option<DefinedTypes>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// Tracking/logging operation label
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Tracking/reporting operation description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Mime type to accept in the payload of the response, with charset etc.
    pub accept: Option<Mimetypes>,
    /// Extension element for the 'accept' primitive field. Contains metadata and extensions.
    pub _accept: Option<Element>,
    /// Mime type of the request payload contents, with charset etc.
    #[serde(rename = "contentType")]
    pub content_type: Option<Mimetypes>,
    /// Extension element for the 'contentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentType")]
    pub _content_type: Option<Element>,
    /// Server responding to the request
    pub destination: Option<IntegerType>,
    /// Extension element for the 'destination' primitive field. Contains metadata and extensions.
    pub _destination: Option<Element>,
    /// Whether or not to send the request url in encoded format
    #[serde(rename = "encodeRequestUrl")]
    pub encode_request_url: BooleanType,
    /// Extension element for the 'encodeRequestUrl' primitive field. Contains metadata and extensions.
    #[serde(rename = "_encodeRequestUrl")]
    pub _encode_request_url: Option<Element>,
    /// delete | get | options | patch | post | put | head
    pub method: Option<HttpOperations>,
    /// Extension element for the 'method' primitive field. Contains metadata and extensions.
    pub _method: Option<Element>,
    /// Server initiating the request
    pub origin: Option<IntegerType>,
    /// Extension element for the 'origin' primitive field. Contains metadata and extensions.
    pub _origin: Option<Element>,
    /// Explicitly defined path parameters
    pub params: Option<StringType>,
    /// Extension element for the 'params' primitive field. Contains metadata and extensions.
    pub _params: Option<Element>,
    /// Fixture Id of mapped request
    #[serde(rename = "requestId")]
    pub request_id: Option<StringType>,
    /// Extension element for the 'requestId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requestId")]
    pub _request_id: Option<Element>,
    /// Fixture Id of mapped response
    #[serde(rename = "responseId")]
    pub response_id: Option<StringType>,
    /// Extension element for the 'responseId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_responseId")]
    pub _response_id: Option<Element>,
    /// Fixture Id of body for PUT and POST requests
    #[serde(rename = "sourceId")]
    pub source_id: Option<StringType>,
    /// Extension element for the 'sourceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sourceId")]
    pub _source_id: Option<Element>,
    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests
    #[serde(rename = "targetId")]
    pub target_id: Option<StringType>,
    /// Extension element for the 'targetId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetId")]
    pub _target_id: Option<Element>,
    /// Request URL
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
}
/// TestScriptSetup nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
}
/// TestScriptMetadata nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptMetadataLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// URL to the specification
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Short description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// TestScriptTeardown nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptTeardownAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The teardown operation to perform
    pub operation: StringType,
}
/// TestScript nested structure for the 'metadata' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptMetadata {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Links to the FHIR specification
    pub link: Option<Vec<TestScriptMetadataLink>>,
    /// Capabilities  that are assumed to function correctly on the FHIR server being tested
    pub capability: Vec<TestScriptMetadataCapability>,
}
/// TestScript nested structure for the 'test' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptTest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A test operation or assert to perform
    pub action: Vec<TestScriptTestAction>,
    /// Tracking/logging name of this test
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Tracking/reporting short description of the test
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}

impl Default for TestScript {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: StringType::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            origin: Default::default(),
            destination: Default::default(),
            metadata: Default::default(),
            fixture: Default::default(),
            profile: Default::default(),
            variable: Default::default(),
            setup: Default::default(),
            test: Default::default(),
            teardown: Default::default(),
        }
    }
}

impl Default for TestScriptTestAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            operation: Default::default(),
            assert: Default::default(),
        }
    }
}

impl Default for TestScriptVariable {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
            default_value: Default::default(),
            _default_value: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            header_field: Default::default(),
            _header_field: Default::default(),
            hint: Default::default(),
            _hint: Default::default(),
            path: Default::default(),
            _path: Default::default(),
            source_id: Default::default(),
            _source_id: Default::default(),
        }
    }
}

impl Default for TestScriptOrigin {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            index: IntegerType::default(),
            _index: Default::default(),
            profile: Coding::default(),
        }
    }
}

impl Default for TestScriptDestination {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            index: IntegerType::default(),
            _index: Default::default(),
            profile: Coding::default(),
        }
    }
}

impl Default for TestScriptFixture {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            autocreate: BooleanType::default(),
            _autocreate: Default::default(),
            autodelete: BooleanType::default(),
            _autodelete: Default::default(),
            resource: Default::default(),
        }
    }
}

impl Default for TestScriptTeardown {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
        }
    }
}

impl Default for TestScriptMetadataCapability {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            required: Default::default(),
            _required: Default::default(),
            validated: Default::default(),
            _validated: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            origin: Default::default(),
            _origin: Default::default(),
            destination: Default::default(),
            _destination: Default::default(),
            link: Default::default(),
            _link: Default::default(),
            capabilities: Default::default(),
            _capabilities: Default::default(),
        }
    }
}

impl Default for TestScriptSetupActionAssert {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            label: Default::default(),
            _label: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            direction: Default::default(),
            _direction: Default::default(),
            compare_to_source_id: Default::default(),
            _compare_to_source_id: Default::default(),
            compare_to_source_expression: Default::default(),
            _compare_to_source_expression: Default::default(),
            compare_to_source_path: Default::default(),
            _compare_to_source_path: Default::default(),
            content_type: Default::default(),
            _content_type: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            header_field: Default::default(),
            _header_field: Default::default(),
            minimum_id: Default::default(),
            _minimum_id: Default::default(),
            navigation_links: Default::default(),
            _navigation_links: Default::default(),
            operator: Default::default(),
            _operator: Default::default(),
            path: Default::default(),
            _path: Default::default(),
            request_method: Default::default(),
            _request_method: Default::default(),
            request_u_r_l: Default::default(),
            _request_u_r_l: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            response: Default::default(),
            _response: Default::default(),
            response_code: Default::default(),
            _response_code: Default::default(),
            source_id: Default::default(),
            _source_id: Default::default(),
            validate_profile_id: Default::default(),
            _validate_profile_id: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            warning_only: Default::default(),
            _warning_only: Default::default(),
        }
    }
}

impl Default for TestScriptSetupActionOperationRequestheader {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            field: Default::default(),
            _field: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for TestScriptSetup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
        }
    }
}

impl Default for TestScriptSetupActionOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            accept: Default::default(),
            _accept: Default::default(),
            content_type: Default::default(),
            _content_type: Default::default(),
            destination: Default::default(),
            _destination: Default::default(),
            encode_request_url: Default::default(),
            _encode_request_url: Default::default(),
            method: Default::default(),
            _method: Default::default(),
            origin: Default::default(),
            _origin: Default::default(),
            params: Default::default(),
            _params: Default::default(),
            request_id: Default::default(),
            _request_id: Default::default(),
            response_id: Default::default(),
            _response_id: Default::default(),
            source_id: Default::default(),
            _source_id: Default::default(),
            target_id: Default::default(),
            _target_id: Default::default(),
            url: Default::default(),
            _url: Default::default(),
        }
    }
}

impl Default for TestScriptSetupAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
        }
    }
}

impl Default for TestScriptMetadataLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            url: Default::default(),
            _url: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for TestScriptTeardownAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            operation: Default::default(),
        }
    }
}

impl Default for TestScriptMetadata {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            link: Default::default(),
            capability: Vec::new(),
        }
    }
}

impl Default for TestScriptTest {
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

// Trait implementations
impl crate::traits::resource::ResourceAccessors for TestScript {
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

impl crate::traits::resource::ResourceMutators for TestScript {
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

impl crate::traits::resource::ResourceExistence for TestScript {
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

impl crate::traits::domain_resource::DomainResourceAccessors for TestScript {
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

impl crate::traits::domain_resource::DomainResourceMutators for TestScript {
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

impl crate::traits::domain_resource::DomainResourceExistence for TestScript {
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

impl crate::traits::test_script::TestScriptAccessors for TestScript {
    fn url(&self) -> StringType {
        self.url.clone()
    }
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> StringType {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn origin(&self) -> &[TestScriptOrigin] {
        self.origin.as_deref().unwrap_or(&[])
    }
    fn destination(&self) -> &[TestScriptDestination] {
        self.destination.as_deref().unwrap_or(&[])
    }
    fn metadata(&self) -> Option<TestScriptMetadata> {
        self.metadata.clone()
    }
    fn fixture(&self) -> &[TestScriptFixture] {
        self.fixture.as_deref().unwrap_or(&[])
    }
    fn profile(&self) -> &[Reference] {
        self.profile.as_deref().unwrap_or(&[])
    }
    fn variable(&self) -> &[TestScriptVariable] {
        self.variable.as_deref().unwrap_or(&[])
    }
    fn setup(&self) -> Option<TestScriptSetup> {
        self.setup.clone()
    }
    fn test(&self) -> &[TestScriptTest] {
        self.test.as_deref().unwrap_or(&[])
    }
    fn teardown(&self) -> Option<TestScriptTeardown> {
        self.teardown.clone()
    }
}

impl crate::traits::test_script::TestScriptMutators for TestScript {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = value;
        resource
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = value;
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_origin(self, value: Vec<TestScriptOrigin>) -> Self {
        let mut resource = self.clone();
        resource.origin = Some(value);
        resource
    }
    fn add_origin(self, item: TestScriptOrigin) -> Self {
        let mut resource = self.clone();
        resource.origin.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_destination(self, value: Vec<TestScriptDestination>) -> Self {
        let mut resource = self.clone();
        resource.destination = Some(value);
        resource
    }
    fn add_destination(self, item: TestScriptDestination) -> Self {
        let mut resource = self.clone();
        resource.destination.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_metadata(self, value: TestScriptMetadata) -> Self {
        let mut resource = self.clone();
        resource.metadata = Some(value);
        resource
    }
    fn set_fixture(self, value: Vec<TestScriptFixture>) -> Self {
        let mut resource = self.clone();
        resource.fixture = Some(value);
        resource
    }
    fn add_fixture(self, item: TestScriptFixture) -> Self {
        let mut resource = self.clone();
        resource.fixture.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_profile(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.profile = Some(value);
        resource
    }
    fn add_profile(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.profile.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_variable(self, value: Vec<TestScriptVariable>) -> Self {
        let mut resource = self.clone();
        resource.variable = Some(value);
        resource
    }
    fn add_variable(self, item: TestScriptVariable) -> Self {
        let mut resource = self.clone();
        resource.variable.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_setup(self, value: TestScriptSetup) -> Self {
        let mut resource = self.clone();
        resource.setup = Some(value);
        resource
    }
    fn set_test(self, value: Vec<TestScriptTest>) -> Self {
        let mut resource = self.clone();
        resource.test = Some(value);
        resource
    }
    fn add_test(self, item: TestScriptTest) -> Self {
        let mut resource = self.clone();
        resource.test.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_teardown(self, value: TestScriptTeardown) -> Self {
        let mut resource = self.clone();
        resource.teardown = Some(value);
        resource
    }
}

impl crate::traits::test_script::TestScriptExistence for TestScript {
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
    fn has_url(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        true
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_origin(&self) -> bool {
        self.origin.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_destination(&self) -> bool {
        self.destination.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }
    fn has_fixture(&self) -> bool {
        self.fixture.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_profile(&self) -> bool {
        self.profile.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_variable(&self) -> bool {
        self.variable.as_ref().is_some_and(|v| !v.is_empty())
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
