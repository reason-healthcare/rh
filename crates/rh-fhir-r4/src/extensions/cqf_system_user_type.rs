use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// systemUserType
///
/// The type of user initiating the request, e.g. patient, healthcare provider, or specific type of healthcare provider (physician, nurse, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-systemUserType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFSystemUserType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFSystemUserType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
