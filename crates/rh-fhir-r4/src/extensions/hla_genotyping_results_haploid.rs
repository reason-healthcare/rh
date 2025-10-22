use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// haploid
///
/// haploid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hla-genotyping-results-haploid
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlaGenotypingResultsHaploid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HlaGenotypingResultsHaploid {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
