use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Replaces
///
/// Completed or terminated request(s) whose function is taken by this new request.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/request-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RequestReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
