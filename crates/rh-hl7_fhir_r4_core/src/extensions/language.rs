use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Human Language of the item
///
/// The Human Language of the item.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/language
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
