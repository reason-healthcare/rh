use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// primaryInd
///
/// Flag indicating if the specialty is the primary specialty of the provider. Normally, a practitioner will have one primary specialty, but in some cases more than one can be primary.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/practitionerrole-primaryInd
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PractitionerrolePrimaryInd {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for PractitionerrolePrimaryInd {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
