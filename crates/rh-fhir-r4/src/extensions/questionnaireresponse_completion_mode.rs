use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// completionMode
///
/// Indicates how the individual completing the QuestionnaireResponse provided their responses.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaireresponse-completionMode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireresponseCompletionMode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireresponseCompletionMode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
