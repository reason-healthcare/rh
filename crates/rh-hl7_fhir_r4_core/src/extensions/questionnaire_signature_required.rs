use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// signatureRequired
///
/// Indicates that a signature (of the specified type) is needed when completing the QuestionnaireResponse.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-signatureRequired
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSignatureRequired {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireSignatureRequired {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
