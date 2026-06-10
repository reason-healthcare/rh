use crate::resources::test_script::TestScript;
use serde::{Deserialize, Serialize};
/// Shareable  Test Script
///
/// Enforces the minimum information set for the test script metadata required by HL7 and other organizations that share and publish test scripts
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/shareabletestscript
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: TestScript
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/TestScript
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shareabletestscript {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: TestScript,
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
            rh_foundation::ElementCardinality::new("TestScript", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.contained", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.extension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.identifier", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.version", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.experimental", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.publisher", 1, Some(1)),
            rh_foundation::ElementCardinality::new("TestScript.contact", 0, None),
            rh_foundation::ElementCardinality::new("TestScript.description", 1, Some(1)),
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
impl crate::traits::resource::ResourceAccessors for Shareabletestscript {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for Shareabletestscript {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Shareabletestscript {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::shareabletestscript::ShareabletestscriptMutators for Shareabletestscript {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for Shareabletestscript {
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
        Some("http://hl7.org/fhir/StructureDefinition/shareabletestscript")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::shareabletestscript::{
    ShareabletestscriptAccessors, ShareabletestscriptExistence, ShareabletestscriptMutators,
};
