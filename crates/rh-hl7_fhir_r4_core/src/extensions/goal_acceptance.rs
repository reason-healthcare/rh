use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Goal acceptance
///
/// Information about the acceptance and relative priority assigned to the goal by the patient, practitioners and other stake-holders.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-acceptance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAcceptance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for GoalAcceptance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
