use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Gateway Device
///
/// The Provenance/AuditEvent resources can represent the same information.  Note that the Provenance/AuditEvent resources can represent the same information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-gatewayDevice
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGatewayDevice {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGatewayDevice {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
