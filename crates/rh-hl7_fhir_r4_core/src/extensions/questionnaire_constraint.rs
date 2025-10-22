use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// constraint
///
/// An invariant that must be satisfied before responses to the questionnaire can be considered "complete".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-constraint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireConstraint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireConstraint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
