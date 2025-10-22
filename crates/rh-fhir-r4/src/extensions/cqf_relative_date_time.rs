use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// relativeDateTime
///
/// A date/time value that is determined based on a duration offset from a target event.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-relativeDateTime
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFRelativeDateTime {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFRelativeDateTime {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
