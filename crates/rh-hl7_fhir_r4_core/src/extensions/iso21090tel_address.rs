use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// TEL-address
///
/// A V3 compliant, RFC 3966 conformant URI version of the telephone or fax number.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-TEL-address
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090TELAddress {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090TELAddress {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
