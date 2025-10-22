use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Supporting Information
///
/// Other resources *from the patient record* that may be relevant to the event.  The information from these resources was either used to create the instance or is provided to help with its interpretation.  This extension **should not** be used if more specific  inline elements  or extensions are available.  For example, use `Observation.hasMember`  instead of supportingInformation for  representing the members of an Observation panel.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/workflow-supportingInfo
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSupportingInfo {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for WorkflowSupportingInfo {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
