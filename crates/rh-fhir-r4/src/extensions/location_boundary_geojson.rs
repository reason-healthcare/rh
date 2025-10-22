use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Boundary (GeoJSON)
///
/// A boundary shape that represents the outside edge of the location (in GeoJSON format) This shape may have holes, and disconnected shapes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/location-boundary-geojson
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationBoundaryGeojson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for LocationBoundaryGeojson {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
