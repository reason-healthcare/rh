use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// CopyNumberEvent
///
/// A variation that increases or decreases the copy number of a given region ([SO:0001019](http://www.sequenceontology.org/browser/current_svn/term/SO:0001019)). Values: amplification/deletion/LOH.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsCopyNumberEvent
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsCopyNumberEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsCopyNumberEvent {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
