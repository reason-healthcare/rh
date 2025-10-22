use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/units-of-time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitsOfTime {
    /// second
    #[serde(rename = "s")]
    S,
    /// minute
    #[serde(rename = "min")]
    Min,
    /// hour
    #[serde(rename = "h")]
    H,
    /// day
    #[serde(rename = "d")]
    D,
    /// week
    #[serde(rename = "wk")]
    Wk,
    /// month
    #[serde(rename = "mo")]
    Mo,
    /// year
    #[serde(rename = "a")]
    A,
}
impl Default for UnitsOfTime {
    fn default() -> Self {
        Self::S
    }
}
