use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// mimeType
///
/// Identifies the kind(s) of attachment allowed to be sent for an element.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/mimeType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimeType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MimeType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
