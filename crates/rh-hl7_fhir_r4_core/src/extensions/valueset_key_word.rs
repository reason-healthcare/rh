use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// keyWord
///
/// Word or words used in an information retrieval system to indicate the content of the value set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-keyWord
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetKeyWord {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetKeyWord {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
