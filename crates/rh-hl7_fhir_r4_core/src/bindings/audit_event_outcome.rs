use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/audit-event-outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventOutcome {
    /// Success
    #[serde(rename = "0")]
    Code0,
    /// Minor failure
    #[serde(rename = "4")]
    Code4,
    /// Serious failure
    #[serde(rename = "8")]
    Code8,
    /// Major failure
    #[serde(rename = "12")]
    Code12,
}
impl Default for AuditEventOutcome {
    fn default() -> Self {
        Self::Code0
    }
}
