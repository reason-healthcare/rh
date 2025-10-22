use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Reason for current status
///
/// Captures the reason for the current state of the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-statusReason
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatusReason {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventStatusReason {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
