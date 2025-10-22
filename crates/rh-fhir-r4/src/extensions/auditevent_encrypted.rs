use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Encrypted
///
/// True or False indicating whether the data was encrypted.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-Encrypted
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventEncrypted {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventEncrypted {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
