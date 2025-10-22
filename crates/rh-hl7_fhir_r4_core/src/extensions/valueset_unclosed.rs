use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// unclosed
///
/// Marks that the expansion is  incomplete, and values other than those listed may be valid. This may be used because post-coordinated codes are allowed, and no practical expansion can be produced.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-unclosed
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetUnclosed {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetUnclosed {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
