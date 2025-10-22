use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expression
///
/// An expression that, when evaluated, provides the value for the element on which it appears.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-expression
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CQFExpression {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CQFExpression {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
