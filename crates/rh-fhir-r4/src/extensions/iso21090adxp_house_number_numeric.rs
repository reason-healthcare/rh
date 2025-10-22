use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-houseNumberNumeric
///
/// The numeric portion of a building number.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-houseNumberNumeric
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPHouseNumberNumeric {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPHouseNumberNumeric {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
