use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// minValue
///
/// The inclusive lower bound on the range of allowed values for the data element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/minValue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MinValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
