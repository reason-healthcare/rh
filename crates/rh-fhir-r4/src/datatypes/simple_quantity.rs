use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// SimpleQuantity
///
/// A fixed quantity (no comparator)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SimpleQuantity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleQuantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for SimpleQuantity {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
