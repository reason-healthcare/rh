use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// minLength
///
/// The minimum number of characters that must be present in the simple data type to be considered a "valid" instance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/minLength
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinLength {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for MinLength {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
