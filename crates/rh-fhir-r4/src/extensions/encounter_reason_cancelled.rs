use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// reasonCancelled
///
/// If the encountered was cancelled after it was planned, why? Applies only if the status is cancelled.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/encounter-reasonCancelled
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterReasonCancelled {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EncounterReasonCancelled {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
