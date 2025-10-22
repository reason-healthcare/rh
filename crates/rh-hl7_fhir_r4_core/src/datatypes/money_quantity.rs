use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// MoneyQuantity
///
/// An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MoneyQuantity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyQuantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for MoneyQuantity {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
