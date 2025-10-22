use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Interpretation
///
/// Clinical Interpretations for variant. It's a reference to an Observation resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsInterpretation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsInterpretation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsInterpretation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
