use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-deliveryInstallationArea
///
/// The location of the delivery installation, usually a town or city, and is only required if the area is different from the municipality. Area to which mail delivery service is provided from any postal facility or service such as an individual letter carrier, rural route, or postal route.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-deliveryInstallationArea
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDeliveryInstallationArea {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDeliveryInstallationArea {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
