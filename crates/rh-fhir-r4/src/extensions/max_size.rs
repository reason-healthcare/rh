use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// maxSize
///
/// For attachment answers, indicates the maximum size an attachment can be.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/maxSize
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxSize {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MaxSize {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
