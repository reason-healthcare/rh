use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Part Of
///
/// A larger event of which this particular event is a component or step.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/event-partOf
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPartOf {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EventPartOf {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
