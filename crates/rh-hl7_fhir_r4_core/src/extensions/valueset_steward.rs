use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// steward
///
/// The entity that is responsible for the content of the Value Set Definition. This is a textual description of the organizational entity responsible for the content and maintenance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-steward
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSteward {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSteward {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
