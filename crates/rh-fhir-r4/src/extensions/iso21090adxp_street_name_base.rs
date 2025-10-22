use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-streetNameBase
///
/// The base name of a roadway or artery recognized by a municipality (excluding street type and direction).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-streetNameBase
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPStreetNameBase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPStreetNameBase {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
