use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Gene
///
/// A region (or regions) that includes all of the sequence elements necessary to encode a functional transcript. A gene may include regulatory regions, transcribed regions and/or other functional sequence regions ([SO:0000704](http://www.sequenceontology.org/browser/current_svn/term/SO:0000704)). This element is the official gene symbol approved by the HGNC, which is a short abbreviated form of the gene name ([HGNC](http://www.genenames.org)). LOINC Code: ([48018-6](http://loinc.org/48018-6)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsGene
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsGene {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsGene {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
