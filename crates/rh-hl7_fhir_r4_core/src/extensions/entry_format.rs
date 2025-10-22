use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// entryFormat
///
/// Additional instructions for the user to guide their input (i.e. a human readable version of a regular expression like "nnn-nnn-nnn"). In most UIs this is the placeholder (or 'ghost') text placed directly inside the edit controls and that disappear when the control gets the focus.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/entryFormat
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryFormat {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for EntryFormat {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
