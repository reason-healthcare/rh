use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// allele-database
///
/// Allele Database.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/hla-genotyping-results-allele-database
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HlaGenotypingResultsAlleleDatabase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for HlaGenotypingResultsAlleleDatabase {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
