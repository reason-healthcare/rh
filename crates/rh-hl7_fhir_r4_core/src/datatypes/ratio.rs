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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("rat-1", rh_foundation::Severity::Error, "Numerator and denominator SHALL both be present, or both are absent. If both are absent, there SHALL be some extension present", "(numerator.empty() xor denominator.exists()) and (numerator.exists() or extension.exists())").with_xpath("(count(f:numerator) = count(f:denominator)) and ((count(f:numerator) > 0) or (count(f:extension) > 0))"),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Ratio.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ratio.extension", 0, None),
            rh_foundation::ElementCardinality::new("Ratio.numerator", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Ratio.denominator", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Ratio {
    fn resource_type(&self) -> &'static str {
        "Ratio"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Ratio")
    }
}
