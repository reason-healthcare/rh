use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// An IANA timezone code for  the timezone offset per BCP 175
///
/// An IANA timezone code for  the timezone offset per [BCP 175](https://www.iana.org/go/rfc6557). The offset is specified as part of a dateTime/instant (or using the tzOffset extension on a date if necessary). The timezone code may also be provided to allow for human display of the location associated with the offset.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/tz-code
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TzCode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for TzCode {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
