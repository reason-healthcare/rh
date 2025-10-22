use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Ratio
///
/// Base StructureDefinition for Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Ratio
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Ratio
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratio {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Numerator value
    pub numerator: Option<Quantity>,
    /// Denominator value
    pub denominator: Option<Quantity>,
}

impl Default for Ratio {
    fn default() -> Self {
        Self {
            base: Element::default(),
            numerator: Default::default(),
            denominator: Default::default(),
        }
    }
}
