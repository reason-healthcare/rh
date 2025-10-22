use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Event Location
///
/// The principal physical location where the {{title}} was performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-location
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLocation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventLocation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
