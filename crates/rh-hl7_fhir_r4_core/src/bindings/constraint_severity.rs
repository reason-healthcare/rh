use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/constraint-severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintSeverity {
    /// Error
    #[serde(rename = "error")]
    Error,
    /// Warning
    #[serde(rename = "warning")]
    Warning,
}
impl Default for ConstraintSeverity {
    fn default() -> Self {
        Self::Error
    }
}
