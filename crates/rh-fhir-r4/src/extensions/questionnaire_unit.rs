use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// unit
///
/// Provides a computable unit of measure associated with numeric questions to support subsequent computation on responses. This is for use on items of type integer and decimal, and it's purpose is to support converting the integer or decimal answer into a Quantity when extracting the data into a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unit
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnit {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireUnit {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
