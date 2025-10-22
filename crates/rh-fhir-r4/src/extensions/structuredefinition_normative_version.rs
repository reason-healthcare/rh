use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// First Normative Version
///
/// If this StructureDefinition is normative, which was the first normative version.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-normative-version
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionNormativeVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionNormativeVersion {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
