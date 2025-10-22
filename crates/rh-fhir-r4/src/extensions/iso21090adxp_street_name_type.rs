use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-streetNameType
///
/// The designation given to the street. (e.g. Street, Avenue, Crescent, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-streetNameType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPStreetNameType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPStreetNameType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
