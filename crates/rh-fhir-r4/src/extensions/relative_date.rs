use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Relative Date Criteria
///
/// Specifies that a date is relative to some event. The event happens [Duration] after [Event].
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/relative-date
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RelativeDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
