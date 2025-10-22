use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// label
///
/// The label to list in front of a code when presenting a list of possible values in a questionnaire-like fashion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-label
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetLabel {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetLabel {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
