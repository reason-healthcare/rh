use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// exposureDescription
///
/// Text description about exposure to the Substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-exposureDescription
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRExposureDescription {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRExposureDescription {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
