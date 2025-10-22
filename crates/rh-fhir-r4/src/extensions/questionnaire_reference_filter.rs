use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// referenceFilter
///
/// Identifies a filter to apply when looking up candidate answers for the question.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceFilter
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceFilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireReferenceFilter {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
