use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// PhaseSet
///
/// Phase set information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsPhaseSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsPhaseSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsPhaseSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
