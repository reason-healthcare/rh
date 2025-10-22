use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// maxDecimalPlaces
///
/// Identifies the maximum number of decimal places that may be specified for the data element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/maxDecimalPlaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxDecimalPlaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MaxDecimalPlaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
