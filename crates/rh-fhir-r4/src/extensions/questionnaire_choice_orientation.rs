use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// choiceOrientation
///
/// Identifies the desired orientation when rendering a list of choices (typically radio-box or check-box lists).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-choiceOrientation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireChoiceOrientation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireChoiceOrientation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
