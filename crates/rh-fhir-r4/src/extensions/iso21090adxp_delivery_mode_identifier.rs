use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-deliveryModeIdentifier
///
/// Represents the routing information such as a letter carrier route number. It is the identifying number of the designator (the box number or rural route number).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-deliveryModeIdentifier
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDeliveryModeIdentifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDeliveryModeIdentifier {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
