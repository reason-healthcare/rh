use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Overrides Composition.subject
///
/// Specifies that the section has a different subject that the Composition, or it's container section.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/composition-section-subject
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionSectionSubject {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for CompositionSectionSubject {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
