use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// conceptOrder
///
/// Identifies the relative order in which concepts within the value set should be presented to a user.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-conceptOrder
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemConceptOrder {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemConceptOrder {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
