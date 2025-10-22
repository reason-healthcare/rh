use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/metric-color
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricColor {
    /// Color Black
    #[serde(rename = "black")]
    Black,
    /// Color Red
    #[serde(rename = "red")]
    Red,
    /// Color Green
    #[serde(rename = "green")]
    Green,
    /// Color Yellow
    #[serde(rename = "yellow")]
    Yellow,
    /// Color Blue
    #[serde(rename = "blue")]
    Blue,
    /// Color Magenta
    #[serde(rename = "magenta")]
    Magenta,
    /// Color Cyan
    #[serde(rename = "cyan")]
    Cyan,
    /// Color White
    #[serde(rename = "white")]
    White,
}
impl Default for MetricColor {
    fn default() -> Self {
        Self::Black
    }
}
