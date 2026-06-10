use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/formularyitem-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormularyitemStatus {
    /// Active
    #[serde(rename = "active")]
    Active,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Inactive
    #[serde(rename = "inactive")]
    Inactive,
}
impl Default for FormularyitemStatus {
    fn default() -> Self {
        Self::Active
    }
}
