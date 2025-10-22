use crate::bindings::quantity_comparator::QuantityComparator;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Quantity
///
/// Base StructureDefinition for Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Quantity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Numerical value (with implicit precision)
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<QuantityComparator>,
    /// Extension element for the 'comparator' primitive field. Contains metadata and extensions.
    pub _comparator: Option<Element>,
    /// Unit representation
    pub unit: Option<StringType>,
    /// Extension element for the 'unit' primitive field. Contains metadata and extensions.
    pub _unit: Option<Element>,
    /// System that defines coded unit form
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// Coded form of the unit
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
}
/// precision
///
/// Explicit precision of the number. This is the number of significant decimal places after the decimal point, irrespective of how many are actually present in the explicitly represented decimal.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/quantity-precision
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityPrecision {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Quantity {
    fn default() -> Self {
        Self {
            base: Element::default(),
            value: Default::default(),
            _value: Default::default(),
            comparator: Default::default(),
            _comparator: Default::default(),
            unit: Default::default(),
            _unit: Default::default(),
            system: Default::default(),
            _system: Default::default(),
            code: Default::default(),
            _code: Default::default(),
        }
    }
}

impl Default for QuantityPrecision {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}
