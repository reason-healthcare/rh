use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Performer Order
///
/// Identifies the relative preference of alternative performers when the request lists multiple performers.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/request-performerOrder
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPerformerOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RequestPerformerOrder {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
