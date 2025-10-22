use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// systemRef
///
/// The formal URI for the code system.  I.e. ValueSet.codeSystem.system (or its equivalent).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-systemRef
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetSystemRef {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetSystemRef {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
