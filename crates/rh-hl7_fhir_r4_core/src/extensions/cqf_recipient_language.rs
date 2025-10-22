use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// recipientLanguage
///
/// Preferred language of the person that will consume the content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-recipientLanguage
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFRecipientLanguage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFRecipientLanguage {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
