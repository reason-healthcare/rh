use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/link-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    /// Replaced-by
    #[serde(rename = "replaced-by")]
    ReplacedBy,
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Refer
    #[serde(rename = "refer")]
    Refer,
    /// See also
    #[serde(rename = "seealso")]
    Seealso,
}
impl Default for LinkType {
    fn default() -> Self {
        Self::ReplacedBy
    }
}
