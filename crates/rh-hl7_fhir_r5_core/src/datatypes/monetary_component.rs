use crate::bindings::price_component_type::PriceComponentType;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::data_type::DataType;
use crate::datatypes::element::Element;
use crate::datatypes::money::Money;
use crate::primitives::decimal::DecimalType;
use serde::{Deserialize, Serialize};
/// MonetaryComponent
///
/// MonetaryComponent Type: Availability data for an {item}.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MonetaryComponent
/// - Version: 5.0.0
/// - Kind: complex-type
/// - Type: MonetaryComponent
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DataType
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DataType,
    /// base | surcharge | deduction | discount | tax | informational
    #[serde(rename = "type")]
    pub type_: PriceComponentType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.
    ///
    /// Binding: example (Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.)
    pub code: Option<CodeableConcept>,
    /// Factor used for calculating this component
    pub factor: Option<DecimalType>,
    /// Extension element for the 'factor' primitive field. Contains metadata and extensions.
    pub _factor: Option<Element>,
    /// Explicit value amount to be used
    pub amount: Option<Money>,
}

impl Default for MonetaryComponent {
    fn default() -> Self {
        Self {
            base: DataType::default(),
            type_: Default::default(),
            _type: Default::default(),
            code: Default::default(),
            factor: Default::default(),
            _factor: Default::default(),
            amount: Default::default(),
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
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "MonetaryComponent.type",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/price-component-type|5.0.0",
        )
        .with_description("The purpose for which an extended contact detail should be used.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("MonetaryComponent.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MonetaryComponent.extension", 0, None),
            rh_foundation::ElementCardinality::new("MonetaryComponent.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("MonetaryComponent.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MonetaryComponent.factor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("MonetaryComponent.amount", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for MonetaryComponent {
    fn resource_type(&self) -> &'static str {
        "MonetaryComponent"
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
        Some("http://hl7.org/fhir/StructureDefinition/MonetaryComponent")
    }
}
