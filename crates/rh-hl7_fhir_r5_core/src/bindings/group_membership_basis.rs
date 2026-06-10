use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/group-membership-basis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupMembershipBasis {
    /// Definitional
    #[serde(rename = "definitional")]
    Definitional,
    /// Enumerated
    #[serde(rename = "enumerated")]
    Enumerated,
}
impl Default for GroupMembershipBasis {
    fn default() -> Self {
        Self::Definitional
    }
}
