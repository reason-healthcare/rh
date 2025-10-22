use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Conformance expectation
///
/// Defines the level of expectation associated with a given system capability.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/capabilitystatement-expectation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitystatementExpectation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CapabilitystatementExpectation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
