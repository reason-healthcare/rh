use crate::bindings::map_group_type_mode::MapGroupTypeMode;
use crate::bindings::map_input_mode::MapInputMode;
use crate::bindings::map_model_mode::MapModelMode;
use crate::bindings::map_source_list_mode::MapSourceListMode;
use crate::bindings::map_target_list_mode::MapTargetListMode;
use crate::bindings::map_transform::MapTransform;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// StructureMap
///
/// A Map of relationships between 2 structures that can be used to transform data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureMap
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: StructureMap
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMap {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this structure map, represented as a URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the structure map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the structure map
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this structure map (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this structure map (human friendly)
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the structure map
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for structure map (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this structure map is defined
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
    /// Structure Definition used by this map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure: Vec<StructureMapStructure>,
    /// Other maps used by this map (canonical URLs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import: Vec<StringType>,
    /// Extension element for the 'import' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _import: Vec<Element>,
    /// Definition of the constant value used in the map rules
    #[serde(rename = "const")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub const_: Vec<StructureMapConst>,
    /// Named sections for reader convenience
    pub group: Vec<StructureMapGroup>,
}
/// StructureMap nested structure for the 'const' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapConst {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Constant name
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// FHIRPath exression - value of the constant
    pub value: Option<StringType>,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// StructureMap nested structure for the 'group' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Named instance provided when invoking the map
    pub input: Vec<StructureMapGroupInput>,
    /// Transform Rule from source to target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<StructureMapGroupRule>,
    /// Human-readable label
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Another group that this group adds rules to
    pub extends: Option<StringType>,
    /// Extension element for the 'extends' primitive field. Contains metadata and extensions.
    pub _extends: Option<Element>,
    /// types | type-and-types
    #[serde(rename = "typeMode")]
    pub type_mode: Option<MapGroupTypeMode>,
    /// Extension element for the 'typeMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_typeMode")]
    pub _type_mode: Option<Element>,
    /// Additional description/explanation for group
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// StructureMapGroup nested structure for the 'input' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupInput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name for this instance of data
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Type for this instance of data
    #[serde(rename = "type")]
    pub type_: Option<StringType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// source | target
    pub mode: MapInputMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Documentation for this instance of data
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// StructureMapGroup nested structure for the 'rule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of the rule for internal references
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Rules contained in this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<StringType>,
    /// Documentation for this instance of data
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// StructureMapGroupRule nested structure for the 'dependent' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRuleDependent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of a rule or group to apply
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Parameter to pass to the rule or group
    pub parameter: Vec<StringType>,
}
/// StructureMapGroupRule nested structure for the 'source' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRuleSource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type or variable this rule applies to
    pub context: StringType,
    /// Extension element for the 'context' primitive field. Contains metadata and extensions.
    pub _context: Option<Element>,
    /// Specified minimum cardinality
    pub min: Option<IntegerType>,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Specified maximum cardinality (number or *)
    pub max: Option<StringType>,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
    /// Rule only applies if source has this type
    #[serde(rename = "type")]
    pub type_: Option<StringType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Default value if no value exists
    #[serde(rename = "defaultValue")]
    pub default_value: Option<StringType>,
    /// Extension element for the 'defaultValue' primitive field. Contains metadata and extensions.
    #[serde(rename = "_defaultValue")]
    pub _default_value: Option<Element>,
    /// Optional field for this source
    pub element: Option<StringType>,
    /// Extension element for the 'element' primitive field. Contains metadata and extensions.
    pub _element: Option<Element>,
    /// first | not_first | last | not_last | only_one
    #[serde(rename = "listMode")]
    pub list_mode: Option<MapSourceListMode>,
    /// Extension element for the 'listMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_listMode")]
    pub _list_mode: Option<Element>,
    /// Named context for field, if a field is specified
    pub variable: Option<StringType>,
    /// Extension element for the 'variable' primitive field. Contains metadata and extensions.
    pub _variable: Option<Element>,
    /// FHIRPath expression  - must be true or the rule does not apply
    pub condition: Option<StringType>,
    /// Extension element for the 'condition' primitive field. Contains metadata and extensions.
    pub _condition: Option<Element>,
    /// FHIRPath expression  - must be true or the mapping engine throws an error instead of completing
    pub check: Option<StringType>,
    /// Extension element for the 'check' primitive field. Contains metadata and extensions.
    pub _check: Option<Element>,
    /// Message to put in log if source exists (FHIRPath)
    #[serde(rename = "logMessage")]
    pub log_message: Option<StringType>,
    /// Extension element for the 'logMessage' primitive field. Contains metadata and extensions.
    #[serde(rename = "_logMessage")]
    pub _log_message: Option<Element>,
}
/// StructureMapGroupRule nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRuleTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Variable this rule applies to
    pub context: Option<StringType>,
    /// Extension element for the 'context' primitive field. Contains metadata and extensions.
    pub _context: Option<Element>,
    /// Field to create in the context
    pub element: Option<StringType>,
    /// Extension element for the 'element' primitive field. Contains metadata and extensions.
    pub _element: Option<Element>,
    /// Named context for field, if desired, and a field is specified
    pub variable: Option<StringType>,
    /// Extension element for the 'variable' primitive field. Contains metadata and extensions.
    pub _variable: Option<Element>,
    /// first | share | last | single
    #[serde(rename = "listMode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_mode: Vec<MapTargetListMode>,
    /// Extension element for the 'listMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_listMode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _list_mode: Vec<Element>,
    /// Internal rule reference for shared list items
    #[serde(rename = "listRuleId")]
    pub list_rule_id: Option<StringType>,
    /// Extension element for the 'listRuleId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_listRuleId")]
    pub _list_rule_id: Option<Element>,
    /// create | copy +
    pub transform: Option<MapTransform>,
    /// Extension element for the 'transform' primitive field. Contains metadata and extensions.
    pub _transform: Option<Element>,
}
/// StructureMapGroupRuleTarget nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRuleTargetParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Parameter value - variable or literal (id)
    #[serde(rename = "valueId")]
    pub value_id: StringType,
    /// Parameter value - variable or literal (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Parameter value - variable or literal (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Parameter value - variable or literal (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Parameter value - variable or literal (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Parameter value - variable or literal (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Parameter value - variable or literal (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Parameter value - variable or literal (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
}
/// StructureMap nested structure for the 'structure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapStructure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Canonical reference to structure definition
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// source | queried | target | produced
    pub mode: MapModelMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Name for type in this map
    pub alias: Option<StringType>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    pub _alias: Option<Element>,
    /// Documentation on use of structure
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}

impl Default for StructureMap {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: StringType::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: StringType::default(),
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
            structure: Default::default(),
            import: Default::default(),
            _import: Default::default(),
            const_: Default::default(),
            group: Vec::new(),
        }
    }
}

impl Default for StructureMapConst {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for StructureMapGroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            input: Vec::new(),
            rule: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            extends: Default::default(),
            _extends: Default::default(),
            type_mode: Default::default(),
            _type_mode: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for StructureMapGroupInput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            mode: Default::default(),
            _mode: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for StructureMapGroupRule {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            rule: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for StructureMapGroupRuleDependent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            parameter: Default::default(),
        }
    }
}

impl Default for StructureMapGroupRuleSource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            context: Default::default(),
            _context: Default::default(),
            min: Default::default(),
            _min: Default::default(),
            max: Default::default(),
            _max: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            default_value: Default::default(),
            _default_value: Default::default(),
            element: Default::default(),
            _element: Default::default(),
            list_mode: Default::default(),
            _list_mode: Default::default(),
            variable: Default::default(),
            _variable: Default::default(),
            condition: Default::default(),
            _condition: Default::default(),
            check: Default::default(),
            _check: Default::default(),
            log_message: Default::default(),
            _log_message: Default::default(),
        }
    }
}

impl Default for StructureMapGroupRuleTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            context: Default::default(),
            _context: Default::default(),
            element: Default::default(),
            _element: Default::default(),
            variable: Default::default(),
            _variable: Default::default(),
            list_mode: Default::default(),
            _list_mode: Default::default(),
            list_rule_id: Default::default(),
            _list_rule_id: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
        }
    }
}

impl Default for StructureMapGroupRuleTargetParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_id: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_decimal: Default::default(),
            value_date: Default::default(),
            value_time: Default::default(),
            value_date_time: Default::default(),
        }
    }
}

impl Default for StructureMapStructure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            url: StringType::default(),
            _url: Default::default(),
            mode: MapModelMode::default(),
            _mode: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
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
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("smp-1", rh_foundation::Severity::Error, "Can only have an element if you have a context", "element.exists() implies context.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("StructureMap.group.input.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-input-mode|5.0.0").with_description("Mode for this instance of data."),
    rh_foundation::ElementBinding::new("StructureMap.group.rule.source.listMode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-source-list-mode|5.0.0").with_description("If field is a list, how to manage the source."),
    rh_foundation::ElementBinding::new("StructureMap.group.rule.target.listMode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-target-list-mode|5.0.0").with_description("If field is a list, how to manage the production."),
    rh_foundation::ElementBinding::new("StructureMap.group.rule.target.transform", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-transform|5.0.0").with_description("How data is copied/created."),
    rh_foundation::ElementBinding::new("StructureMap.group.typeMode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-group-type-mode|5.0.0").with_description("If this is the default rule set to apply for the source type, or this combination of types."),
    rh_foundation::ElementBinding::new("StructureMap.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("StructureMap.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
    rh_foundation::ElementBinding::new("StructureMap.structure.mode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/map-model-mode|5.0.0").with_description("How the referenced structure is used in this mapping."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("StructureMap.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.contained", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.extension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.identifier", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.contact", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.useContext", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.structure", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.structure.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.structure.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "StructureMap.structure.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("StructureMap.structure.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.structure.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.structure.alias", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "StructureMap.structure.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("StructureMap.import", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.const", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.const.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.const.extension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.const.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.const.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.const.value", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group", 1, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.extension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.extends", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.typeMode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.documentation", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.input", 1, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.input.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.input.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("StructureMap.group.input.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.input.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.input.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.input.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.source", 1, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.source.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.context",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.min",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.max",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.defaultValue",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.element",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.listMode",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.variable",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.condition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.check",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.source.logMessage",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.target", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.target.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.context",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.element",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.variable",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.listMode",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.listRuleId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.transform",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.parameter",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.parameter.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.parameter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.target.parameter.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.rule", 0, None),
            rh_foundation::ElementCardinality::new("StructureMap.group.rule.dependent", 0, None),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.dependent.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.dependent.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.dependent.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.dependent.name",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.dependent.parameter",
                1,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "StructureMap.group.rule.documentation",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for StructureMap {
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

impl crate::traits::resource::ResourceMutators for StructureMap {
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

impl crate::traits::resource::ResourceExistence for StructureMap {
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

impl crate::traits::domain_resource::DomainResourceAccessors for StructureMap {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for StructureMap {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for StructureMap {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::structure_map::StructureMapAccessors for StructureMap {
    fn url(&self) -> StringType {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> StringType {
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
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
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
    fn structure(&self) -> &[StructureMapStructure] {
        self.structure.as_slice()
    }
    fn import(&self) -> &[StringType] {
        self.import.as_slice()
    }
    fn const_(&self) -> &[StructureMapConst] {
        self.const_.as_slice()
    }
    fn group(&self) -> &[StructureMapGroup] {
        &self.group
    }
}

impl crate::traits::structure_map::StructureMapMutators for StructureMap {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = value;
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = value;
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
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
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
    fn set_structure(self, value: Vec<StructureMapStructure>) -> Self {
        let mut resource = self.clone();
        resource.structure = value;
        resource
    }
    fn add_structure(self, item: StructureMapStructure) -> Self {
        let mut resource = self.clone();
        resource.structure.push(item);
        resource
    }
    fn set_import(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.import = value;
        resource
    }
    fn add_import(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.import.push(item);
        resource
    }
    fn set_const_(self, value: Vec<StructureMapConst>) -> Self {
        let mut resource = self.clone();
        resource.const_ = value;
        resource
    }
    fn add_const_(self, item: StructureMapConst) -> Self {
        let mut resource = self.clone();
        resource.const_.push(item);
        resource
    }
    fn set_group(self, value: Vec<StructureMapGroup>) -> Self {
        let mut resource = self.clone();
        resource.group = value;
        resource
    }
    fn add_group(self, item: StructureMapGroup) -> Self {
        let mut resource = self.clone();
        resource.group.push(item);
        resource
    }
}

impl crate::traits::structure_map::StructureMapExistence for StructureMap {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        true
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
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
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
    fn has_structure(&self) -> bool {
        !self.structure.is_empty()
    }
    fn has_import(&self) -> bool {
        !self.import.is_empty()
    }
    fn has_const_(&self) -> bool {
        !self.const_.is_empty()
    }
    fn has_group(&self) -> bool {
        !self.group.is_empty()
    }
}

impl crate::validation::ValidatableResource for StructureMap {
    fn resource_type(&self) -> &'static str {
        "StructureMap"
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
        Some("http://hl7.org/fhir/StructureDefinition/StructureMap")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::structure_map::{
    StructureMapAccessors, StructureMapExistence, StructureMapMutators,
};
