use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/explanationofbenefit-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplanationofbenefitStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    /// Draft
    #[serde(rename = "draft")]
    Draft,
    /// Entered In Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for ExplanationofbenefitStatus {
    fn default() -> Self {
        Self::Active
    }
}
