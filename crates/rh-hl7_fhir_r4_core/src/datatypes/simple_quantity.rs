use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// SimpleQuantity
///
/// A fixed quantity (no comparator)
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SimpleQuantity
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Quantity
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleQuantity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for SimpleQuantity {
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
            rh_foundation::Invariant::new(
                "sqty-1",
                rh_foundation::Severity::Error,
                "The comparator is not used on a SimpleQuantity",
                "comparator.empty()",
            )
            .with_xpath("not(exists(f:comparator))"),
        ]
    });

impl crate::validation::ValidatableResource for SimpleQuantity {
    fn resource_type(&self) -> &'static str {
        "Quantity"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SimpleQuantity")
    }
}
