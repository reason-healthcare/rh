use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Flag Priority
///
/// A code that identifies the priority of the alert, for example the Alert Priority flags column in IHE PCD TF 2 Table B.8-4.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/flag-priority
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagPriority {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for FlagPriority {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
