use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// warning
///
/// An extra warning about the correct use of the value set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-warning
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemWarning {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemWarning {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
