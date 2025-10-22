use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// summary
///
/// Additional text for the summary presentation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-summary
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionSummary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionSummary {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
