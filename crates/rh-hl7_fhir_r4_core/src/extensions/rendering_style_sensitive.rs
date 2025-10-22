use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// styleSensitive
///
/// Indicates that the style extensions (style, markdown and xhtml) in this resource instance are essential to the interpretation of the instance and that systems that are not capable of rendering using those extensions should not be used to render the resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendering-styleSensitive
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingStyleSensitive {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderingStyleSensitive {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
