use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// unitValueSet
///
/// A set of units that the user may choose when providing a quantity value.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-unitValueSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireUnitValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireUnitValueSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
