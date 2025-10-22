use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Rule Reference
///
/// A reference to where the rule is defined (based on the authoritative URLs in the applicable conformance resources).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/operationoutcome-authority
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationoutcomeAuthority {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OperationoutcomeAuthority {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
