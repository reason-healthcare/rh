use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// capabilities
///
/// A set of codes that defines what the server is capable of.
///
/// **Source:**
/// - URL: http://fhir-registry.smarthealthit.org/StructureDefinition/capabilities
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
