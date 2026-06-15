use crate::datatypes::extension::Extension;
use serde::{Deserialize, Serialize};
/// Version Specific Value
///
/// Provides an alternative value for the element carrying this extension that is the correct value for a particular range of FHIR versions other than the default value. This extension is found in contexts where a definition is applying to more than one version, usually defining extensions, and should only be used in context that clearly document how a cross-version definition is used. While there are no limitations to where this extension can be used, known uses are: StructureDefinition.context, ElementDefinition.type, ElementDefinition.additionalBinding and ...
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/version-specific-value
/// - Version: 5.1.0-snapshot1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSpecificValue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for VersionSpecificValue {
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Extension.extension.value[x]",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/major-fhir-version",
        )
        .with_description("1.0, 3.0, 4.0, 4.3, or 5.0")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Extension", 0, None),
            rh_foundation::ElementCardinality::new("Extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 3, None),
            rh_foundation::ElementCardinality::new("Extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.extension", 0, Some(0)),
            rh_foundation::ElementCardinality::new("Extension.extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.extension.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Extension.value[x]", 0, Some(0)),
        ]
    });

impl crate::validation::ValidatableResource for VersionSpecificValue {
    fn resource_type(&self) -> &'static str {
        "Extension"
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
        Some("http://hl7.org/fhir/StructureDefinition/version-specific-value")
    }
}
