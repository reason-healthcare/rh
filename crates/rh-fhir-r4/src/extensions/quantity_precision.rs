use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// precision
///
/// Explicit precision of the number. This is the number of significant decimal places after the decimal point, irrespective of how many are actually present in the explicitly represented decimal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/quantity-precision
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityPrecision {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuantityPrecision {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
