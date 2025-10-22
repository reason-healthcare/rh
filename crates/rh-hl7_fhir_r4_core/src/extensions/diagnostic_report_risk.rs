use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Risk
///
/// Provides a link to an assessment of prognosis or risk as informed by the diagnostic results (For example, genetic results and possibly by patient genetic family history information).  This extension is used when need RiskAssessment as an alternate choice  for `Observation.hasMember` or `DiagnosticReport.result`.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/diagnosticReport-risk
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReportRisk {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for DiagnosticReportRisk {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
