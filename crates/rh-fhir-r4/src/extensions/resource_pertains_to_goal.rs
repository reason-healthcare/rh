use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// pertains to goal
///
/// Indicates that the resource is related to either the measurement, achievement or progress towards the referenced goal.  For example, a Procedure to exercise pertainsToGoal of losing weight.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-pertainsToGoal
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePertainsToGoal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ResourcePertainsToGoal {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
