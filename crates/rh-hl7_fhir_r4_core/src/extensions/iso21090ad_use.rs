use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// AD-use
///
/// Uses of Addresses - codes not defined as part of Address.use.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-AD-use
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ADUse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ADUse {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
