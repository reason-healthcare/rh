use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::quantity::Quantity;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// ProdCharacteristic
///
/// Base StructureDefinition for ProdCharacteristic Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ProdCharacteristic
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ProdCharacteristic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProdCharacteristic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Where applicable, the height can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    pub height: Option<Quantity>,
    /// Where applicable, the width can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    pub width: Option<Quantity>,
    /// Where applicable, the depth can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    pub depth: Option<Quantity>,
    /// Where applicable, the weight can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    pub weight: Option<Quantity>,
    /// Where applicable, the nominal volume can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    #[serde(rename = "nominalVolume")]
    pub nominal_volume: Option<Quantity>,
    /// Where applicable, the external diameter can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    #[serde(rename = "externalDiameter")]
    pub external_diameter: Option<Quantity>,
    /// Where applicable, the shape can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub shape: Option<StringType>,
    /// Extension element for the 'shape' primitive field. Contains metadata and extensions.
    pub _shape: Option<Element>,
    /// Where applicable, the color can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub color: Option<Vec<StringType>>,
    /// Extension element for the 'color' primitive field. Contains metadata and extensions.
    pub _color: Option<Element>,
    /// Where applicable, the imprint can be specified as text
    pub imprint: Option<Vec<StringType>>,
    /// Extension element for the 'imprint' primitive field. Contains metadata and extensions.
    pub _imprint: Option<Element>,
    /// Where applicable, the image can be provided The format of the image attachment shall be specified by regional implementations
    pub image: Option<Vec<Attachment>>,
    /// Where applicable, the scoring can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub scoring: Option<CodeableConcept>,
}

impl Default for ProdCharacteristic {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            height: Default::default(),
            width: Default::default(),
            depth: Default::default(),
            weight: Default::default(),
            nominal_volume: Default::default(),
            external_diameter: Default::default(),
            shape: Default::default(),
            _shape: Default::default(),
            color: Default::default(),
            _color: Default::default(),
            imprint: Default::default(),
            _imprint: Default::default(),
            image: Default::default(),
            scoring: Default::default(),
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
            rh_foundation::ElementCardinality::new("ProdCharacteristic.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.extension", 0, None),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.height", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.width", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.depth", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.weight", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.nominalVolume", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ProdCharacteristic.externalDiameter",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.shape", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.color", 0, None),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.imprint", 0, None),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.image", 0, None),
            rh_foundation::ElementCardinality::new("ProdCharacteristic.scoring", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ProdCharacteristic {
    fn resource_type(&self) -> &'static str {
        "ProdCharacteristic"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ProdCharacteristic")
    }
}
