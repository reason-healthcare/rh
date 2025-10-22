use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// test
///
/// Observations that confirm or refute the risk and/or the substance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-test
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRTest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRTest {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
