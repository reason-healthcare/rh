use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/contributor-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributorType {
    /// Author
    #[serde(rename = "author")]
    Author,
    /// Editor
    #[serde(rename = "editor")]
    Editor,
    /// Reviewer
    #[serde(rename = "reviewer")]
    Reviewer,
    /// Endorser
    #[serde(rename = "endorser")]
    Endorser,
}
impl Default for ContributorType {
    fn default() -> Self {
        Self::Author
    }
}
