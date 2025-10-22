use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// effectiveDate
///
/// This is the first date-time when the value set version becomes active, so this value is present on Inactive value set versions as well. The start Date_time is expected to be as of 0001 UTC of the Effective Date.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-effectiveDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetEffectiveDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetEffectiveDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
