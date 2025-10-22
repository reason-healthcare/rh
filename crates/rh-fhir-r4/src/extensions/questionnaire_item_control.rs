use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// itemControl
///
/// The type of data entry control or structure that should be used to render the item.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-itemControl
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireItemControl {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireItemControl {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
