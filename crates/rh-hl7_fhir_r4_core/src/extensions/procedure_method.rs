use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// method
///
/// The method used to perform this procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/procedure-method
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureMethod {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ProcedureMethod {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
