use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// supportLink
///
/// A URL that resolves to additional supporting information or guidance related to the question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-supportLink
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSupportLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireSupportLink {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
