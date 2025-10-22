use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// fmm-no-warnings
///
/// The FMM level that would be assigned to the artifact if it had no warnings.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-fmm-no-warnings
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionFmmNoWarnings {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionFmmNoWarnings {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
