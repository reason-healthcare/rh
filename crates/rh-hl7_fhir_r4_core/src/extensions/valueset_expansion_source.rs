use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expansionSource
///
/// The logical URL of the ValueSet definition that was used to generate this expansion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expansionSource
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpansionSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpansionSource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
