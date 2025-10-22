use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-deliveryAddressLine
///
/// A delivery address line is frequently used instead of breaking out delivery mode, delivery installation, etc. An address generally has only a delivery address line or a street address line, but not both.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-deliveryAddressLine
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDeliveryAddressLine {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDeliveryAddressLine {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
