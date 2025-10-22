use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// signature
///
/// Represents a wet or electronic signature for either the form overall or for the question or item it's associated with.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaireresponse-signature
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireresponseSignature {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireresponseSignature {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
