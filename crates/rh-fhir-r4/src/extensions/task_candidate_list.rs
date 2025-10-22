use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Candidate List
///
/// Identifies the individuals who are candidates for being the owner of the task.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/task-candidateList
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCandidateList {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TaskCandidateList {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
