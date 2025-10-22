use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Distance
///
/// A calculated distance between the resource and a provided location.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/location-distance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDistance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for LocationDistance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
