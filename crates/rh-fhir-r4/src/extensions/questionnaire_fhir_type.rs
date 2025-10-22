use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// fhirType
///
/// For questionnaires generated from FHIR profiles, indicates the FHIR data type or resource type that corresponds to this node.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/questionnaire-fhirType
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireFHIRType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for QuestionnaireFHIRType {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
