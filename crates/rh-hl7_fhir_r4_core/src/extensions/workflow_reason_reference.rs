use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Reason Reference
///
/// Indicates another resource whose existence justifies this event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-reasonReference
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowReasonReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowReasonReference {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
