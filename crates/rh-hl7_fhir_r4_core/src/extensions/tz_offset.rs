use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Timezone offset, for date
///
/// Timezone offset, for dates where timezone is not allowed as part of the base date.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/tz-offset
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TzOffset {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TzOffset {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
