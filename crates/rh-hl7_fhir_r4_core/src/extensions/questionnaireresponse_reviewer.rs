use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Reviewer
///
/// Individual responsible for ensuring that the questionnaire response have been completed appropriately and signs off on the content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaireresponse-reviewer
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireresponseReviewer {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireresponseReviewer {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
