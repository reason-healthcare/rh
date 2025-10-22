use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// related
///
/// This condition has an unspecified relationship with another condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/condition-related
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRelated {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConditionRelated {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
