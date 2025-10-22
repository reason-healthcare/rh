use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// replacedby
///
/// A code that replaces this - use this code instead.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-replacedby
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemReplacedby {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemReplacedby {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
