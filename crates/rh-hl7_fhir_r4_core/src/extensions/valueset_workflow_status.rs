use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// workflowStatus
///
/// Workflow Status is used to represent details of the value set development process while in a single Activity Status.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-workflowStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetWorkflowStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetWorkflowStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
