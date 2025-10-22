use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// authoritativeSource
///
/// A reference to the authoritative accessible, persisted source of truth of the entire Value Set Definition, including textual information and available versions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-authoritativeSource
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetAuthoritativeSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetAuthoritativeSource {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
