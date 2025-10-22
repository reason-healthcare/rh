use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// otherName
///
/// Human readable names for the codesystem.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-otherName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemOtherName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemOtherName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
