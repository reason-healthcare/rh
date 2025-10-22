use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Questionnaire Requested
///
/// Reference to a specific Questionnaire Resource as an ordered item.  Allows for ordering a specific questionnaire to be completed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-questionnaireRequest
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicerequestQuestionnaireRequest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ServicerequestQuestionnaireRequest {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
