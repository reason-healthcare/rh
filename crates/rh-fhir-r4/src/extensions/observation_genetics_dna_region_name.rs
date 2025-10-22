use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// DNARegionName
///
/// A human readable name for the region of interest. Typically Exon #, Intron # or other. NOTE: This is not standardized and is mainly for convenience and display purposes.  LOINC Code: ([47999-8](http://loinc.org/47999-8)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsDNARegionName
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsDNARegionName {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsDNARegionName {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
