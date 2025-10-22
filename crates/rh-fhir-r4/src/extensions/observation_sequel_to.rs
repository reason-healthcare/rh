use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Sequel To
///
/// This observation follows the target observation (e.g. timed tests such as Glucose Tolerance Test).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-sequelTo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSequelTo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationSequelTo {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
