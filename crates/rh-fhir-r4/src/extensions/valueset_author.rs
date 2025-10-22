use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// author
///
/// The entity or set of entities that create and may modify the Value Set Definition content. The name of a group or an individual, along with contact details.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-author
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetAuthor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetAuthor {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
