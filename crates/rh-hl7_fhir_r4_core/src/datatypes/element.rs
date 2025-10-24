use crate::datatypes::extension::Extension;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Element
///
/// Base StructureDefinition for Element Type: Base definition for all elements in a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Element
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Unique id for inter-element referencing
    pub id: Option<StringType>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: Default::default(),
            extension: Default::default(),
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

impl crate::validation::ValidatableResource for Element {
    fn resource_type(&self) -> &'static str {
        "Element"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }
}
