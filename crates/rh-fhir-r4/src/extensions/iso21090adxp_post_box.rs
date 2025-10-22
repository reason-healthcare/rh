use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-postBox
///
/// A numbered box located in a post station.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-postBox
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPPostBox {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPPostBox {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
