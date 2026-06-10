use crate::bindings::concept_map_relationship::ConceptMapRelationship;
use crate::bindings::conceptmap_attribute_type::ConceptmapAttributeType;
use crate::bindings::conceptmap_property_type::ConceptmapPropertyType;
use crate::bindings::conceptmap_unmapped_mode::ConceptmapUnmappedMode;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ConceptMap
///
/// A statement of relationships from one set of concepts to one or more other concepts - either concepts in code systems, or data element/data element concepts, or classes in class models.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ConceptMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMap {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this concept map, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the concept map
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the concept map
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this concept map (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this concept map (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the concept map
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for concept map (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this concept map is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// When the ConceptMap was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the ConceptMap was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the ConceptMap is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the ConceptMap
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the ConceptMap
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the ConceptMap
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the ConceptMap
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Additional properties of the mapping
    pub property: Option<Vec<ConceptMapProperty>>,
    /// Definition of an additional attribute to act as a data source or target
    #[serde(rename = "additionalAttribute")]
    pub additional_attribute: Option<Vec<ConceptMapAdditionalattribute>>,
    /// The source value set that contains the concepts that are being mapped (uri)
    #[serde(rename = "sourceScopeUri")]
    pub source_scope_uri: Option<StringType>,
    /// The source value set that contains the concepts that are being mapped (canonical)
    #[serde(rename = "sourceScopeCanonical")]
    pub source_scope_canonical: Option<StringType>,
    /// The target value set which provides context for the mappings (uri)
    #[serde(rename = "targetScopeUri")]
    pub target_scope_uri: Option<StringType>,
    /// The target value set which provides context for the mappings (canonical)
    #[serde(rename = "targetScopeCanonical")]
    pub target_scope_canonical: Option<StringType>,
    /// Same source and target systems
    pub group: Option<Vec<ConceptMapGroup>>,
}
/// ConceptMap nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifies the property on the mappings, and when referred to in the $translate operation
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Formal identifier for the property
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// Why the property is defined, and/or what it conveys
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Coding | string | integer | boolean | dateTime | decimal | code
    #[serde(rename = "type")]
    pub type_: ConceptmapPropertyType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The CodeSystem from which code values come
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
}
/// ConceptMapGroup nested structure for the 'unmapped' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroupUnmapped {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// use-source-code | fixed | other-map
    pub mode: ConceptmapUnmappedMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Fixed code when mode = fixed
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Display for the code
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Fixed code set when mode = fixed
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: Option<ConceptMapRelationship>,
    /// Extension element for the 'relationship' primitive field. Contains metadata and extensions.
    pub _relationship: Option<Element>,
    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    #[serde(rename = "otherMap")]
    pub other_map: Option<StringType>,
    /// Extension element for the 'otherMap' primitive field. Contains metadata and extensions.
    #[serde(rename = "_otherMap")]
    pub _other_map: Option<Element>,
}
/// ConceptMapGroupElementTarget nested structure for the 'dependsOn' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroupElementTargetDependson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A reference to a mapping attribute defined in ConceptMap.additionalAttribute
    pub attribute: StringType,
    /// Extension element for the 'attribute' primitive field. Contains metadata and extensions.
    pub _attribute: Option<Element>,
    /// Value of the referenced data element (code)
    #[serde(rename = "valueCode")]
    pub value_code: Option<StringType>,
    /// Value of the referenced data element (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
    /// Value of the referenced data element (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Value of the referenced data element (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Value of the referenced data element (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    /// The mapping depends on a data element with a value from this value set
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
}
/// ConceptMap nested structure for the 'additionalAttribute' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapAdditionalattribute {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifies this additional attribute through this resource
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Formal identifier for the data element referred to in this attribte
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// Why the additional attribute is defined, and/or what the data element it refers to is
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// code | Coding | string | boolean | Quantity
    #[serde(rename = "type")]
    pub type_: ConceptmapAttributeType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// ConceptMap nested structure for the 'group' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Mappings for a concept from the source set
    pub element: Vec<ConceptMapGroupElement>,
    /// What to do when there is no mapping target for the source concept and ConceptMap.group.element.noMap is not true
    pub unmapped: Option<ConceptMapGroupUnmapped>,
    /// Source system where concepts to be mapped are defined
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// Target system that the concepts are to be mapped to
    pub target: Option<StringType>,
    /// Extension element for the 'target' primitive field. Contains metadata and extensions.
    pub _target: Option<Element>,
}
/// ConceptMapGroupElement nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroupElementTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code that identifies the target element
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Display for the code
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Identifies the set of target concepts
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: ConceptMapRelationship,
    /// Extension element for the 'relationship' primitive field. Contains metadata and extensions.
    pub _relationship: Option<Element>,
    /// Description of status/issues in mapping
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Other data elements that this mapping also produces
    pub product: Option<Vec<StringType>>,
}
/// ConceptMapGroupElementTarget nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroupElementTargetProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to ConceptMap.property.code
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Value of the property for this concept (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Value of the property for this concept (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Value of the property for this concept (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Value of the property for this concept (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Value of the property for this concept (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Value of the property for this concept (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Value of the property for this concept (code)
    #[serde(rename = "valueCode")]
    pub value_code: StringType,
}
/// ConceptMapGroup nested structure for the 'element' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMapGroupElement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifies element being mapped
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Display for the code
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Identifies the set of concepts being mapped
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// No mapping to a target concept for this source concept
    #[serde(rename = "noMap")]
    pub no_map: Option<BooleanType>,
    /// Extension element for the 'noMap' primitive field. Contains metadata and extensions.
    #[serde(rename = "_noMap")]
    pub _no_map: Option<Element>,
}

impl Default for ConceptMap {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            approval_date: Default::default(),
            _approval_date: Default::default(),
            last_review_date: Default::default(),
            _last_review_date: Default::default(),
            effective_period: Default::default(),
            topic: Default::default(),
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            related_artifact: Default::default(),
            property: Default::default(),
            additional_attribute: Default::default(),
            source_scope_uri: Default::default(),
            source_scope_canonical: Default::default(),
            target_scope_uri: Default::default(),
            target_scope_canonical: Default::default(),
            group: Default::default(),
        }
    }
}

impl Default for ConceptMapProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: StringType::default(),
            _code: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            system: Default::default(),
            _system: Default::default(),
        }
    }
}

impl Default for ConceptMapGroupUnmapped {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: Default::default(),
            _mode: Default::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
            relationship: Default::default(),
            _relationship: Default::default(),
            other_map: Default::default(),
            _other_map: Default::default(),
        }
    }
}

impl Default for ConceptMapGroupElementTargetDependson {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            attribute: Default::default(),
            _attribute: Default::default(),
            value_code: Default::default(),
            value_coding: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_quantity: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
        }
    }
}

impl Default for ConceptMapAdditionalattribute {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for ConceptMapGroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            element: Vec::new(),
            unmapped: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            target: Default::default(),
            _target: Default::default(),
        }
    }
}

impl Default for ConceptMapGroupElementTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
            relationship: Default::default(),
            _relationship: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            product: Default::default(),
        }
    }
}

impl Default for ConceptMapGroupElementTargetProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            value_coding: Default::default(),
            value_string: Default::default(),
            value_integer: Default::default(),
            value_boolean: Default::default(),
            value_date_time: Default::default(),
            value_decimal: Default::default(),
            value_code: Default::default(),
        }
    }
}

impl Default for ConceptMapGroupElement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
            no_map: Default::default(),
            _no_map: Default::default(),
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
    rh_foundation::Invariant::new("cmd-1", rh_foundation::Severity::Error, "If the map is source-is-broader-than-target or not-related-to, there SHALL be some comments, unless the status is 'draft'", "comment.exists() or (%resource.status = 'draft') or relationship.empty() or ((relationship != 'source-is-broader-than-target') and (relationship != 'not-related-to'))"),
    rh_foundation::Invariant::new("cmd-10", rh_foundation::Severity::Error, "If the mode is not 'other-map', otherMap is not allowed", "(mode != 'other-map') implies otherMap.empty()"),
    rh_foundation::Invariant::new("cmd-11", rh_foundation::Severity::Error, "If the property type is code, a system SHALL be specified", "type = 'code' implies system.exists()"),
    rh_foundation::Invariant::new("cmd-2", rh_foundation::Severity::Error, "If the mode is 'fixed', either a code or valueSet must be provided, but not both.", "(mode = 'fixed') implies ((code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists()))"),
    rh_foundation::Invariant::new("cmd-3", rh_foundation::Severity::Error, "If the mode is 'other-map', a url for the other map must be provided", "(mode = 'other-map') implies otherMap.exists()"),
    rh_foundation::Invariant::new("cmd-4", rh_foundation::Severity::Error, "If noMap is present, target SHALL NOT be present", "(noMap.exists() and noMap=true) implies target.empty()"),
    rh_foundation::Invariant::new("cmd-5", rh_foundation::Severity::Error, "Either code or valueSet SHALL be present but not both.", "(code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-6", rh_foundation::Severity::Error, "One of value[x] or valueSet must exist, but not both.", "(value.exists() and valueSet.empty()) or (value.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-7", rh_foundation::Severity::Error, "Either code or valueSet SHALL be present but not both.", "(code.exists() and valueSet.empty()) or (code.empty() and valueSet.exists())"),
    rh_foundation::Invariant::new("cmd-8", rh_foundation::Severity::Error, "If the mode is not 'fixed', code, display and valueSet are not allowed", "(mode != 'fixed') implies (code.empty() and display.empty() and valueSet.empty())"),
    rh_foundation::Invariant::new("cmd-9", rh_foundation::Severity::Error, "If the mode is not 'other-map', relationship must be provided", "(mode != 'other-map') implies relationship.exists()"),
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ConceptMap.additionalAttribute.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-attribute-type|5.0.0").with_description("The type of a mapping attribute value."),
    rh_foundation::ElementBinding::new("ConceptMap.group.element.target.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-map-relationship|5.0.0").with_description("The relationship between concepts."),
    rh_foundation::ElementBinding::new("ConceptMap.group.unmapped.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-unmapped-mode|5.0.0").with_description("Defines which action to take if there is no match in the group."),
    rh_foundation::ElementBinding::new("ConceptMap.group.unmapped.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-map-relationship|5.0.0").with_description("The default relationship value to apply between the source and target concepts when no concept mapping is specified."),
    rh_foundation::ElementBinding::new("ConceptMap.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("ConceptMap.property.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/conceptmap-property-type|5.0.0").with_description("The type of a property value."),
    rh_foundation::ElementBinding::new("ConceptMap.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ConceptMap.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.contained", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.extension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.contact", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.topic", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.author", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.editor", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.endorser", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.property", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.uri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.property.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.additionalAttribute", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.additionalAttribute.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.uri",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.additionalAttribute.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.sourceScope[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.targetScope[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.extension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.source", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.target", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element", 1, None),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.valueSet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.noMap", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.element.target", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.code",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.comment",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.attribute",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.dependsOn.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.element.target.product",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ConceptMap.group.unmapped.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.relationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ConceptMap.group.unmapped.otherMap",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ConceptMap {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for ConceptMap {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ConceptMap {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for ConceptMap {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for ConceptMap {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ConceptMap {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::concept_map::ConceptMapAccessors for ConceptMap {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_deref().unwrap_or(&[])
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_deref().unwrap_or(&[])
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn approval_date(&self) -> Option<DateType> {
        self.approval_date.clone()
    }
    fn last_review_date(&self) -> Option<DateType> {
        self.last_review_date.clone()
    }
    fn effective_period(&self) -> Option<Period> {
        self.effective_period.clone()
    }
    fn topic(&self) -> &[CodeableConcept] {
        self.topic.as_deref().unwrap_or(&[])
    }
    fn author(&self) -> &[ContactDetail] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_deref().unwrap_or(&[])
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_deref().unwrap_or(&[])
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_deref().unwrap_or(&[])
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[ConceptMapProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn additional_attribute(&self) -> &[ConceptMapAdditionalattribute] {
        self.additional_attribute.as_deref().unwrap_or(&[])
    }
    fn group(&self) -> &[ConceptMapGroup] {
        self.group.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::concept_map::ConceptMapMutators for ConceptMap {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = Some(value);
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = Some(value);
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = Some(value);
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .jurisdiction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_approval_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.approval_date = Some(value);
        resource
    }
    fn set_last_review_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_review_date = Some(value);
        resource
    }
    fn set_effective_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.effective_period = Some(value);
        resource
    }
    fn set_topic(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.topic = Some(value);
        resource
    }
    fn add_topic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.topic.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = Some(value);
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = Some(value);
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = Some(value);
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = Some(value);
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource
            .related_artifact
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_property(self, value: Vec<ConceptMapProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: ConceptMapProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_additional_attribute(self, value: Vec<ConceptMapAdditionalattribute>) -> Self {
        let mut resource = self.clone();
        resource.additional_attribute = Some(value);
        resource
    }
    fn add_additional_attribute(self, item: ConceptMapAdditionalattribute) -> Self {
        let mut resource = self.clone();
        resource
            .additional_attribute
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_group(self, value: Vec<ConceptMapGroup>) -> Self {
        let mut resource = self.clone();
        resource.group = Some(value);
        resource
    }
    fn add_group(self, item: ConceptMapGroup) -> Self {
        let mut resource = self.clone();
        resource.group.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::concept_map::ConceptMapExistence for ConceptMap {
    fn has_target_scope(&self) -> bool {
        self.target_scope_uri.is_some() || self.target_scope_canonical.is_some()
    }
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_source_scope(&self) -> bool {
        self.source_scope_uri.is_some() || self.source_scope_canonical.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        self.use_context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_jurisdiction(&self) -> bool {
        self.jurisdiction.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_approval_date(&self) -> bool {
        self.approval_date.is_some()
    }
    fn has_last_review_date(&self) -> bool {
        self.last_review_date.is_some()
    }
    fn has_effective_period(&self) -> bool {
        self.effective_period.is_some()
    }
    fn has_topic(&self) -> bool {
        self.topic.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_editor(&self) -> bool {
        self.editor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reviewer(&self) -> bool {
        self.reviewer.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_endorser(&self) -> bool {
        self.endorser.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_related_artifact(&self) -> bool {
        self.related_artifact
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_additional_attribute(&self) -> bool {
        self.additional_attribute
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_group(&self) -> bool {
        self.group.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ConceptMap {
    fn resource_type(&self) -> &'static str {
        "ConceptMap"
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
        Some("http://hl7.org/fhir/StructureDefinition/ConceptMap")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::concept_map::{
    ConceptMapAccessors, ConceptMapExistence, ConceptMapMutators,
};
