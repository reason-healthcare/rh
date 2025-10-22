use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// method
///
/// The platform, methodology and software applied at the time of thegenotyping.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hla-genotyping-results-method
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlaGenotypingResultsMethod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HlaGenotypingResultsMethod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
