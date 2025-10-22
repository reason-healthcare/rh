use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Geolocation
///
/// The absolute geographic location of the Location, expressed using the WGS84 datum (This is the same co-ordinate system used in KML).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/geolocation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geolocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Geolocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
