use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// location
///
/// The anatomical location / body site(s) where the symptoms manifested.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-location
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRLocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
