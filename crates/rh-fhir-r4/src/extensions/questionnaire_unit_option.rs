use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// unitOption
///
/// A unit that the user may choose when providing a quantity value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unitOption
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnitOption {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireUnitOption {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
