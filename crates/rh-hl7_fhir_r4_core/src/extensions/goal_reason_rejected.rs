use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// reason rejected
///
/// The reason the goal was not accepted. Applies only if the status of the goal is rejected.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-reasonRejected
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalReasonRejected {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for GoalReasonRejected {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
