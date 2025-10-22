use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// mothers-family
///
/// The portion of the family name that is derived from the person's mother.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/humanname-mothers-family
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumannameMothersFamily {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HumannameMothersFamily {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
