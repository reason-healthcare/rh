use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// changeBase
///
/// Reference to the List that a "change" list is asserting changes with respect to.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/list-changeBase
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChangeBase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ListChangeBase {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
