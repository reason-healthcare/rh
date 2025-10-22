use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Conformance prohibition
///
/// If set to true, indicates that support for the specified behavior would make a system non-conformant with the specification.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/capabilitystatement-prohibited
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitystatementProhibited {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CapabilitystatementProhibited {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
