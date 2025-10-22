use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Episode of Care
///
/// The episode(s) of care that establishes the context for this {{title}}.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-episodeOfCare
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEpisodeOfCare {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowEpisodeOfCare {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
