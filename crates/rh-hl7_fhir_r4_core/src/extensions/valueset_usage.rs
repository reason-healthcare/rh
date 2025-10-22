use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// usage
///
/// Consumers of the value set and the implementations, projects or standards that the author has utilized the value set in.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-usage
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetUsage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetUsage {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
