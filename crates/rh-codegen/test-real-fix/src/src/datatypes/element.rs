use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Element
///
/// Base definition for all elements in a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Element
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}
