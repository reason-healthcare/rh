use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// nullFlavor
///
/// If the value is not a proper value, indicates the reason.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-nullFlavor
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090NullFlavor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090NullFlavor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
