use crate::bindings::assert_direction_codes::AssertDirectionCodes;
use crate::bindings::assert_manual_completion_codes::AssertManualCompletionCodes;
use crate::bindings::assert_operator_codes::AssertOperatorCodes;
use crate::bindings::assert_response_code_types::AssertResponseCodeTypes;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScript {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this test script, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the test script
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
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
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the test script
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for test script (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this test script is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// An abstract server representing a client or sender in a message exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<TestScriptOrigin>,
    /// An abstract server representing a destination or receiver in a message exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<TestScriptDestination>,
    /// Required capability that is assumed to function correctly on the FHIR server being tested
    pub metadata: Option<TestScriptMetadata>,
    /// Indication of the artifact(s) that are tested by this test case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<TestScriptScope>,
    /// Fixture in the test script - by reference (uri)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture: Vec<TestScriptFixture>,
    /// Reference of the validation profile
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _profile: Vec<Element>,
    /// Placeholder for evaluated elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<TestScriptVariable>,
    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,
    /// A test in this script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestScriptTest>,
    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
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
    /// The url path of the destination server
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
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
/// TestScript nested structure for the 'metadata' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptMetadata {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Capabilities  that are assumed to function correctly on the FHIR server being tested
    pub capability: Vec<TestScriptMetadataCapability>,
    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<TestScriptMetadataLink>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<IntegerType>,
    /// Extension element for the 'origin' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _origin: Vec<Element>,
    /// Which server these requirements apply to
    pub destination: Option<IntegerType>,
    /// Extension element for the 'destination' primitive field. Contains metadata and extensions.
    pub _destination: Option<Element>,
    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<StringType>,
    /// Extension element for the 'link' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _link: Vec<Element>,
    /// Required Capability Statement
    pub capabilities: StringType,
    /// Extension element for the 'capabilities' primitive field. Contains metadata and extensions.
    pub _capabilities: Option<Element>,
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
    /// The url path of the origin server
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
}
/// TestScript nested structure for the 'scope' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptScope {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specific conformance artifact being tested
    pub artifact: StringType,
    /// Extension element for the 'artifact' primitive field. Contains metadata and extensions.
    pub _artifact: Option<Element>,
    /// required | optional | strict
    ///
    /// Binding: extensible (The expectation of whether the test must pass for the system to be considered conformant with the artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-scope-conformance-codes
    pub conformance: Option<CodeableConcept>,
    /// unit | integration | production
    ///
    /// Binding: extensible (The phase of testing for this artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-scope-phase-codes
    pub phase: Option<CodeableConcept>,
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
/// TestScriptSetup nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
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
    /// fail | pass | skip | stop
    #[serde(rename = "defaultManualCompletion")]
    pub default_manual_completion: Option<AssertManualCompletionCodes>,
    /// Extension element for the 'defaultManualCompletion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_defaultManualCompletion")]
    pub _default_manual_completion: Option<Element>,
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
    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval | manualEval
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
    ///
    /// Binding: extensible (A list of all the concrete types defined in this version of the FHIR specification - Data Types and Resource Types.)
    ///
    /// Available values:
    /// - `Account`
    /// - `ActivityDefinition`
    /// - `ActorDefinition`
    /// - `Address`
    /// - `Address`
    /// - `AdministrableProductDefinition`
    /// - `AdverseEvent`
    /// - `Age`
    /// - `Age`
    /// - `AllergyIntolerance`
    /// - ... and 253 more values
    pub resource: Option<StringType>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// continue | switchingProtocols | okay | created | accepted | nonAuthoritativeInformation | noContent | resetContent | partialContent | multipleChoices | movedPermanently | found | seeOther | notModified | useProxy | temporaryRedirect | permanentRedirect | badRequest | unauthorized | paymentRequired | forbidden | notFound | methodNotAllowed | notAcceptable | proxyAuthenticationRequired | requestTimeout | conflict | gone | lengthRequired | preconditionFailed | contentTooLarge | uriTooLong | unsupportedMediaType | rangeNotSatisfiable | expectationFailed | misdirectedRequest | unprocessableContent | upgradeRequired | internalServerError | notImplemented | badGateway | serviceUnavailable | gatewayTimeout | httpVersionNotSupported
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
    /// If this assert fails, will the current test execution stop?
    #[serde(rename = "stopTestOnFail")]
    pub stop_test_on_fail: BooleanType,
    /// Extension element for the 'stopTestOnFail' primitive field. Contains metadata and extensions.
    #[serde(rename = "_stopTestOnFail")]
    pub _stop_test_on_fail: Option<Element>,
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
/// TestScriptSetupActionAssert nested structure for the 'requirement' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupActionAssertRequirement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Link or reference to the testing requirement (uri)
    #[serde(rename = "linkUri")]
    pub link_uri: Option<StringType>,
    /// Link or reference to the testing requirement (canonical)
    #[serde(rename = "linkCanonical")]
    pub link_canonical: Option<StringType>,
}
/// TestScriptSetupAction nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptSetupActionOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The operation code type that will be executed
    ///
    /// Binding: extensible (FHIR Operation Code Types)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-operation-codes
    #[serde(rename = "type")]
    pub type_: Option<Coding>,
    /// Resource type
    ///
    /// Binding: extensible (A list of all the concrete types defined in this version of the FHIR specification - Data Types and Resource Types.)
    ///
    /// Available values:
    /// - `Account`
    /// - `ActivityDefinition`
    /// - `ActorDefinition`
    /// - `Address`
    /// - `Address`
    /// - `AdministrableProductDefinition`
    /// - `AdverseEvent`
    /// - `Age`
    /// - `Age`
    /// - `AllergyIntolerance`
    /// - ... and 253 more values
    pub resource: Option<StringType>,
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
    /// Mime type to accept in the payload of the response, with charset etc
    pub accept: Option<Mimetypes>,
    /// Extension element for the 'accept' primitive field. Contains metadata and extensions.
    pub _accept: Option<Element>,
    /// Mime type of the request payload contents, with charset etc
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
/// TestScript nested structure for the 'teardown' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScriptTeardown {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// One or more teardown operations to perform
    pub action: Vec<TestScriptTeardownAction>,
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

impl Default for TestScript {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
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
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            origin: Default::default(),
            destination: Default::default(),
            metadata: Default::default(),
            scope: Default::default(),
            fixture: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            variable: Default::default(),
            setup: Default::default(),
            test: Default::default(),
            teardown: Default::default(),
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
            url: Default::default(),
            _url: Default::default(),
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

impl Default for TestScriptMetadata {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            capability: Vec::new(),
            link: Default::default(),
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

impl Default for TestScriptOrigin {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            index: IntegerType::default(),
            _index: Default::default(),
            profile: Coding::default(),
            url: Default::default(),
            _url: Default::default(),
        }
    }
}

impl Default for TestScriptScope {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            artifact: StringType::default(),
            _artifact: Default::default(),
            conformance: Default::default(),
            phase: Default::default(),
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

impl Default for TestScriptSetupAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
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
            default_manual_completion: Default::default(),
            _default_manual_completion: Default::default(),
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
            stop_test_on_fail: Default::default(),
            _stop_test_on_fail: Default::default(),
            validate_profile_id: Default::default(),
            _validate_profile_id: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            warning_only: Default::default(),
            _warning_only: Default::default(),
        }
    }
}

impl Default for TestScriptSetupActionAssertRequirement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            link_uri: Default::default(),
            link_canonical: Default::default(),
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

impl Default for TestScriptTeardown {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action: Vec::new(),
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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("tst-1", rh_foundation::Severity::Error, "Setup action SHALL contain either an operation or assert but not both.", "operation.exists() xor assert.exists()"),
    rh_foundation::Invariant::new("tst-10", rh_foundation::Severity::Error, "Setup action assert SHALL contain either compareToSourceId and compareToSourceExpression, compareToSourceId and compareToSourcePath or neither.", "compareToSourceId.empty() xor (compareToSourceExpression.exists() or compareToSourcePath.exists())"),
    rh_foundation::Invariant::new("tst-11", rh_foundation::Severity::Error, "Test action assert SHALL contain either compareToSourceId and compareToSourceExpression, compareToSourceId and compareToSourcePath or neither.", "compareToSourceId.empty() xor (compareToSourceExpression.exists() or compareToSourcePath.exists())"),
    rh_foundation::Invariant::new("tst-12", rh_foundation::Severity::Error, "Setup action assert response and responseCode SHALL be empty when direction equals request", "(response.empty() and responseCode.empty() and direction = 'request') or direction.empty() or direction = 'response'"),
    rh_foundation::Invariant::new("tst-13", rh_foundation::Severity::Error, "Test action assert response and response and responseCode SHALL be empty when direction equals request", "(response.empty() and responseCode.empty() and direction = 'request') or direction.empty() or direction = 'response'"),
    rh_foundation::Invariant::new("tst-2", rh_foundation::Severity::Error, "Test action SHALL contain either an operation or assert but not both.", "operation.exists() xor assert.exists()"),
    rh_foundation::Invariant::new("tst-3", rh_foundation::Severity::Error, "Variable can only contain one of expression, headerField or path.", "expression.empty() or headerField.empty() or path.empty()"),
    rh_foundation::Invariant::new("tst-4", rh_foundation::Severity::Error, "TestScript metadata capability SHALL contain required or validated or both.", "capability.required.exists() or capability.validated.exists()"),
    rh_foundation::Invariant::new("tst-5", rh_foundation::Severity::Error, "Only a single assertion SHALL be present within setup action assert element.", "extension.exists() or (contentType.count() + expression.count() + headerField.count() + minimumId.count() + navigationLinks.count() + path.count() + requestMethod.count() + resource.count() + responseCode.count() + response.count() + validateProfileId.count() <=1) or (((expression.count() + minimumId.count() <=2) or (expression.count() + validateProfileId.count() <=2)) and (expression.count() + path.count() <=1) and (minimumId.count() + validateProfileId.count() <=1)) or (((path.count() + minimumId.count() <=2) or (path.count() + validateProfileId.count() <=2)) and (expression.count() + path.count() <=1) and (minimumId.count() + validateProfileId.count() <=1))"),
    rh_foundation::Invariant::new("tst-6", rh_foundation::Severity::Error, "Only a single assertion SHALL be present within test action assert element.", "extension.exists() or (contentType.count() + expression.count() + headerField.count() + minimumId.count() + navigationLinks.count() + path.count() + requestMethod.count() + resource.count() + responseCode.count() + response.count() + validateProfileId.count() <=1) or (((expression.count() + minimumId.count() <=2) or (expression.count() + validateProfileId.count() <=2)) and (expression.count() + path.count() <=1) and (minimumId.count() + validateProfileId.count() <=1)) or (((path.count() + minimumId.count() <=2) or (path.count() + validateProfileId.count() <=2)) and (expression.count() + path.count() <=1) and (minimumId.count() + validateProfileId.count() <=1))"),
    rh_foundation::Invariant::new("tst-7", rh_foundation::Severity::Error, "Setup operation SHALL contain either sourceId or targetId or params or url.", "sourceId.exists() or (targetId.count() + url.count() + params.count() = 1) or (type.code in ('capabilities' |'search' | 'transaction' | 'history'))"),
    rh_foundation::Invariant::new("tst-8", rh_foundation::Severity::Error, "Test operation SHALL contain either sourceId or targetId or params or url.", "sourceId.exists() or (targetId.count() + url.count() + params.count() = 1) or (type.code in ('capabilities' | 'search' | 'transaction' | 'history'))"),
    rh_foundation::Invariant::new("tst-9", rh_foundation::Severity::Error, "Teardown operation SHALL contain either sourceId or targetId or params or url.", "sourceId.exists() or (targetId.count() + url.count() + params.count() = 1) or (type.code in ('capabilities' | 'search' | 'transaction' | 'history'))"),
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
                "TestScript.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.contentType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.defaultManualCompletion",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/assert-manual-completion-codes|5.0.0",
            )
            .with_description("The default type of manual completion to use for assertion."),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.direction",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/assert-direction-codes|5.0.0",
            )
            .with_description("The direction to use for assertions."),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.operator",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/assert-operator-codes|5.0.0",
            )
            .with_description("The type of operator to use for assertions."),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.requestMethod",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/http-operations|5.0.0",
            )
            .with_description("The allowable request method or HTTP operation codes."),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.assert.response",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/assert-response-code-types|5.0.0",
            )
            .with_description("The response code to expect in the response."),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.operation.accept",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.operation.contentType",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|5.0.0",
            )
            .with_description("BCP 13 (RFCs 2045, 2046, 2047, 4288, 4289 and 2049)"),
            rh_foundation::ElementBinding::new(
                "TestScript.setup.action.operation.method",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/http-operations|5.0.0",
            )
            .with_description("The allowable request method or HTTP operation codes."),
            rh_foundation::ElementBinding::new(
                "TestScript.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("TestScript.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.contained", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.identifier", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.contact", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.useContext", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.origin", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.origin.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.origin.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.origin.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.origin.index", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.origin.profile", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.origin.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.destination", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.destination.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.destination.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.destination.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.destination.index", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.destination.profile", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.destination.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.metadata", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.metadata.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.metadata.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.metadata.link", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.metadata.link.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.metadata.link.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.link.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.metadata.link.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.link.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestScript.metadata.capability", 1, None),
            rh_foundation::ElementCardinality::new("TestScript.metadata.capability.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.required",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.validated",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.origin",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.destination",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestScript.metadata.capability.link", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.metadata.capability.capabilities",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestScript.scope", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.scope.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.scope.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.scope.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.scope.artifact", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.scope.conformance", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.scope.phase", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.fixture", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.fixture.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.fixture.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.fixture.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.fixture.autocreate", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.fixture.autodelete", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.fixture.resource", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.profile", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.variable", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.variable.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.variable.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.variable.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.defaultValue", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.expression", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.headerField", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.hint", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.path", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.variable.sourceId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.setup", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.setup.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.setup.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.setup.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.setup.action", 1, None),
            rh_foundation::ElementCardinality::new("TestScript.setup.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.setup.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.setup.action.operation", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.label",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.accept",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.contentType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.destination",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.encodeRequestUrl",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.method",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.origin",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.params",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader.field",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestHeader.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.requestId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.responseId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.sourceId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.targetId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.operation.url",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestScript.setup.action.assert", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.setup.action.assert.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.label",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.direction",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.compareToSourceId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.compareToSourceExpression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.compareToSourcePath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.contentType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.defaultManualCompletion",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.headerField",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.minimumId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.navigationLinks",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.operator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.path",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requestMethod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requestURL",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.response",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.responseCode",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.sourceId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.stopTestOnFail",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.validateProfileId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.value",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.warningOnly",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requirement.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requirement.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requirement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.setup.action.assert.requirement.link[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("TestScript.test", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.test.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.test.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.test.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.test.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.test.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.test.action", 1, None),
            rh_foundation::ElementCardinality::new("TestScript.test.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.test.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.test.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.test.action.operation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.test.action.assert", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.teardown", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.teardown.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.teardown.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.teardown.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("TestScript.teardown.action", 1, None),
            rh_foundation::ElementCardinality::new("TestScript.teardown.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.teardown.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "TestScript.teardown.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "TestScript.teardown.action.operation",
                1,
                Some(1),
            ),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for TestScript {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::test_script::TestScriptAccessors for TestScript {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
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
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn origin(&self) -> &[TestScriptOrigin] {
        self.origin.as_slice()
    }
    fn destination(&self) -> &[TestScriptDestination] {
        self.destination.as_slice()
    }
    fn metadata(&self) -> Option<TestScriptMetadata> {
        self.metadata.clone()
    }
    fn scope(&self) -> &[TestScriptScope] {
        self.scope.as_slice()
    }
    fn fixture(&self) -> &[TestScriptFixture] {
        self.fixture.as_slice()
    }
    fn profile(&self) -> &[StringType] {
        self.profile.as_slice()
    }
    fn variable(&self) -> &[TestScriptVariable] {
        self.variable.as_slice()
    }
    fn setup(&self) -> Option<TestScriptSetup> {
        self.setup.clone()
    }
    fn test(&self) -> &[TestScriptTest] {
        self.test.as_slice()
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
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
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
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
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
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_origin(self, value: Vec<TestScriptOrigin>) -> Self {
        let mut resource = self.clone();
        resource.origin = value;
        resource
    }
    fn add_origin(self, item: TestScriptOrigin) -> Self {
        let mut resource = self.clone();
        resource.origin.push(item);
        resource
    }
    fn set_destination(self, value: Vec<TestScriptDestination>) -> Self {
        let mut resource = self.clone();
        resource.destination = value;
        resource
    }
    fn add_destination(self, item: TestScriptDestination) -> Self {
        let mut resource = self.clone();
        resource.destination.push(item);
        resource
    }
    fn set_metadata(self, value: TestScriptMetadata) -> Self {
        let mut resource = self.clone();
        resource.metadata = Some(value);
        resource
    }
    fn set_scope(self, value: Vec<TestScriptScope>) -> Self {
        let mut resource = self.clone();
        resource.scope = value;
        resource
    }
    fn add_scope(self, item: TestScriptScope) -> Self {
        let mut resource = self.clone();
        resource.scope.push(item);
        resource
    }
    fn set_fixture(self, value: Vec<TestScriptFixture>) -> Self {
        let mut resource = self.clone();
        resource.fixture = value;
        resource
    }
    fn add_fixture(self, item: TestScriptFixture) -> Self {
        let mut resource = self.clone();
        resource.fixture.push(item);
        resource
    }
    fn set_profile(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.profile = value;
        resource
    }
    fn add_profile(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.profile.push(item);
        resource
    }
    fn set_variable(self, value: Vec<TestScriptVariable>) -> Self {
        let mut resource = self.clone();
        resource.variable = value;
        resource
    }
    fn add_variable(self, item: TestScriptVariable) -> Self {
        let mut resource = self.clone();
        resource.variable.push(item);
        resource
    }
    fn set_setup(self, value: TestScriptSetup) -> Self {
        let mut resource = self.clone();
        resource.setup = Some(value);
        resource
    }
    fn set_test(self, value: Vec<TestScriptTest>) -> Self {
        let mut resource = self.clone();
        resource.test = value;
        resource
    }
    fn add_test(self, item: TestScriptTest) -> Self {
        let mut resource = self.clone();
        resource.test.push(item);
        resource
    }
    fn set_teardown(self, value: TestScriptTeardown) -> Self {
        let mut resource = self.clone();
        resource.teardown = Some(value);
        resource
    }
}

impl crate::traits::test_script::TestScriptExistence for TestScript {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
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
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_origin(&self) -> bool {
        !self.origin.is_empty()
    }
    fn has_destination(&self) -> bool {
        !self.destination.is_empty()
    }
    fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }
    fn has_scope(&self) -> bool {
        !self.scope.is_empty()
    }
    fn has_fixture(&self) -> bool {
        !self.fixture.is_empty()
    }
    fn has_profile(&self) -> bool {
        !self.profile.is_empty()
    }
    fn has_variable(&self) -> bool {
        !self.variable.is_empty()
    }
    fn has_setup(&self) -> bool {
        self.setup.is_some()
    }
    fn has_test(&self) -> bool {
        !self.test.is_empty()
    }
    fn has_teardown(&self) -> bool {
        self.teardown.is_some()
    }
}

impl crate::validation::ValidatableResource for TestScript {
    fn resource_type(&self) -> &'static str {
        "TestScript"
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
        Some("http://hl7.org/fhir/StructureDefinition/TestScript")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::test_script::{
    TestScriptAccessors, TestScriptExistence, TestScriptMutators,
};
