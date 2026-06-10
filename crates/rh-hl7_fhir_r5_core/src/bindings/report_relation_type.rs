use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/report-relation-type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportRelationType {
    /// Replaces
    #[serde(rename = "replaces")]
    Replaces,
    /// Amends
    #[serde(rename = "amends")]
    Amends,
    /// Appends
    #[serde(rename = "appends")]
    Appends,
    /// Transforms
    #[serde(rename = "transforms")]
    Transforms,
    /// Replaced With
    #[serde(rename = "replacedWith")]
    ReplacedWith,
    /// Amended With
    #[serde(rename = "amendedWith")]
    AmendedWith,
    /// Appended With
    #[serde(rename = "appendedWith")]
    AppendedWith,
    /// Transformed With
    #[serde(rename = "transformedWith")]
    TransformedWith,
}
impl Default for ReportRelationType {
    fn default() -> Self {
        Self::Replaces
    }
}
