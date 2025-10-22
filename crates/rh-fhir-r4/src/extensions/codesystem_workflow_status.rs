use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// workflowStatus
///
/// Workflow Status is used to represent details of the value set development process while the value set has an Activity Status of Preliminary. The development of a value set often follows a formal workflow process from initiation to completion, and this element carries the state variable for this state machine. The assumption is that when first created a value set would have a workflow state of Draft. Additional workflow states may be used.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-workflowStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemWorkflowStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemWorkflowStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
