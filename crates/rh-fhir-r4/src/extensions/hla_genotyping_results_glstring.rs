use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// glstring
///
/// glstring.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hla-genotyping-results-glstring
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlaGenotypingResultsGlstring {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HlaGenotypingResultsGlstring {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
