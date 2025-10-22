use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Associated Study
///
/// Indicates that this event is relevant to the specified research study(ies).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-researchStudy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResearchStudy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowResearchStudy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
