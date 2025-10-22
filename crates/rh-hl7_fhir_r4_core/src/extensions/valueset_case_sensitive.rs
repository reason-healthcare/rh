use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// caseSensitive
///
/// If this a case sensitive code.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-caseSensitive
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetCaseSensitive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetCaseSensitive {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
