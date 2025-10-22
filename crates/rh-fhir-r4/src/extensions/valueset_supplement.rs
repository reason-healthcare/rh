use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// supplement
///
/// This extension declares that a value set depends on a particular supplement and should not be used in its absence.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-supplement
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSupplement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSupplement {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
