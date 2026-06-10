use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/imagingselection-status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImagingselectionStatus {
    /// Available
    #[serde(rename = "available")]
    Available,
    /// Entered in Error
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    /// Unknown
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for ImagingselectionStatus {
    fn default() -> Self {
        Self::Available
    }
}
