use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// administration
///
/// Link to an actual medication administration record with the full details of the administration, if a link is known.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/openEHR-administration
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEHRAdministration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for OpenEHRAdministration {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
