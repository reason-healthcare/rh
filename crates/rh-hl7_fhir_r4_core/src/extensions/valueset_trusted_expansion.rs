use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// trusted-expansion
///
/// Indicates an authoritative source for performing value set expansions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-trusted-expansion
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetTrustedExpansion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetTrustedExpansion {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
