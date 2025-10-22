use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Flag details
///
/// Points to the Observation, AllergyIntolerance or other record that provides additional supporting information about this flag.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/flag-detail
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FlagDetail {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
