use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Replaces
///
/// This observation replaces a previous observation (i.e. a revised value).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/observation-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ObservationReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
