use crate::bindings::currencies::Currencies;
use crate::datatypes::element::Element;
use crate::primitives::decimal::DecimalType;
use serde::{Deserialize, Serialize};
/// Money
///
/// Base StructureDefinition for Money Type: An amount of economic utility in some recognized currency.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Money
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Money
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Numerical value (with implicit precision)
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// ISO 4217 Currency Code
    pub currency: Option<Currencies>,
    /// Extension element for the 'currency' primitive field. Contains metadata and extensions.
    pub _currency: Option<Element>,
}

impl Default for Money {
    fn default() -> Self {
        Self {
            base: Element::default(),
            value: Default::default(),
            _value: Default::default(),
            currency: Default::default(),
            _currency: Default::default(),
        }
    }
}
