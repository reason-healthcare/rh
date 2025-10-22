use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// referenceProfile
///
/// Where the type for a question is "Reference", indicates a profile that the resource instances pointed to in answers to this question must be valid against.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceProfile
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceProfile {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireReferenceProfile {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
