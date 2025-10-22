use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// modeOfArrival
///
/// Identifies whether a patient arrives at the reporting facility via ambulance and the type of ambulance that was used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/encounter-modeOfArrival
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterModeOfArrival {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EncounterModeOfArrival {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
