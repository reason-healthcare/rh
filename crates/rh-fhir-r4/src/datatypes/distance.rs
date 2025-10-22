use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Distance
///
/// Base StructureDefinition for Distance Type: A length - a value with a unit that is a physical distance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Distance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Distance
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for Distance {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
