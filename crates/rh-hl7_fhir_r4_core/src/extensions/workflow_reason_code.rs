use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Reason Code
///
/// Describes why the event occurred in coded or textual form.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-reasonCode
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowReasonCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowReasonCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
