use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// map
///
/// A reference to a concept map that is relevant for the interpretation of this value set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-map
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetMap {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetMap {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
