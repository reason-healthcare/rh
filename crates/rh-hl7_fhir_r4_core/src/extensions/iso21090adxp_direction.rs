use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-direction
///
/// Direction (e.g., N, S, W, E).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-direction
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDirection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDirection {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
