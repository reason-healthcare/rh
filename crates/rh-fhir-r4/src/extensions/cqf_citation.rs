use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// citation
///
/// A bibliographic citation for the related resource. This text SHOULD be formatted according to an accepted citation format.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-citation
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFCitation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFCitation {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
