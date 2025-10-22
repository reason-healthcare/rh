use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// progressStatus
///
/// A code to track a detailed progress of  a procedure (e.g. In Recovery, Prepared for Surgery).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-progressStatus
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureProgressStatus {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureProgressStatus {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
