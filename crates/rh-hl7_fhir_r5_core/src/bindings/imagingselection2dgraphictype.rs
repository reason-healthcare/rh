use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/imagingselection-2dgraphictype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Imagingselection2dgraphictype {
    /// POINT
    #[serde(rename = "point")]
    Point,
    /// POLYLINE
    #[serde(rename = "polyline")]
    Polyline,
    /// INTERPOLATED
    #[serde(rename = "interpolated")]
    Interpolated,
    /// CIRCLE
    #[serde(rename = "circle")]
    Circle,
    /// ELLIPSE
    #[serde(rename = "ellipse")]
    Ellipse,
}
impl Default for Imagingselection2dgraphictype {
    fn default() -> Self {
        Self::Point
    }
}
