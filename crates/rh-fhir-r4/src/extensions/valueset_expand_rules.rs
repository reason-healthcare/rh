use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expand-rules
///
/// Defines how concepts are processed into the expansion when it's for UI presentation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expand-rules
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpandRules {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpandRules {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
