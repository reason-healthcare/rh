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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            )
            .with_xpath("@value|f:*|h:div"),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            )
            .with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
        ]
    });

impl crate::validation::ValidatableResource for Money {
    fn resource_type(&self) -> &'static str {
        "Money"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Money")
    }
}
