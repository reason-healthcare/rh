use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// causedBy
///
/// This procedure is because of the related item.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-causedBy
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureCausedBy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureCausedBy {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
