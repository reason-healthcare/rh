use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Applicable Version
///
/// An additional version that this profile apples to, other than the version explicitly stated in StructureDefinition.fhirVersion.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/structuredefinition-applicable-version
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredefinitionApplicableVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for StructuredefinitionApplicableVersion {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
