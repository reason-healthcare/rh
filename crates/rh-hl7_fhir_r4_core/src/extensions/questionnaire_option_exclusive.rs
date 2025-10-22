use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// optionExclusive
///
/// If true, indicates that if this answerOption is selected, no other possible answers may be selected, even if the item is a repeating question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-optionExclusive
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireOptionExclusive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireOptionExclusive {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
