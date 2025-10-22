use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// usageMode
///
/// Identifies that the specified element should only appear in certain "modes" of operation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-usageMode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUsageMode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireUsageMode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
