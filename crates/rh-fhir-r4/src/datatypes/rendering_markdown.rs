use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// markdown
///
/// This is an equivalent of the string on which the extension is sent, but includes additional markdown (see documentation about [markdown](datatypes.html#markdown). Note that using HTML  [xhtml](extension-rendering-xhtml.html) can allow for greater precision of display.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/rendering-markdown
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingMarkdown {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for RenderingMarkdown {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
