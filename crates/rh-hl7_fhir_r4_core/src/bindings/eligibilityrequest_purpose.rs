use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/eligibilityrequest-purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EligibilityrequestPurpose {
    /// Coverage auth-requirements
    #[serde(rename = "auth-requirements")]
    AuthRequirements,
    /// Coverage benefits
    #[serde(rename = "benefits")]
    Benefits,
    /// Coverage Discovery
    #[serde(rename = "discovery")]
    Discovery,
    /// Coverage Validation
    #[serde(rename = "validation")]
    Validation,
}
impl Default for EligibilityrequestPurpose {
    fn default() -> Self {
        Self::AuthRequirements
    }
}
