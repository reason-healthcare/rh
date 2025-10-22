use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// replaces
///
/// Completed or terminated task(s) whose function is taken by this new task.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/task-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TaskReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
