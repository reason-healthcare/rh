use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// history
///
/// Information on changes made to the Value Set Definition over time, and also has a contained audit trail of all such changes.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-history
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemHistory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
