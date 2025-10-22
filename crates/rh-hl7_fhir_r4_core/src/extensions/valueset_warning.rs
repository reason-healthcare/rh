use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// warning
///
/// An extra warning about the correct use of the value set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-warning
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetWarning {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetWarning {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
