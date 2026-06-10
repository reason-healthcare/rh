use serde::{Deserialize, Serialize};
/// Generated enum for ValueSet: http://hl7.org/fhir/ValueSet/imagingselection-3dgraphictype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Imagingselection3dgraphictype {
    /// POINT
    #[serde(rename = "point")]
    Point,
    /// MULTIPOINT
    #[serde(rename = "multipoint")]
    Multipoint,
    /// POLYLINE
    #[serde(rename = "polyline")]
    Polyline,
    /// POLYGON
    #[serde(rename = "polygon")]
    Polygon,
    /// ELLIPSE
    #[serde(rename = "ellipse")]
    Ellipse,
    /// ELLIPSOID
    #[serde(rename = "ellipsoid")]
    Ellipsoid,
}
impl Default for Imagingselection3dgraphictype {
    fn default() -> Self {
        Self::Point
    }
}
