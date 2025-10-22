use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// baseType
///
/// This identifies the underlying type in a profile, when a questionnaire is generated from a profile.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-baseType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireBaseType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireBaseType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
