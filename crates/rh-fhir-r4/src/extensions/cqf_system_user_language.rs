use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// systemUserLanguage
///
/// Preferred language of the person using the system.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-systemUserLanguage
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFSystemUserLanguage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFSystemUserLanguage {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
