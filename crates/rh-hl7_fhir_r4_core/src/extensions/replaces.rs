use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// replaces
///
/// Indicates a resource that this resource is replacing.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Replaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
