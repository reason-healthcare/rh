use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// library
///
/// A reference to a Library containing the formal logic used by the artifact.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-library
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFLibrary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFLibrary {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
