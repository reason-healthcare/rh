use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// MoneyQuantity
///
/// An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/MoneyQuantity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyQuantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for MoneyQuantity {
    fn default() -> Self {
        Self {
            base: Quantity::default(),
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
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("mqty-1", rh_foundation::Severity::Error, "There SHALL be a code if there is a value and it SHALL be an expression of currency.  If system is present, it SHALL be ISO 4217 (system = \"urn:iso:std:iso:4217\" - currency).", "(code.exists() or value.empty()) and (system.empty() or system = 'urn:iso:std:iso:4217')").with_xpath("(f:code or not(f:value)) and (not(exists(f:system)) or f:system/@value='urn:iso:std:iso:4217')"),
    rh_foundation::Invariant::new("qty-3", rh_foundation::Severity::Error, "If a code for the unit is present, the system SHALL also be present", "code.empty() or system.exists()").with_xpath("not(exists(f:code)) or exists(f:system)"),
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
            "http://hl7.org/fhir/ValueSet/quantity-comparator|4.0.1",
        )
        .with_description("How the Quantity should be understood and represented.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Quantity", 0, None),
            rh_foundation::ElementCardinality::new("Quantity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.extension", 0, None),
            rh_foundation::ElementCardinality::new("Quantity.value", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.comparator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.unit", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Quantity.code", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for MoneyQuantity {
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
        Some("http://hl7.org/fhir/StructureDefinition/MoneyQuantity")
    }
}
