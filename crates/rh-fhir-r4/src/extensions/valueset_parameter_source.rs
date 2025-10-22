use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// parameterSource
///
/// Declares what the source of this parameter is.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-parameterSource
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetParameterSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetParameterSource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
