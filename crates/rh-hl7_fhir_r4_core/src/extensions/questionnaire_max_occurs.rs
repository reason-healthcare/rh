use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// maxOccurs
///
/// The maximum number of times the group must appear, or the maximum number of answers for a question - when greater than 1 and not unlimited.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-maxOccurs
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireMaxOccurs {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireMaxOccurs {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
