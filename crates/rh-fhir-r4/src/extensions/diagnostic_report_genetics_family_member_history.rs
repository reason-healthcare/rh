use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// FamilyMemberHistory
///
/// Significant health events and conditions for a person related to the patient relevant in the context of care for the patient.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsFamilyMemberHistory
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsFamilyMemberHistory {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportGeneticsFamilyMemberHistory {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
