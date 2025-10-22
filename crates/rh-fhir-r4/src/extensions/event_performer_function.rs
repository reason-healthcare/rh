use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Performer Function
///
/// Distinguishes the type of involvement of the performer in the event. For example, 'author',  'verifier' or 'responsible party'.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-performerFunction
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPerformerFunction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventPerformerFunction {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
