use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// display-hint
///
/// Hinting information for the narrative generator - a series of name: value; pairs.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-display-hint
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionDisplayHint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionDisplayHint {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
