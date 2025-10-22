use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-streetAddressLine
///
/// A street address line is frequently used instead of breaking out building number, street name, street type, etc. An address generally has only a delivery address line or a street address line, but not both.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-streetAddressLine
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPStreetAddressLine {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPStreetAddressLine {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
