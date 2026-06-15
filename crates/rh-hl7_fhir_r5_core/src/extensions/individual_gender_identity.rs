use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Individual Gender Identity
///
/// An individual's personal sense of being a man, woman, boy, girl, nonbinary, or something else. This represents an individual’s identity, ascertained by asking them what that identity is.   In the case where the gender identity is communicated by a third party, for example, if a spouse indicates the gender identity of their partner on an intake form, a Provenance resource can be used with a Provenance.target referring to the Patient, with a targetElement extension identifying the gender identity extension as the target element within the Patient resource.  When exchanging this concept, refer to the guidance in the [Gender Harmony Implementation Guide](http://hl7.org/xprod/ig/uv/gender-harmony/).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/individual-genderIdentity
/// - Version: 5.1.0-snapshot1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualGenderIdentity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for IndividualGenderIdentity {
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
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 1, None),
            rh_foundation::ElementCardinality::new("Extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.value[x]", 0, Some(0)),
        ]
    });

impl crate::validation::ValidatableResource for IndividualGenderIdentity {
    fn resource_type(&self) -> &'static str {
        "Extension"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/individual-genderIdentity")
    }
}
