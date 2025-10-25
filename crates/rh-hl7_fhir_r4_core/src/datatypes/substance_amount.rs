use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// SubstanceAmount
///
/// Base StructureDefinition for SubstanceAmount Type: Chemical substances are a single substance type whose primary defining element is the molecular structure. Chemical substances shall be defined on the basis of their complete covalent molecular structure; the presence of a salt (counter-ion) and/or solvates (water, alcohols) is also captured. Purity, grade, physical form or particle size are not taken into account in the definition of a chemical substance or in the assignment of a Substance ID.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubstanceAmount
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: SubstanceAmount
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceAmount {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field (Quantity)
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,
    /// Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field (Range)
    #[serde(rename = "amountRange")]
    pub amount_range: Option<Range>,
    /// Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field (string)
    #[serde(rename = "amountString")]
    pub amount_string: Option<StringType>,
    /// Most elements that require a quantitative value will also have a field called amount type. Amount type should always be specified because the actual value of the amount is often dependent on it. EXAMPLE: In capturing the actual relative amounts of substances or molecular fragments it is essential to indicate whether the amount refers to a mole ratio or weight ratio. For any given element an effort should be made to use same the amount type for all related definitional elements
    #[serde(rename = "amountType")]
    pub amount_type: Option<CodeableConcept>,
    /// A textual comment on a numeric value
    #[serde(rename = "amountText")]
    pub amount_text: Option<StringType>,
    /// Extension element for the 'amountText' primitive field. Contains metadata and extensions.
    #[serde(rename = "_amountText")]
    pub _amount_text: Option<Element>,
    /// Reference range of possible or expected values
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Element>,
}
/// SubstanceAmount nested structure for the 'referenceRange' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceAmountReferencerange {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Lower limit possible or expected
    #[serde(rename = "lowLimit")]
    pub low_limit: Option<Quantity>,
    /// Upper limit possible or expected
    #[serde(rename = "highLimit")]
    pub high_limit: Option<Quantity>,
}

impl Default for SubstanceAmount {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            amount_quantity: Default::default(),
            amount_range: Default::default(),
            amount_string: Default::default(),
            amount_type: Default::default(),
            amount_text: Default::default(),
            _amount_text: Default::default(),
            reference_range: Default::default(),
        }
    }
}

impl Default for SubstanceAmountReferencerange {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            low_limit: Default::default(),
            high_limit: Default::default(),
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
            rh_foundation::ElementCardinality::new("SubstanceAmount.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceAmount.extension", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceAmount.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SubstanceAmount.amount[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceAmount.amountType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceAmount.amountText", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceAmount.referenceRange", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubstanceAmount.referenceRange.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubstanceAmount.referenceRange.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceAmount.referenceRange.lowLimit",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubstanceAmount.referenceRange.highLimit",
                0,
                Some(1),
            ),
        ]
    });

impl crate::validation::ValidatableResource for SubstanceAmount {
    fn resource_type(&self) -> &'static str {
        "SubstanceAmount"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/SubstanceAmount")
    }
}
