use crate::datatypes::extension::Extension;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Element
///
/// Base StructureDefinition for Element Type: Base definition for all elements in a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Element
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Unique id for inter-element referencing
    pub id: Option<StringType>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: Default::default(),
            extension: Default::default(),
        }
    }
}
