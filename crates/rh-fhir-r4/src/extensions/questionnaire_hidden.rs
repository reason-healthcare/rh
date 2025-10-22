use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// hidden
///
/// If true, indicates that the extended item should not be displayed to the user.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-hidden
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireHidden {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireHidden {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
