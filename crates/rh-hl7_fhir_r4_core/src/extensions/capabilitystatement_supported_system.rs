use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Supported Code System
///
/// A code system that is supported by the system that is not defined in a value set resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/capabilitystatement-supported-system
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitystatementSupportedSystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CapabilitystatementSupportedSystem {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
