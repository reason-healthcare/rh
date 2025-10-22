use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/versioning-policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersioningPolicy {
    /// No VersionId Support
    #[serde(rename = "no-version")]
    NoVersion,
    /// Versioned
    #[serde(rename = "versioned")]
    Versioned,
    /// VersionId tracked fully
    #[serde(rename = "versioned-update")]
    VersionedUpdate,
}
impl Default for VersioningPolicy {
    fn default() -> Self {
        Self::NoVersion
    }
}
