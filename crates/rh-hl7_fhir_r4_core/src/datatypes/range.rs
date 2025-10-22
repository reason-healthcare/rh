use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Range
///
/// Base StructureDefinition for Range Type: A set of ordered Quantities defined by a low and high limit.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Range
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Range
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Low limit
    pub low: Option<Quantity>,
    /// High limit
    pub high: Option<Quantity>,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            base: Element::default(),
            low: Default::default(),
            high: Default::default(),
        }
    }
}
