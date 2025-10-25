use crate::bindings::identifier_use::IdentifierUse;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Identifier
///
/// Base StructureDefinition for Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Identifier
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Identifier
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// usual | official | temp | secondary | old (If known)
    #[serde(rename = "use")]
    pub use_: Option<IdentifierUse>,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Description of identifier
    ///
    /// Binding: extensible (A coded type for an identifier that can be used to determine which identifier to use for a specific purpose.)
    ///
    /// Available values:
    /// - `DL`
    /// - `PPN`
    /// - `BRN`
    /// - `MR`
    /// - `MCN`
    /// - `EN`
    /// - `TAX`
    /// - `NIIP`
    /// - `PRN`
    /// - `MD`
    /// - ... and 8 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The namespace for the identifier value
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// The value that is unique
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Time period when id is/was valid for use
    pub period: Option<Period>,
    /// Organization that issued id (may be just text)
    pub assigner: Option<Box<Reference>>,
}
/// validDate
///
/// Indicates a date on which this identifier value was deemed to apply to this instance.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/identifier-validDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierValidDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            base: Element::default(),
            use_: Default::default(),
            _use: Default::default(),
            type_: Default::default(),
            system: Default::default(),
            _system: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            period: Default::default(),
            assigner: Default::default(),
        }
    }
}

impl Default for IdentifierValidDate {
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
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Identifier.use",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/identifier-use|4.0.1",
        )
        .with_description("Identifies the purpose for this identifier, if known .")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Identifier.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.extension", 0, None),
            rh_foundation::ElementCardinality::new("Identifier.use", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.value", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Identifier.assigner", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for Identifier {
    fn resource_type(&self) -> &'static str {
        "Identifier"
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
        Some("http://hl7.org/fhir/StructureDefinition/Identifier")
    }
}
