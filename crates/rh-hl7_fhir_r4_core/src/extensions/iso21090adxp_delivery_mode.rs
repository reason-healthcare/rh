use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-deliveryMode
///
/// Indicates the type of service offered, method of delivery. For example: post office box, rural route, general delivery, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-deliveryMode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDeliveryMode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDeliveryMode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
