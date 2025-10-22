use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// bodyPosition
///
/// The position of the body when the observation was done, e.g. standing, sitting. To be used only when the body position in not precoordinated in the observation code.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-bodyPosition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationBodyPosition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationBodyPosition {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
