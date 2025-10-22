use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/audit-event-action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventAction {
    /// Create
    #[serde(rename = "C")]
    C,
    /// Read/View/Print
    #[serde(rename = "R")]
    R,
    /// Update
    #[serde(rename = "U")]
    U,
    /// Delete
    #[serde(rename = "D")]
    D,
    /// Execute
    #[serde(rename = "E")]
    E,
}
impl Default for AuditEventAction {
    fn default() -> Self {
        Self::C
    }
}
