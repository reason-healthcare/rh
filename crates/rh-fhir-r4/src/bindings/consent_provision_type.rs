use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/consent-provision-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentProvisionType {
    /// Opt Out
    #[serde(rename = "deny")]
    Deny,
    /// Opt In
    #[serde(rename = "permit")]
    Permit,
}
impl Default for ConsentProvisionType {
    fn default() -> Self {
        Self::Deny
    }
}
