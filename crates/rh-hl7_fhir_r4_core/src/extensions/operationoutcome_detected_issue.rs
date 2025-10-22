use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Clinical Issue
///
/// A reference to a stored contraindication that is the basis for this issue. A recipient can expect that the item referenced in this extension is being retained for record keeping purposes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/operationoutcome-detectedIssue
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationoutcomeDetectedIssue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OperationoutcomeDetectedIssue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
