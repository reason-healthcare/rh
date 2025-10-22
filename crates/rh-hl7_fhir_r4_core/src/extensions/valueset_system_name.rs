use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// systemName
///
/// The human-readable name for the code system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-systemName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSystemName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSystemName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
