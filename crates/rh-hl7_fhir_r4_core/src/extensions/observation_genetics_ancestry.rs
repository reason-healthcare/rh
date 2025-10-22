use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Ancestry
///
/// Ancestry information.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-geneticsAncestry
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationGeneticsAncestry {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationGeneticsAncestry {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
