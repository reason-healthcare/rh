use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// GenomicSourceClass
///
/// Source of sample used to determine the sequence in sequencing lab -- germline, somatic, prenatal. LOINC Code: ([48002-0](http://loinc.org/48002-0)).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsGenomicSourceClass
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsGenomicSourceClass {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsGenomicSourceClass {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
