use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-unitID
///
/// The number or name of a specific unit contained within a building or complex, as assigned by that building or complex.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-unitID
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPUnitID {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPUnitID {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
