use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/substance-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstanceStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
}
impl Default for SubstanceStatus {
    fn default() -> Self {
        Self::Active
    }
}
