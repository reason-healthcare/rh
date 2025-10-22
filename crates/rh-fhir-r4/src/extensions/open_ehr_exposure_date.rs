use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// exposureDate
///
/// Record of the date and/or time of the first exposure to the Substance for this Reaction Event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-exposureDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRExposureDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRExposureDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
