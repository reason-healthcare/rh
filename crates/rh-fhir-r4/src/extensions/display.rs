use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Display Name
///
/// The title or other name to display when referencing a resource by canonical URL.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/display
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Display {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
