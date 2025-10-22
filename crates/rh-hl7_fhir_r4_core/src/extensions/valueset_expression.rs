use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// expression
///
/// An expression that provides an alternative definition of the content of the value set. There are two different ways to use this expression extension: If both an expression and a compose element is present, the compose is understood the make the same statement as the expression. If there is no compose, the expression is the only definition of the value set, and the value set can only be processed by a server that understands the expression syntax, it that is computable.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/valueset-expression
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesetExpression {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ValuesetExpression {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
