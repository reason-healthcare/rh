use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// displayCategory
///
/// Describes the intended purpose of the rendered text.  For example - instructions, guidance on access control, maintenance information, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-displayCategory
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireDisplayCategory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireDisplayCategory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
