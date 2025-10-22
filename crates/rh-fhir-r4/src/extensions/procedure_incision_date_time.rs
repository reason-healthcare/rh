use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// incisionDateTime
///
/// The time of the first incision.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-incisionDateTime
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureIncisionDateTime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureIncisionDateTime {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
