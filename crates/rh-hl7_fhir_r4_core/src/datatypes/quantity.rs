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
            rh_foundation::Invariant::new(
                "qty-3",
                rh_foundation::Severity::Error,
                "If a code for the unit is present, the system SHALL also be present",
                "code.empty() or system.exists()",
            )
            .with_xpath("not(exists(f:code)) or exists(f:system)"),
        ]
    });

impl crate::validation::ValidatableResource for Quantity {
    fn resource_type(&self) -> &'static str {
        "Quantity"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Quantity")
    }
}
