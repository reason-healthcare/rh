use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Comment
///
/// A comment that explains how this code is used in this context (where the value set is expected to be used).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-concept-comments
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetConceptComments {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetConceptComments {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
