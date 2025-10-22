use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// occurredFollowing
///
/// Further conditions, problems, diagnoses, procedures or events or the substance that preceded this Condition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/condition-occurredFollowing
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionOccurredFollowing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ConditionOccurredFollowing {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
