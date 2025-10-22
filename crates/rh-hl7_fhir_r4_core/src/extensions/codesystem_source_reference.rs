use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// sourceReference
///
/// This text is intended to act as a citation to work done elsewhere that is not part of the current stewarding process where the referenced source is in some way a basis of the current value set definition.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/codesystem-sourceReference
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodesystemSourceReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CodesystemSourceReference {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
