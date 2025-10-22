use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// regex
///
/// A regular expression that defines the syntax for the data element to be considered valid.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/regex
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regex {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Regex {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
