use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// extensible
///
/// Whether this is intended to be used with an extensible binding or not.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-extensible
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExtensible {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExtensible {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
