use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use serde::{Deserialize, Serialize};
/// UsageContext
///
/// Base StructureDefinition for UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/UsageContext
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: UsageContext
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Element,
    /// Type of context being specified
    ///
    /// Binding: extensible (A code that specifies a type of context being specified by a usage context.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/usage-context-type
    pub code: Coding,
    /// Value that defines the context (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Value that defines the context (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value that defines the context (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value that defines the context (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
}

impl Default for UsageContext {
    fn default() -> Self {
        Self {
            base: Element::default(),
            code: Coding::default(),
            value_codeable_concept: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_reference: Default::default(),
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

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("UsageContext.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("UsageContext.extension", 0, None),
            rh_foundation::ElementCardinality::new("UsageContext.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("UsageContext.value[x]", 1, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for UsageContext {
    fn resource_type(&self) -> &'static str {
        "UsageContext"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/UsageContext")
    }
}
