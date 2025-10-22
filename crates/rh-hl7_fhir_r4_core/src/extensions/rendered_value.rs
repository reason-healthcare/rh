use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Rendered Value
///
/// Provides a rendered version of the value intended for human display.  For example, a sensitive identifier (e.g. social security number) partially obscured by asterisks; a drivers licence number with dashes inserted; a date formatted as MMM dd, yyyy; etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendered-value
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderedValue {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
