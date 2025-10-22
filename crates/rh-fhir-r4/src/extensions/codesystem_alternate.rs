use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// alternate
///
/// An additional code that may be used to represent the concept.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-alternate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemAlternate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemAlternate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
