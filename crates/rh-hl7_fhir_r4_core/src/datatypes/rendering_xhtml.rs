use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// xhtml
///
/// This is an equivalent of the string on which the extension is sent, but includes additional XHTML markup, such as bold, italics, styles, tables, etc. Existing [restrictions on XHTML content](narrative.html#security) apply. Note that using [markdown](extension-rendering-markdown.html) allows for greater flexibility of display.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendering-xhtml
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingXhtml {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderingXhtml {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
