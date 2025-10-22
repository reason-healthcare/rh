use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// directedBy
///
/// The target of the extension is a distinct actor from the requester and has decision-making authority over the service and takes direct responsibility to manage the service.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-directedBy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureDirectedBy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureDirectedBy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
