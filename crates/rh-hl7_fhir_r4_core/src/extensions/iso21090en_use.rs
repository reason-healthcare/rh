use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// EN-use
///
/// A set of codes advising a system or user which name in a set of names to select for a given purpose.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/iso21090-EN-use
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso21090ENUse {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Iso21090ENUse {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
