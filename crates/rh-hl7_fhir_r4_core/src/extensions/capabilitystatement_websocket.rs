use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// WebSocket
///
/// Where the server provides its web socket end-point.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/capabilitystatement-websocket
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitystatementWebsocket {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CapabilitystatementWebsocket {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
