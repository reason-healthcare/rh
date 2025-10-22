use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// exposureDuration
///
/// The amount of time the individual was exposed to the Substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-exposureDuration
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRExposureDuration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRExposureDuration {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
