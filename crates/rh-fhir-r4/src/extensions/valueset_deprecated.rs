use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// deprecated
///
/// if ture, indicates that the concept is deprecated from the value set - that is, it should not be used, and is planned to be withdrawn.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-deprecated
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetDeprecated {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetDeprecated {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
