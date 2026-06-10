use crate::bindings::quantity_comparator::QuantityComparator;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::primitives::decimal::DecimalType;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Quantity
///
/// Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Quantity
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// Numerical value (with implicit precision)
    pub value: Option<DecimalType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// < | <= | >= | > | ad - how to understand the value
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

impl Default for Quantity {
    fn default() -> Self {
        Self {
            base: DataType::default(),
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
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
            rh_foundation::Invariant::new(
                "qty-3",
                rh_foundation::Severity::Error,
                "If a code for the unit is present, the system SHALL also be present",
                "code.empty() or system.exists()",
            ),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Quantity.comparator",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/quantity-comparator|5.0.0",
        )
        .with_description("How the Quantity should be understood and represented.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Quantity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.extension", 0, None),
            rh_foundation::ElementCardinality::new("Quantity.value", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.comparator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.unit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.code", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Quantity {
    fn resource_type(&self) -> &'static str {
        "Quantity"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Quantity")
    }
}
