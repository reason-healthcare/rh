use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Anonymized
///
/// True or False indicating whether all patient identifying information was removed from the data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/auditevent-Anonymized
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditeventAnonymized {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for AuditeventAnonymized {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
