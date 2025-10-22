use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// ADXP-delimiter
///
/// Delimiters are printed without framing white space. If no value component is provided, the delimiter appears as a line break.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-ADXP-delimiter
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADXPDelimiter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADXPDelimiter {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
