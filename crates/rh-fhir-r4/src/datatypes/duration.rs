use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Duration
///
/// Base StructureDefinition for Duration Type: A length of time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Duration
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Duration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
