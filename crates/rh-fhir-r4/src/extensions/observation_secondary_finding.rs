use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Secondary Finding
///
/// Secondary findings are genetic test results that provide information about variants in a gene unrelated to the primary purpose for the testing, most often discovered when [Whole Exome Sequencing (WES)](https://en.wikipedia.org/wiki/Exome_sequencing) or [Whole Genome Sequencing (WGS)](https://en.wikipedia.org/wiki/Whole_genome_sequencing) is performed. This extension should be used to denote when a genetic finding is being shared as a secondary finding, and ideally refer to a corresponding guideline or policy statement.  For more detail, please see: https://ghr.nlm.nih.gov/primer/testing/secondaryfindings.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-secondaryFinding
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationSecondaryFinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationSecondaryFinding {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
