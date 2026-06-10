use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/permission-rule-combining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionRuleCombining {
    /// Deny-overrides
    #[serde(rename = "deny-overrides")]
    DenyOverrides,
    /// Permit-overrides
    #[serde(rename = "permit-overrides")]
    PermitOverrides,
    /// Ordered-deny-overrides
    #[serde(rename = "ordered-deny-overrides")]
    OrderedDenyOverrides,
    /// Ordered-permit-overrides
    #[serde(rename = "ordered-permit-overrides")]
    OrderedPermitOverrides,
    /// Deny-unless-permit
    #[serde(rename = "deny-unless-permit")]
    DenyUnlessPermit,
    /// Permit-unless-deny
    #[serde(rename = "permit-unless-deny")]
    PermitUnlessDeny,
}
impl Default for PermissionRuleCombining {
    fn default() -> Self {
        Self::DenyOverrides
    }
}
