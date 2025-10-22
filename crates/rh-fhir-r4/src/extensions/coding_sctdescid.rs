use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// sctdescid
///
/// The SNOMED CT Description ID for the display.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/coding-sctdescid
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingSctdescid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodingSctdescid {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
