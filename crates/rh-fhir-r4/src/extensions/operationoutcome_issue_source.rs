use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Source of Issue
///
/// Helps a user track down the source of the problem.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/operationoutcome-issue-source
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationoutcomeIssueSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OperationoutcomeIssueSource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
