use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// related goal
///
/// Establishes a relationship between this goal and other goals.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-relationship
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRelationship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for GoalRelationship {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
