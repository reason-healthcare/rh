use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// referenceResource
///
/// Where the type for a question is "Reference", indicates a type of resource that is permitted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-referenceResource
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireReferenceResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireReferenceResource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
