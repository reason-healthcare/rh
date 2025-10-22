use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Analysis
///
/// Knowledge-based comments on the effect of the sequence on patient's condition/medication reaction.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport-geneticsAnalysis
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportGeneticsAnalysis {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportGeneticsAnalysis {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
