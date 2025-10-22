use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Status of Implantable Devices
///
/// Codes to represent the functional status of a device implanted in a patient.  Both overall device status and an implant status need to be considered. The implant status should only be used when the [device status](device-definitions.html#Device.status) is `active `.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/device-implantStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceImplantStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DeviceImplantStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
