use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Translation
///
/// Language translation from base language of resource to another language.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/translation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
