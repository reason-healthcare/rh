use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// inheritedExtensibleValueSet
///
/// A reference to an extensible value set specified in a parent profie in order to allow a conformance checking tool to validate that a code not in the extensible value set of the profile is not violating rules defined by parent profile bindings.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/elementdefinition-inheritedExtensibleValueSet
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementdefinitionInheritedExtensibleValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for ElementdefinitionInheritedExtensibleValueSet {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
