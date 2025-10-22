use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-careOf
///
/// The name of the party who will take receipt at the specified address, and will take on responsibility for ensuring delivery to the target recipient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-careOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPCareOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPCareOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
