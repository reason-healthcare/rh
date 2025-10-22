use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/namingsystem-identifier-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NamingsystemIdentifierType {
    /// OID
    #[serde(rename = "oid")]
    Oid,
    /// UUID
    #[serde(rename = "uuid")]
    Uuid,
    /// URI
    #[serde(rename = "uri")]
    Uri,
    /// Other
    #[serde(rename = "other")]
    Other,
}
impl Default for NamingsystemIdentifierType {
    fn default() -> Self {
        Self::Oid
    }
}
