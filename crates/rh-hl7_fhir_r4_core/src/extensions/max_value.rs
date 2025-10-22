use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// maxValue
///
/// The inclusive upper bound on the range of allowed values for the data element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/maxValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MaxValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
