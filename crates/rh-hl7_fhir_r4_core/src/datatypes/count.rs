use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Count
///
/// Base StructureDefinition for Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Count
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Count
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for Count {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
        }
    }
}
