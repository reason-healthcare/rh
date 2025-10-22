use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Based On
///
/// A plan, proposal or order that is fulfilled in whole or in part by this event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-basedOn
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBasedOn {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventBasedOn {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
