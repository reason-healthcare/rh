use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// style
///
/// Identifies how the specified element should be rendered when displayed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendering-style
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingStyle {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderingStyle {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
