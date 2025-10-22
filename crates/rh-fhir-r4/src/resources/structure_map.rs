use crate::bindings::map_context_type::MapContextType;
use crate::bindings::map_group_type_mode::MapGroupTypeMode;
use crate::bindings::map_input_mode::MapInputMode;
use crate::bindings::map_model_mode::MapModelMode;
use crate::bindings::map_source_list_mode::MapSourceListMode;
use crate::bindings::map_target_list_mode::MapTargetListMode;
use crate::bindings::map_transform::MapTransform;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::address::Address;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::contact_point::ContactPoint;
use crate::datatypes::contributor::Contributor;
use crate::datatypes::count::Count;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::distance::Distance;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::human_name::HumanName;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::meta::Meta;
use crate::datatypes::money::Money;
use crate::datatypes::parameter_definition::ParameterDefinition;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::sampled_data::SampledData;
use crate::datatypes::signature::Signature;
use crate::datatypes::timing::Timing;
use crate::datatypes::trigger_definition::TriggerDefinition;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::base64binary::Base64BinaryType;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::integer::IntegerType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// StructureMap
///
/// A Map of relationships between 2 structures that can be used to transform data.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureMap
/// - Version: 4.0.1
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
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the structure map
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
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
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the structure map
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for structure map (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this structure map is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Structure Definition used by this map
    pub structure: Option<Vec<StructureMapStructure>>,
    /// Other maps used by this map (canonical URLs)
    pub import: Option<Vec<StringType>>,
    /// Extension element for the 'import' primitive field. Contains metadata and extensions.
    pub _import: Option<Element>,
    /// Named sections for reader convenience
    pub group: Vec<StructureMapGroup>,
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
/// StructureMapGroupRule nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRuleTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type or variable this rule applies to
    pub context: Option<StringType>,
    /// Extension element for the 'context' primitive field. Contains metadata and extensions.
    pub _context: Option<Element>,
    /// type | variable
    #[serde(rename = "contextType")]
    pub context_type: Option<MapContextType>,
    /// Extension element for the 'contextType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contextType")]
    pub _context_type: Option<Element>,
    /// Field to create in the context
    pub element: Option<StringType>,
    /// Extension element for the 'element' primitive field. Contains metadata and extensions.
    pub _element: Option<Element>,
    /// Named context for field, if desired, and a field is specified
    pub variable: Option<StringType>,
    /// Extension element for the 'variable' primitive field. Contains metadata and extensions.
    pub _variable: Option<Element>,
    /// first | share | last | collate
    #[serde(rename = "listMode")]
    pub list_mode: Option<Vec<MapTargetListMode>>,
    /// Extension element for the 'listMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_listMode")]
    pub _list_mode: Option<Element>,
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
    /// Default value if no value exists (base64Binary)
    #[serde(rename = "defaultValueBase64Binary")]
    pub default_value_base64_binary: Option<Base64BinaryType>,
    /// Default value if no value exists (boolean)
    #[serde(rename = "defaultValueBoolean")]
    pub default_value_boolean: Option<BooleanType>,
    /// Default value if no value exists (canonical)
    #[serde(rename = "defaultValueCanonical")]
    pub default_value_canonical: Option<StringType>,
    /// Default value if no value exists (code)
    #[serde(rename = "defaultValueCode")]
    pub default_value_code: Option<StringType>,
    /// Default value if no value exists (date)
    #[serde(rename = "defaultValueDate")]
    pub default_value_date: Option<StringType>,
    /// Default value if no value exists (dateTime)
    #[serde(rename = "defaultValueDateTime")]
    pub default_value_date_time: Option<DateTimeType>,
    /// Default value if no value exists (decimal)
    #[serde(rename = "defaultValueDecimal")]
    pub default_value_decimal: Option<DecimalType>,
    /// Default value if no value exists (id)
    #[serde(rename = "defaultValueId")]
    pub default_value_id: Option<StringType>,
    /// Default value if no value exists (instant)
    #[serde(rename = "defaultValueInstant")]
    pub default_value_instant: Option<InstantType>,
    /// Default value if no value exists (integer)
    #[serde(rename = "defaultValueInteger")]
    pub default_value_integer: Option<IntegerType>,
    /// Default value if no value exists (markdown)
    #[serde(rename = "defaultValueMarkdown")]
    pub default_value_markdown: Option<StringType>,
    /// Default value if no value exists (oid)
    #[serde(rename = "defaultValueOid")]
    pub default_value_oid: Option<StringType>,
    /// Default value if no value exists (positiveInt)
    #[serde(rename = "defaultValuePositiveInt")]
    pub default_value_positive_int: Option<PositiveIntType>,
    /// Default value if no value exists (string)
    #[serde(rename = "defaultValueString")]
    pub default_value_string: Option<StringType>,
    /// Default value if no value exists (time)
    #[serde(rename = "defaultValueTime")]
    pub default_value_time: Option<TimeType>,
    /// Default value if no value exists (unsignedInt)
    #[serde(rename = "defaultValueUnsignedInt")]
    pub default_value_unsigned_int: Option<UnsignedIntType>,
    /// Default value if no value exists (uri)
    #[serde(rename = "defaultValueUri")]
    pub default_value_uri: Option<StringType>,
    /// Default value if no value exists (url)
    #[serde(rename = "defaultValueUrl")]
    pub default_value_url: Option<StringType>,
    /// Default value if no value exists (uuid)
    #[serde(rename = "defaultValueUuid")]
    pub default_value_uuid: Option<StringType>,
    /// Default value if no value exists (Address)
    #[serde(rename = "defaultValueAddress")]
    pub default_value_address: Option<Address>,
    /// Default value if no value exists (Age)
    #[serde(rename = "defaultValueAge")]
    pub default_value_age: Option<Age>,
    /// Default value if no value exists (Annotation)
    #[serde(rename = "defaultValueAnnotation")]
    pub default_value_annotation: Option<Annotation>,
    /// Default value if no value exists (Attachment)
    #[serde(rename = "defaultValueAttachment")]
    pub default_value_attachment: Option<Attachment>,
    /// Default value if no value exists (CodeableConcept)
    #[serde(rename = "defaultValueCodeableConcept")]
    pub default_value_codeable_concept: Option<CodeableConcept>,
    /// Default value if no value exists (Coding)
    #[serde(rename = "defaultValueCoding")]
    pub default_value_coding: Option<Coding>,
    /// Default value if no value exists (ContactPoint)
    #[serde(rename = "defaultValueContactPoint")]
    pub default_value_contact_point: Option<ContactPoint>,
    /// Default value if no value exists (Count)
    #[serde(rename = "defaultValueCount")]
    pub default_value_count: Option<Count>,
    /// Default value if no value exists (Distance)
    #[serde(rename = "defaultValueDistance")]
    pub default_value_distance: Option<Distance>,
    /// Default value if no value exists (Duration)
    #[serde(rename = "defaultValueDuration")]
    pub default_value_duration: Option<Duration>,
    /// Default value if no value exists (HumanName)
    #[serde(rename = "defaultValueHumanName")]
    pub default_value_human_name: Option<HumanName>,
    /// Default value if no value exists (Identifier)
    #[serde(rename = "defaultValueIdentifier")]
    pub default_value_identifier: Option<Identifier>,
    /// Default value if no value exists (Money)
    #[serde(rename = "defaultValueMoney")]
    pub default_value_money: Option<Money>,
    /// Default value if no value exists (Period)
    #[serde(rename = "defaultValuePeriod")]
    pub default_value_period: Option<Period>,
    /// Default value if no value exists (Quantity)
    #[serde(rename = "defaultValueQuantity")]
    pub default_value_quantity: Option<Quantity>,
    /// Default value if no value exists (Range)
    #[serde(rename = "defaultValueRange")]
    pub default_value_range: Option<Range>,
    /// Default value if no value exists (Ratio)
    #[serde(rename = "defaultValueRatio")]
    pub default_value_ratio: Option<Ratio>,
    /// Default value if no value exists (Reference)
    #[serde(rename = "defaultValueReference")]
    pub default_value_reference: Option<Reference>,
    /// Default value if no value exists (SampledData)
    #[serde(rename = "defaultValueSampledData")]
    pub default_value_sampled_data: Option<SampledData>,
    /// Default value if no value exists (Signature)
    #[serde(rename = "defaultValueSignature")]
    pub default_value_signature: Option<Signature>,
    /// Default value if no value exists (Timing)
    #[serde(rename = "defaultValueTiming")]
    pub default_value_timing: Option<Timing>,
    /// Default value if no value exists (ContactDetail)
    #[serde(rename = "defaultValueContactDetail")]
    pub default_value_contact_detail: Option<ContactDetail>,
    /// Default value if no value exists (Contributor)
    #[serde(rename = "defaultValueContributor")]
    pub default_value_contributor: Option<Contributor>,
    /// Default value if no value exists (DataRequirement)
    #[serde(rename = "defaultValueDataRequirement")]
    pub default_value_data_requirement: Option<DataRequirement>,
    /// Default value if no value exists (Expression)
    #[serde(rename = "defaultValueExpression")]
    pub default_value_expression: Option<Expression>,
    /// Default value if no value exists (ParameterDefinition)
    #[serde(rename = "defaultValueParameterDefinition")]
    pub default_value_parameter_definition: Option<ParameterDefinition>,
    /// Default value if no value exists (RelatedArtifact)
    #[serde(rename = "defaultValueRelatedArtifact")]
    pub default_value_related_artifact: Option<RelatedArtifact>,
    /// Default value if no value exists (TriggerDefinition)
    #[serde(rename = "defaultValueTriggerDefinition")]
    pub default_value_trigger_definition: Option<TriggerDefinition>,
    /// Default value if no value exists (UsageContext)
    #[serde(rename = "defaultValueUsageContext")]
    pub default_value_usage_context: Option<UsageContext>,
    /// Default value if no value exists (Dosage)
    #[serde(rename = "defaultValueDosage")]
    pub default_value_dosage: Option<Dosage>,
    /// Default value if no value exists (Meta)
    #[serde(rename = "defaultValueMeta")]
    pub default_value_meta: Option<Meta>,
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
}
/// StructureMapGroup nested structure for the 'rule' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroupRule {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of the rule for internal references
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Rules contained in this rule
    pub rule: Option<Vec<StringType>>,
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
    /// Variable to pass to the rule or group
    pub variable: Vec<StringType>,
    /// Extension element for the 'variable' primitive field. Contains metadata and extensions.
    pub _variable: Option<Element>,
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
/// StructureMap nested structure for the 'group' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureMapGroup {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Transform Rule from source to target
    pub rule: Vec<StructureMapGroupRule>,
    /// Named instance provided when invoking the map
    pub input: Vec<StructureMapGroupInput>,
    /// Human-readable label
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Another group that this group adds rules to
    pub extends: Option<StringType>,
    /// Extension element for the 'extends' primitive field. Contains metadata and extensions.
    pub _extends: Option<Element>,
    /// none | types | type-and-types
    #[serde(rename = "typeMode")]
    pub type_mode: MapGroupTypeMode,
    /// Extension element for the 'typeMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_typeMode")]
    pub _type_mode: Option<Element>,
    /// Additional description/explanation for group
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
            structure: Default::default(),
            import: Default::default(),
            _import: Default::default(),
            group: Vec::new(),
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

impl Default for StructureMapGroupRuleTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            context: Default::default(),
            _context: Default::default(),
            context_type: Default::default(),
            _context_type: Default::default(),
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
            default_value_base64_binary: Default::default(),
            default_value_boolean: Default::default(),
            default_value_canonical: Default::default(),
            default_value_code: Default::default(),
            default_value_date: Default::default(),
            default_value_date_time: Default::default(),
            default_value_decimal: Default::default(),
            default_value_id: Default::default(),
            default_value_instant: Default::default(),
            default_value_integer: Default::default(),
            default_value_markdown: Default::default(),
            default_value_oid: Default::default(),
            default_value_positive_int: Default::default(),
            default_value_string: Default::default(),
            default_value_time: Default::default(),
            default_value_unsigned_int: Default::default(),
            default_value_uri: Default::default(),
            default_value_url: Default::default(),
            default_value_uuid: Default::default(),
            default_value_address: Default::default(),
            default_value_age: Default::default(),
            default_value_annotation: Default::default(),
            default_value_attachment: Default::default(),
            default_value_codeable_concept: Default::default(),
            default_value_coding: Default::default(),
            default_value_contact_point: Default::default(),
            default_value_count: Default::default(),
            default_value_distance: Default::default(),
            default_value_duration: Default::default(),
            default_value_human_name: Default::default(),
            default_value_identifier: Default::default(),
            default_value_money: Default::default(),
            default_value_period: Default::default(),
            default_value_quantity: Default::default(),
            default_value_range: Default::default(),
            default_value_ratio: Default::default(),
            default_value_reference: Default::default(),
            default_value_sampled_data: Default::default(),
            default_value_signature: Default::default(),
            default_value_timing: Default::default(),
            default_value_contact_detail: Default::default(),
            default_value_contributor: Default::default(),
            default_value_data_requirement: Default::default(),
            default_value_expression: Default::default(),
            default_value_parameter_definition: Default::default(),
            default_value_related_artifact: Default::default(),
            default_value_trigger_definition: Default::default(),
            default_value_usage_context: Default::default(),
            default_value_dosage: Default::default(),
            default_value_meta: Default::default(),
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

impl Default for StructureMapGroupRuleTargetParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_id: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_decimal: Default::default(),
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
            variable: Default::default(),
            _variable: Default::default(),
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

impl Default for StructureMapGroup {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            rule: Vec::new(),
            input: Vec::new(),
            name: StringType::default(),
            _name: Default::default(),
            extends: Default::default(),
            _extends: Default::default(),
            type_mode: MapGroupTypeMode::default(),
            _type_mode: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

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
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
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

impl crate::traits::domain_resource::DomainResourceExistence for StructureMap {
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

impl crate::traits::structure_map::StructureMapAccessors for StructureMap {
    fn url(&self) -> StringType {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
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
    fn structure(&self) -> &[StructureMapStructure] {
        self.structure.as_deref().unwrap_or(&[])
    }
    fn import(&self) -> &[StringType] {
        self.import.as_deref().unwrap_or(&[])
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
    fn set_structure(self, value: Vec<StructureMapStructure>) -> Self {
        let mut resource = self.clone();
        resource.structure = Some(value);
        resource
    }
    fn add_structure(self, item: StructureMapStructure) -> Self {
        let mut resource = self.clone();
        resource.structure.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_import(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.import = Some(value);
        resource
    }
    fn add_import(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.import.get_or_insert_with(Vec::new).push(item);
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
    fn has_url(&self) -> bool {
        true
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_structure(&self) -> bool {
        self.structure.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_import(&self) -> bool {
        self.import.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group(&self) -> bool {
        !self.group.is_empty()
    }
}
