use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Allele
///
/// Allele information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAllele
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAllele {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsAllele {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
