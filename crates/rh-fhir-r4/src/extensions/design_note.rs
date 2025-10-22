use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Design Note
///
/// Information captured by the author/maintainer of the questionnaire for development purposes, not intended to be seen by users.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/designNote
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignNote {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DesignNote {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
