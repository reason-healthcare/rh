use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// reason
///
/// The factor(s) that caused the questionnaire to be answered.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaireresponse-reason
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireresponseReason {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireresponseReason {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
