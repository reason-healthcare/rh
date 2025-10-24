use crate::datatypes::quantity::Quantity;
use serde::{Deserialize, Serialize};
/// Count
///
/// Base StructureDefinition for Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Count
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Count
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Count {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Quantity,
}

impl Default for Count {
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
    rh_foundation::Invariant::new("cnt-3", rh_foundation::Severity::Error, "There SHALL be a code with a value of \"1\" if there is a value. If system is present, it SHALL be UCUM.  If present, the value SHALL be a whole number.", "(code.exists() or value.empty()) and (system.empty() or system = %ucum) and (code.empty() or code = '1') and (value.empty() or value.hasValue().not() or value.toString().contains('.').not())").with_xpath("(f:code or not(f:value)) and (not(exists(f:system)) or (f:system/@value='http://unitsofmeasure.org' and f:code/@value='1')) and not(contains(f:value/@value, '.'))"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("qty-3", rh_foundation::Severity::Error, "If a code for the unit is present, the system SHALL also be present", "code.empty() or system.exists()").with_xpath("not(exists(f:code)) or exists(f:system)"),
]
    });

impl crate::validation::ValidatableResource for Count {
    fn resource_type(&self) -> &'static str {
        "Count"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Count")
    }
}
