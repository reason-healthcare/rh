use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// special-status
///
/// A special note for implementers about the status of the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-special-status
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSpecialStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSpecialStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
