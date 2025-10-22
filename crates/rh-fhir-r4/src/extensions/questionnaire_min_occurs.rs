use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// minOccurs
///
/// The minimum number of times the group must appear, or the minimum number of answers for a question - when greater than 1.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-minOccurs
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireMinOccurs {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireMinOccurs {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
