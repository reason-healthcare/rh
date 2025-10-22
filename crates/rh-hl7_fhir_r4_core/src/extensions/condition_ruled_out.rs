use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ruledOut
///
/// Identifies what potential diagnoses have been ruled out for this condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/condition-ruledOut
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRuledOut {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConditionRuledOut {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
