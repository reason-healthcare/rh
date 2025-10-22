use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Author
///
/// Allows capturing, on a specific question or group of questions, exactly who was responsible for providing the answer(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaireresponse-author
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireresponseAuthor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireresponseAuthor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
