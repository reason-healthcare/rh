use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Device Code
///
/// A code representing the the type of device used for this observation.  Should only be used if not implicit in the code found in `Observation.code`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-deviceCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDeviceCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationDeviceCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
