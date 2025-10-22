use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Reagent
///
/// Reference to reagents used to generate this observation.  This is intended for this for in-lab transactions between instruments and Laboratory Information Systems (LIS).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-reagent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReagent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationReagent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
