use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// author
///
/// User or Org actually involved in creating the value set content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-author
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemAuthor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemAuthor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
