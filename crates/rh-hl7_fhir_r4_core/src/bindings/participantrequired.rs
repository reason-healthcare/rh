use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/participantrequired
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Participantrequired {
    /// Required
    #[serde(rename = "required")]
    Required,
    /// Optional
    #[serde(rename = "optional")]
    Optional,
    /// Information Only
    #[serde(rename = "information-only")]
    InformationOnly,
}
impl Default for Participantrequired {
    fn default() -> Self {
        Self::Required
    }
}
