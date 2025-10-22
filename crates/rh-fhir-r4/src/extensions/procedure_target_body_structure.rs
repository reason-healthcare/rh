use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// targetBodyStructure
///
/// The target body site used for this procedure.  Multiple locations are allowed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-targetBodyStructure
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureTargetBodyStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureTargetBodyStructure {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
