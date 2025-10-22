use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// approachBodyStructure
///
/// The approach body site used for this procedure.  Multiple locations are allowed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-approachBodyStructure
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureApproachBodyStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureApproachBodyStructure {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
