use crate::bindings::binding_strength::BindingStrength;
use crate::bindings::constraint_severity::ConstraintSeverity;
use crate::bindings::discriminator_type::DiscriminatorType;
use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::property_representation::PropertyRepresentation;
use crate::bindings::reference_version_rules::ReferenceVersionRules;
use crate::bindings::resource_aggregation_mode::ResourceAggregationMode;
use crate::bindings::resource_slicing_rules::ResourceSlicingRules;
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
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::instant::InstantType;
use crate::primitives::integer::IntegerType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::primitives::time::TimeType;
use crate::primitives::unsigned_int::UnsignedIntType;
use serde::{Deserialize, Serialize};
/// ElementDefinition
///
/// Base StructureDefinition for ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: ElementDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/BackboneElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Path of the element in the hierarchy of elements
    pub path: StringType,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    pub representation: Option<Vec<PropertyRepresentation>>,
    /// Extension element for the 'representation' primitive field. Contains metadata and extensions.
    pub _representation: Option<Element>,
    /// Name for this particular element (in a set of slices)
    #[serde(rename = "sliceName")]
    pub slice_name: Option<StringType>,
    /// Extension element for the 'sliceName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sliceName")]
    pub _slice_name: Option<Element>,
    /// If this slice definition constrains an inherited slice definition (or not)
    #[serde(rename = "sliceIsConstraining")]
    pub slice_is_constraining: Option<BooleanType>,
    /// Extension element for the 'sliceIsConstraining' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sliceIsConstraining")]
    pub _slice_is_constraining: Option<Element>,
    /// Name for element to display with or prompt for element
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Corresponding codes in terminologies
    ///
    /// Binding: example (Codes that indicate the meaning of a data element.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub code: Option<Vec<Coding>>,
    /// This element is sliced - slices follow
    pub slicing: Option<Element>,
    /// Concise definition for space-constrained presentation
    pub short: Option<StringType>,
    /// Extension element for the 'short' primitive field. Contains metadata and extensions.
    pub _short: Option<Element>,
    /// Full formal definition as narrative text
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Comments about the use of this element
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Why this resource has been created
    pub requirements: Option<StringType>,
    /// Extension element for the 'requirements' primitive field. Contains metadata and extensions.
    pub _requirements: Option<Element>,
    /// Other names
    pub alias: Option<Vec<StringType>>,
    /// Extension element for the 'alias' primitive field. Contains metadata and extensions.
    pub _alias: Option<Element>,
    /// Minimum Cardinality
    pub min: Option<UnsignedIntType>,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Maximum Cardinality (a number or *)
    pub max: Option<StringType>,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
    /// Base definition information for tools
    #[serde(rename = "base")]
    pub base_definition: Option<Element>,
    /// Reference to definition of content for the element
    #[serde(rename = "contentReference")]
    pub content_reference: Option<StringType>,
    /// Extension element for the 'contentReference' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentReference")]
    pub _content_reference: Option<Element>,
    /// Data type and Profile for this element
    #[serde(rename = "type")]
    pub type_: Option<Vec<Element>>,
    /// Specified value if missing from instance (base64Binary)
    #[serde(rename = "defaultValueBase64Binary")]
    pub default_value_base64_binary: Option<Base64BinaryType>,
    /// Specified value if missing from instance (boolean)
    #[serde(rename = "defaultValueBoolean")]
    pub default_value_boolean: Option<BooleanType>,
    /// Specified value if missing from instance (canonical)
    #[serde(rename = "defaultValueCanonical")]
    pub default_value_canonical: Option<StringType>,
    /// Specified value if missing from instance (code)
    #[serde(rename = "defaultValueCode")]
    pub default_value_code: Option<StringType>,
    /// Specified value if missing from instance (date)
    #[serde(rename = "defaultValueDate")]
    pub default_value_date: Option<DateType>,
    /// Specified value if missing from instance (dateTime)
    #[serde(rename = "defaultValueDateTime")]
    pub default_value_date_time: Option<DateTimeType>,
    /// Specified value if missing from instance (decimal)
    #[serde(rename = "defaultValueDecimal")]
    pub default_value_decimal: Option<DecimalType>,
    /// Specified value if missing from instance (id)
    #[serde(rename = "defaultValueId")]
    pub default_value_id: Option<StringType>,
    /// Specified value if missing from instance (instant)
    #[serde(rename = "defaultValueInstant")]
    pub default_value_instant: Option<InstantType>,
    /// Specified value if missing from instance (integer)
    #[serde(rename = "defaultValueInteger")]
    pub default_value_integer: Option<IntegerType>,
    /// Specified value if missing from instance (markdown)
    #[serde(rename = "defaultValueMarkdown")]
    pub default_value_markdown: Option<StringType>,
    /// Specified value if missing from instance (oid)
    #[serde(rename = "defaultValueOid")]
    pub default_value_oid: Option<StringType>,
    /// Specified value if missing from instance (positiveInt)
    #[serde(rename = "defaultValuePositiveInt")]
    pub default_value_positive_int: Option<PositiveIntType>,
    /// Specified value if missing from instance (string)
    #[serde(rename = "defaultValueString")]
    pub default_value_string: Option<StringType>,
    /// Specified value if missing from instance (time)
    #[serde(rename = "defaultValueTime")]
    pub default_value_time: Option<TimeType>,
    /// Specified value if missing from instance (unsignedInt)
    #[serde(rename = "defaultValueUnsignedInt")]
    pub default_value_unsigned_int: Option<UnsignedIntType>,
    /// Specified value if missing from instance (uri)
    #[serde(rename = "defaultValueUri")]
    pub default_value_uri: Option<StringType>,
    /// Specified value if missing from instance (url)
    #[serde(rename = "defaultValueUrl")]
    pub default_value_url: Option<StringType>,
    /// Specified value if missing from instance (uuid)
    #[serde(rename = "defaultValueUuid")]
    pub default_value_uuid: Option<StringType>,
    /// Specified value if missing from instance (Address)
    #[serde(rename = "defaultValueAddress")]
    pub default_value_address: Option<Address>,
    /// Specified value if missing from instance (Age)
    #[serde(rename = "defaultValueAge")]
    pub default_value_age: Option<Age>,
    /// Specified value if missing from instance (Annotation)
    #[serde(rename = "defaultValueAnnotation")]
    pub default_value_annotation: Option<Annotation>,
    /// Specified value if missing from instance (Attachment)
    #[serde(rename = "defaultValueAttachment")]
    pub default_value_attachment: Option<Attachment>,
    /// Specified value if missing from instance (CodeableConcept)
    #[serde(rename = "defaultValueCodeableConcept")]
    pub default_value_codeable_concept: Option<CodeableConcept>,
    /// Specified value if missing from instance (Coding)
    #[serde(rename = "defaultValueCoding")]
    pub default_value_coding: Option<Coding>,
    /// Specified value if missing from instance (ContactPoint)
    #[serde(rename = "defaultValueContactPoint")]
    pub default_value_contact_point: Option<ContactPoint>,
    /// Specified value if missing from instance (Count)
    #[serde(rename = "defaultValueCount")]
    pub default_value_count: Option<Count>,
    /// Specified value if missing from instance (Distance)
    #[serde(rename = "defaultValueDistance")]
    pub default_value_distance: Option<Distance>,
    /// Specified value if missing from instance (Duration)
    #[serde(rename = "defaultValueDuration")]
    pub default_value_duration: Option<Duration>,
    /// Specified value if missing from instance (HumanName)
    #[serde(rename = "defaultValueHumanName")]
    pub default_value_human_name: Option<HumanName>,
    /// Specified value if missing from instance (Identifier)
    #[serde(rename = "defaultValueIdentifier")]
    pub default_value_identifier: Option<Identifier>,
    /// Specified value if missing from instance (Money)
    #[serde(rename = "defaultValueMoney")]
    pub default_value_money: Option<Money>,
    /// Specified value if missing from instance (Period)
    #[serde(rename = "defaultValuePeriod")]
    pub default_value_period: Option<Period>,
    /// Specified value if missing from instance (Quantity)
    #[serde(rename = "defaultValueQuantity")]
    pub default_value_quantity: Option<Quantity>,
    /// Specified value if missing from instance (Range)
    #[serde(rename = "defaultValueRange")]
    pub default_value_range: Option<Range>,
    /// Specified value if missing from instance (Ratio)
    #[serde(rename = "defaultValueRatio")]
    pub default_value_ratio: Option<Ratio>,
    /// Specified value if missing from instance (Reference)
    #[serde(rename = "defaultValueReference")]
    pub default_value_reference: Option<Reference>,
    /// Specified value if missing from instance (SampledData)
    #[serde(rename = "defaultValueSampledData")]
    pub default_value_sampled_data: Option<SampledData>,
    /// Specified value if missing from instance (Signature)
    #[serde(rename = "defaultValueSignature")]
    pub default_value_signature: Option<Signature>,
    /// Specified value if missing from instance (Timing)
    #[serde(rename = "defaultValueTiming")]
    pub default_value_timing: Option<Timing>,
    /// Specified value if missing from instance (ContactDetail)
    #[serde(rename = "defaultValueContactDetail")]
    pub default_value_contact_detail: Option<ContactDetail>,
    /// Specified value if missing from instance (Contributor)
    #[serde(rename = "defaultValueContributor")]
    pub default_value_contributor: Option<Contributor>,
    /// Specified value if missing from instance (DataRequirement)
    #[serde(rename = "defaultValueDataRequirement")]
    pub default_value_data_requirement: Option<DataRequirement>,
    /// Specified value if missing from instance (Expression)
    #[serde(rename = "defaultValueExpression")]
    pub default_value_expression: Option<Expression>,
    /// Specified value if missing from instance (ParameterDefinition)
    #[serde(rename = "defaultValueParameterDefinition")]
    pub default_value_parameter_definition: Option<ParameterDefinition>,
    /// Specified value if missing from instance (RelatedArtifact)
    #[serde(rename = "defaultValueRelatedArtifact")]
    pub default_value_related_artifact: Option<RelatedArtifact>,
    /// Specified value if missing from instance (TriggerDefinition)
    #[serde(rename = "defaultValueTriggerDefinition")]
    pub default_value_trigger_definition: Option<TriggerDefinition>,
    /// Specified value if missing from instance (UsageContext)
    #[serde(rename = "defaultValueUsageContext")]
    pub default_value_usage_context: Option<UsageContext>,
    /// Specified value if missing from instance (Dosage)
    #[serde(rename = "defaultValueDosage")]
    pub default_value_dosage: Option<Dosage>,
    /// Specified value if missing from instance (Meta)
    #[serde(rename = "defaultValueMeta")]
    pub default_value_meta: Option<Meta>,
    /// Implicit meaning when this element is missing
    #[serde(rename = "meaningWhenMissing")]
    pub meaning_when_missing: Option<StringType>,
    /// Extension element for the 'meaningWhenMissing' primitive field. Contains metadata and extensions.
    #[serde(rename = "_meaningWhenMissing")]
    pub _meaning_when_missing: Option<Element>,
    /// What the order of the elements means
    #[serde(rename = "orderMeaning")]
    pub order_meaning: Option<StringType>,
    /// Extension element for the 'orderMeaning' primitive field. Contains metadata and extensions.
    #[serde(rename = "_orderMeaning")]
    pub _order_meaning: Option<Element>,
    /// Value must be exactly this (base64Binary)
    #[serde(rename = "fixedBase64Binary")]
    pub fixed_base64_binary: Option<Base64BinaryType>,
    /// Value must be exactly this (boolean)
    #[serde(rename = "fixedBoolean")]
    pub fixed_boolean: Option<BooleanType>,
    /// Value must be exactly this (canonical)
    #[serde(rename = "fixedCanonical")]
    pub fixed_canonical: Option<StringType>,
    /// Value must be exactly this (code)
    #[serde(rename = "fixedCode")]
    pub fixed_code: Option<StringType>,
    /// Value must be exactly this (date)
    #[serde(rename = "fixedDate")]
    pub fixed_date: Option<DateType>,
    /// Value must be exactly this (dateTime)
    #[serde(rename = "fixedDateTime")]
    pub fixed_date_time: Option<DateTimeType>,
    /// Value must be exactly this (decimal)
    #[serde(rename = "fixedDecimal")]
    pub fixed_decimal: Option<DecimalType>,
    /// Value must be exactly this (id)
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<StringType>,
    /// Value must be exactly this (instant)
    #[serde(rename = "fixedInstant")]
    pub fixed_instant: Option<InstantType>,
    /// Value must be exactly this (integer)
    #[serde(rename = "fixedInteger")]
    pub fixed_integer: Option<IntegerType>,
    /// Value must be exactly this (markdown)
    #[serde(rename = "fixedMarkdown")]
    pub fixed_markdown: Option<StringType>,
    /// Value must be exactly this (oid)
    #[serde(rename = "fixedOid")]
    pub fixed_oid: Option<StringType>,
    /// Value must be exactly this (positiveInt)
    #[serde(rename = "fixedPositiveInt")]
    pub fixed_positive_int: Option<PositiveIntType>,
    /// Value must be exactly this (string)
    #[serde(rename = "fixedString")]
    pub fixed_string: Option<StringType>,
    /// Value must be exactly this (time)
    #[serde(rename = "fixedTime")]
    pub fixed_time: Option<TimeType>,
    /// Value must be exactly this (unsignedInt)
    #[serde(rename = "fixedUnsignedInt")]
    pub fixed_unsigned_int: Option<UnsignedIntType>,
    /// Value must be exactly this (uri)
    #[serde(rename = "fixedUri")]
    pub fixed_uri: Option<StringType>,
    /// Value must be exactly this (url)
    #[serde(rename = "fixedUrl")]
    pub fixed_url: Option<StringType>,
    /// Value must be exactly this (uuid)
    #[serde(rename = "fixedUuid")]
    pub fixed_uuid: Option<StringType>,
    /// Value must be exactly this (Address)
    #[serde(rename = "fixedAddress")]
    pub fixed_address: Option<Address>,
    /// Value must be exactly this (Age)
    #[serde(rename = "fixedAge")]
    pub fixed_age: Option<Age>,
    /// Value must be exactly this (Annotation)
    #[serde(rename = "fixedAnnotation")]
    pub fixed_annotation: Option<Annotation>,
    /// Value must be exactly this (Attachment)
    #[serde(rename = "fixedAttachment")]
    pub fixed_attachment: Option<Attachment>,
    /// Value must be exactly this (CodeableConcept)
    #[serde(rename = "fixedCodeableConcept")]
    pub fixed_codeable_concept: Option<CodeableConcept>,
    /// Value must be exactly this (Coding)
    #[serde(rename = "fixedCoding")]
    pub fixed_coding: Option<Coding>,
    /// Value must be exactly this (ContactPoint)
    #[serde(rename = "fixedContactPoint")]
    pub fixed_contact_point: Option<ContactPoint>,
    /// Value must be exactly this (Count)
    #[serde(rename = "fixedCount")]
    pub fixed_count: Option<Count>,
    /// Value must be exactly this (Distance)
    #[serde(rename = "fixedDistance")]
    pub fixed_distance: Option<Distance>,
    /// Value must be exactly this (Duration)
    #[serde(rename = "fixedDuration")]
    pub fixed_duration: Option<Duration>,
    /// Value must be exactly this (HumanName)
    #[serde(rename = "fixedHumanName")]
    pub fixed_human_name: Option<HumanName>,
    /// Value must be exactly this (Identifier)
    #[serde(rename = "fixedIdentifier")]
    pub fixed_identifier: Option<Identifier>,
    /// Value must be exactly this (Money)
    #[serde(rename = "fixedMoney")]
    pub fixed_money: Option<Money>,
    /// Value must be exactly this (Period)
    #[serde(rename = "fixedPeriod")]
    pub fixed_period: Option<Period>,
    /// Value must be exactly this (Quantity)
    #[serde(rename = "fixedQuantity")]
    pub fixed_quantity: Option<Quantity>,
    /// Value must be exactly this (Range)
    #[serde(rename = "fixedRange")]
    pub fixed_range: Option<Range>,
    /// Value must be exactly this (Ratio)
    #[serde(rename = "fixedRatio")]
    pub fixed_ratio: Option<Ratio>,
    /// Value must be exactly this (Reference)
    #[serde(rename = "fixedReference")]
    pub fixed_reference: Option<Reference>,
    /// Value must be exactly this (SampledData)
    #[serde(rename = "fixedSampledData")]
    pub fixed_sampled_data: Option<SampledData>,
    /// Value must be exactly this (Signature)
    #[serde(rename = "fixedSignature")]
    pub fixed_signature: Option<Signature>,
    /// Value must be exactly this (Timing)
    #[serde(rename = "fixedTiming")]
    pub fixed_timing: Option<Timing>,
    /// Value must be exactly this (ContactDetail)
    #[serde(rename = "fixedContactDetail")]
    pub fixed_contact_detail: Option<ContactDetail>,
    /// Value must be exactly this (Contributor)
    #[serde(rename = "fixedContributor")]
    pub fixed_contributor: Option<Contributor>,
    /// Value must be exactly this (DataRequirement)
    #[serde(rename = "fixedDataRequirement")]
    pub fixed_data_requirement: Option<DataRequirement>,
    /// Value must be exactly this (Expression)
    #[serde(rename = "fixedExpression")]
    pub fixed_expression: Option<Expression>,
    /// Value must be exactly this (ParameterDefinition)
    #[serde(rename = "fixedParameterDefinition")]
    pub fixed_parameter_definition: Option<ParameterDefinition>,
    /// Value must be exactly this (RelatedArtifact)
    #[serde(rename = "fixedRelatedArtifact")]
    pub fixed_related_artifact: Option<RelatedArtifact>,
    /// Value must be exactly this (TriggerDefinition)
    #[serde(rename = "fixedTriggerDefinition")]
    pub fixed_trigger_definition: Option<TriggerDefinition>,
    /// Value must be exactly this (UsageContext)
    #[serde(rename = "fixedUsageContext")]
    pub fixed_usage_context: Option<UsageContext>,
    /// Value must be exactly this (Dosage)
    #[serde(rename = "fixedDosage")]
    pub fixed_dosage: Option<Dosage>,
    /// Value must be exactly this (Meta)
    #[serde(rename = "fixedMeta")]
    pub fixed_meta: Option<Meta>,
    /// Value must have at least these property values (base64Binary)
    #[serde(rename = "patternBase64Binary")]
    pub pattern_base64_binary: Option<Base64BinaryType>,
    /// Value must have at least these property values (boolean)
    #[serde(rename = "patternBoolean")]
    pub pattern_boolean: Option<BooleanType>,
    /// Value must have at least these property values (canonical)
    #[serde(rename = "patternCanonical")]
    pub pattern_canonical: Option<StringType>,
    /// Value must have at least these property values (code)
    #[serde(rename = "patternCode")]
    pub pattern_code: Option<StringType>,
    /// Value must have at least these property values (date)
    #[serde(rename = "patternDate")]
    pub pattern_date: Option<DateType>,
    /// Value must have at least these property values (dateTime)
    #[serde(rename = "patternDateTime")]
    pub pattern_date_time: Option<DateTimeType>,
    /// Value must have at least these property values (decimal)
    #[serde(rename = "patternDecimal")]
    pub pattern_decimal: Option<DecimalType>,
    /// Value must have at least these property values (id)
    #[serde(rename = "patternId")]
    pub pattern_id: Option<StringType>,
    /// Value must have at least these property values (instant)
    #[serde(rename = "patternInstant")]
    pub pattern_instant: Option<InstantType>,
    /// Value must have at least these property values (integer)
    #[serde(rename = "patternInteger")]
    pub pattern_integer: Option<IntegerType>,
    /// Value must have at least these property values (markdown)
    #[serde(rename = "patternMarkdown")]
    pub pattern_markdown: Option<StringType>,
    /// Value must have at least these property values (oid)
    #[serde(rename = "patternOid")]
    pub pattern_oid: Option<StringType>,
    /// Value must have at least these property values (positiveInt)
    #[serde(rename = "patternPositiveInt")]
    pub pattern_positive_int: Option<PositiveIntType>,
    /// Value must have at least these property values (string)
    #[serde(rename = "patternString")]
    pub pattern_string: Option<StringType>,
    /// Value must have at least these property values (time)
    #[serde(rename = "patternTime")]
    pub pattern_time: Option<TimeType>,
    /// Value must have at least these property values (unsignedInt)
    #[serde(rename = "patternUnsignedInt")]
    pub pattern_unsigned_int: Option<UnsignedIntType>,
    /// Value must have at least these property values (uri)
    #[serde(rename = "patternUri")]
    pub pattern_uri: Option<StringType>,
    /// Value must have at least these property values (url)
    #[serde(rename = "patternUrl")]
    pub pattern_url: Option<StringType>,
    /// Value must have at least these property values (uuid)
    #[serde(rename = "patternUuid")]
    pub pattern_uuid: Option<StringType>,
    /// Value must have at least these property values (Address)
    #[serde(rename = "patternAddress")]
    pub pattern_address: Option<Address>,
    /// Value must have at least these property values (Age)
    #[serde(rename = "patternAge")]
    pub pattern_age: Option<Age>,
    /// Value must have at least these property values (Annotation)
    #[serde(rename = "patternAnnotation")]
    pub pattern_annotation: Option<Annotation>,
    /// Value must have at least these property values (Attachment)
    #[serde(rename = "patternAttachment")]
    pub pattern_attachment: Option<Attachment>,
    /// Value must have at least these property values (CodeableConcept)
    #[serde(rename = "patternCodeableConcept")]
    pub pattern_codeable_concept: Option<CodeableConcept>,
    /// Value must have at least these property values (Coding)
    #[serde(rename = "patternCoding")]
    pub pattern_coding: Option<Coding>,
    /// Value must have at least these property values (ContactPoint)
    #[serde(rename = "patternContactPoint")]
    pub pattern_contact_point: Option<ContactPoint>,
    /// Value must have at least these property values (Count)
    #[serde(rename = "patternCount")]
    pub pattern_count: Option<Count>,
    /// Value must have at least these property values (Distance)
    #[serde(rename = "patternDistance")]
    pub pattern_distance: Option<Distance>,
    /// Value must have at least these property values (Duration)
    #[serde(rename = "patternDuration")]
    pub pattern_duration: Option<Duration>,
    /// Value must have at least these property values (HumanName)
    #[serde(rename = "patternHumanName")]
    pub pattern_human_name: Option<HumanName>,
    /// Value must have at least these property values (Identifier)
    #[serde(rename = "patternIdentifier")]
    pub pattern_identifier: Option<Identifier>,
    /// Value must have at least these property values (Money)
    #[serde(rename = "patternMoney")]
    pub pattern_money: Option<Money>,
    /// Value must have at least these property values (Period)
    #[serde(rename = "patternPeriod")]
    pub pattern_period: Option<Period>,
    /// Value must have at least these property values (Quantity)
    #[serde(rename = "patternQuantity")]
    pub pattern_quantity: Option<Quantity>,
    /// Value must have at least these property values (Range)
    #[serde(rename = "patternRange")]
    pub pattern_range: Option<Range>,
    /// Value must have at least these property values (Ratio)
    #[serde(rename = "patternRatio")]
    pub pattern_ratio: Option<Ratio>,
    /// Value must have at least these property values (Reference)
    #[serde(rename = "patternReference")]
    pub pattern_reference: Option<Reference>,
    /// Value must have at least these property values (SampledData)
    #[serde(rename = "patternSampledData")]
    pub pattern_sampled_data: Option<SampledData>,
    /// Value must have at least these property values (Signature)
    #[serde(rename = "patternSignature")]
    pub pattern_signature: Option<Signature>,
    /// Value must have at least these property values (Timing)
    #[serde(rename = "patternTiming")]
    pub pattern_timing: Option<Timing>,
    /// Value must have at least these property values (ContactDetail)
    #[serde(rename = "patternContactDetail")]
    pub pattern_contact_detail: Option<ContactDetail>,
    /// Value must have at least these property values (Contributor)
    #[serde(rename = "patternContributor")]
    pub pattern_contributor: Option<Contributor>,
    /// Value must have at least these property values (DataRequirement)
    #[serde(rename = "patternDataRequirement")]
    pub pattern_data_requirement: Option<DataRequirement>,
    /// Value must have at least these property values (Expression)
    #[serde(rename = "patternExpression")]
    pub pattern_expression: Option<Expression>,
    /// Value must have at least these property values (ParameterDefinition)
    #[serde(rename = "patternParameterDefinition")]
    pub pattern_parameter_definition: Option<ParameterDefinition>,
    /// Value must have at least these property values (RelatedArtifact)
    #[serde(rename = "patternRelatedArtifact")]
    pub pattern_related_artifact: Option<RelatedArtifact>,
    /// Value must have at least these property values (TriggerDefinition)
    #[serde(rename = "patternTriggerDefinition")]
    pub pattern_trigger_definition: Option<TriggerDefinition>,
    /// Value must have at least these property values (UsageContext)
    #[serde(rename = "patternUsageContext")]
    pub pattern_usage_context: Option<UsageContext>,
    /// Value must have at least these property values (Dosage)
    #[serde(rename = "patternDosage")]
    pub pattern_dosage: Option<Dosage>,
    /// Value must have at least these property values (Meta)
    #[serde(rename = "patternMeta")]
    pub pattern_meta: Option<Meta>,
    /// Example value (as defined for type)
    pub example: Option<Vec<Element>>,
    /// Minimum Allowed Value (for some types) (date)
    #[serde(rename = "minValueDate")]
    pub min_value_date: Option<DateType>,
    /// Minimum Allowed Value (for some types) (dateTime)
    #[serde(rename = "minValueDateTime")]
    pub min_value_date_time: Option<DateTimeType>,
    /// Minimum Allowed Value (for some types) (instant)
    #[serde(rename = "minValueInstant")]
    pub min_value_instant: Option<InstantType>,
    /// Minimum Allowed Value (for some types) (time)
    #[serde(rename = "minValueTime")]
    pub min_value_time: Option<TimeType>,
    /// Minimum Allowed Value (for some types) (decimal)
    #[serde(rename = "minValueDecimal")]
    pub min_value_decimal: Option<DecimalType>,
    /// Minimum Allowed Value (for some types) (integer)
    #[serde(rename = "minValueInteger")]
    pub min_value_integer: Option<IntegerType>,
    /// Minimum Allowed Value (for some types) (positiveInt)
    #[serde(rename = "minValuePositiveInt")]
    pub min_value_positive_int: Option<PositiveIntType>,
    /// Minimum Allowed Value (for some types) (unsignedInt)
    #[serde(rename = "minValueUnsignedInt")]
    pub min_value_unsigned_int: Option<UnsignedIntType>,
    /// Minimum Allowed Value (for some types) (Quantity)
    #[serde(rename = "minValueQuantity")]
    pub min_value_quantity: Option<Quantity>,
    /// Maximum Allowed Value (for some types) (date)
    #[serde(rename = "maxValueDate")]
    pub max_value_date: Option<DateType>,
    /// Maximum Allowed Value (for some types) (dateTime)
    #[serde(rename = "maxValueDateTime")]
    pub max_value_date_time: Option<DateTimeType>,
    /// Maximum Allowed Value (for some types) (instant)
    #[serde(rename = "maxValueInstant")]
    pub max_value_instant: Option<InstantType>,
    /// Maximum Allowed Value (for some types) (time)
    #[serde(rename = "maxValueTime")]
    pub max_value_time: Option<TimeType>,
    /// Maximum Allowed Value (for some types) (decimal)
    #[serde(rename = "maxValueDecimal")]
    pub max_value_decimal: Option<DecimalType>,
    /// Maximum Allowed Value (for some types) (integer)
    #[serde(rename = "maxValueInteger")]
    pub max_value_integer: Option<IntegerType>,
    /// Maximum Allowed Value (for some types) (positiveInt)
    #[serde(rename = "maxValuePositiveInt")]
    pub max_value_positive_int: Option<PositiveIntType>,
    /// Maximum Allowed Value (for some types) (unsignedInt)
    #[serde(rename = "maxValueUnsignedInt")]
    pub max_value_unsigned_int: Option<UnsignedIntType>,
    /// Maximum Allowed Value (for some types) (Quantity)
    #[serde(rename = "maxValueQuantity")]
    pub max_value_quantity: Option<Quantity>,
    /// Max length for strings
    #[serde(rename = "maxLength")]
    pub max_length: Option<IntegerType>,
    /// Extension element for the 'maxLength' primitive field. Contains metadata and extensions.
    #[serde(rename = "_maxLength")]
    pub _max_length: Option<Element>,
    /// Reference to invariant about presence
    pub condition: Option<Vec<StringType>>,
    /// Extension element for the 'condition' primitive field. Contains metadata and extensions.
    pub _condition: Option<Element>,
    /// Condition that must evaluate to true
    pub constraint: Option<Vec<Element>>,
    /// If the element must be supported
    #[serde(rename = "mustSupport")]
    pub must_support: Option<BooleanType>,
    /// Extension element for the 'mustSupport' primitive field. Contains metadata and extensions.
    #[serde(rename = "_mustSupport")]
    pub _must_support: Option<Element>,
    /// If this modifies the meaning of other elements
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<BooleanType>,
    /// Extension element for the 'isModifier' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isModifier")]
    pub _is_modifier: Option<Element>,
    /// Reason that this element is marked as a modifier
    #[serde(rename = "isModifierReason")]
    pub is_modifier_reason: Option<StringType>,
    /// Extension element for the 'isModifierReason' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isModifierReason")]
    pub _is_modifier_reason: Option<Element>,
    /// Include when _summary = true?
    #[serde(rename = "isSummary")]
    pub is_summary: Option<BooleanType>,
    /// Extension element for the 'isSummary' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isSummary")]
    pub _is_summary: Option<Element>,
    /// ValueSet details if this is coded
    pub binding: Option<Element>,
    /// Map element to another set of definitions
    pub mapping: Option<Vec<Element>>,
}
/// ElementDefinitionSlicing nested structure for the 'discriminator' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionSlicingDiscriminator {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// value | exists | pattern | type | profile
    #[serde(rename = "type")]
    pub type_: DiscriminatorType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Path to element value
    pub path: StringType,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
}
/// ElementDefinition nested structure for the 'type' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionType {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Data type or Resource (reference to definition)
    ///
    /// Binding: extensible (Either a resource or a data type, including logical model types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/defined-types
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Profiles (StructureDefinition or IG) - one must apply
    pub profile: Option<Vec<StringType>>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Profile (StructureDefinition or IG) on the Reference/canonical target - one must apply
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<StringType>>,
    /// Extension element for the 'targetProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetProfile")]
    pub _target_profile: Option<Element>,
    /// contained | referenced | bundled - how aggregated
    pub aggregation: Option<Vec<ResourceAggregationMode>>,
    /// Extension element for the 'aggregation' primitive field. Contains metadata and extensions.
    pub _aggregation: Option<Element>,
    /// either | independent | specific
    pub versioning: Option<ReferenceVersionRules>,
    /// Extension element for the 'versioning' primitive field. Contains metadata and extensions.
    pub _versioning: Option<Element>,
}
/// ElementDefinition nested structure for the 'base' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionBase {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Path that identifies the base element
    pub path: StringType,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// Min cardinality of the base element
    pub min: UnsignedIntType,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Max cardinality of the base element
    pub max: StringType,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
}
/// ElementDefinition nested structure for the 'example' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionExample {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Describes the purpose of this example
    pub label: StringType,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// Value of Example (one of allowed types) (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
    /// Value of Example (one of allowed types) (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Value of Example (one of allowed types) (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: StringType,
    /// Value of Example (one of allowed types) (code)
    #[serde(rename = "valueCode")]
    pub value_code: StringType,
    /// Value of Example (one of allowed types) (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Value of Example (one of allowed types) (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Value of Example (one of allowed types) (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Value of Example (one of allowed types) (id)
    #[serde(rename = "valueId")]
    pub value_id: StringType,
    /// Value of Example (one of allowed types) (instant)
    #[serde(rename = "valueInstant")]
    pub value_instant: InstantType,
    /// Value of Example (one of allowed types) (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Value of Example (one of allowed types) (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: StringType,
    /// Value of Example (one of allowed types) (oid)
    #[serde(rename = "valueOid")]
    pub value_oid: StringType,
    /// Value of Example (one of allowed types) (positiveInt)
    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: PositiveIntType,
    /// Value of Example (one of allowed types) (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Value of Example (one of allowed types) (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Value of Example (one of allowed types) (unsignedInt)
    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: UnsignedIntType,
    /// Value of Example (one of allowed types) (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// Value of Example (one of allowed types) (url)
    #[serde(rename = "valueUrl")]
    pub value_url: StringType,
    /// Value of Example (one of allowed types) (uuid)
    #[serde(rename = "valueUuid")]
    pub value_uuid: StringType,
    /// Value of Example (one of allowed types) (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Address,
    /// Value of Example (one of allowed types) (Age)
    #[serde(rename = "valueAge")]
    pub value_age: Age,
    /// Value of Example (one of allowed types) (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,
    /// Value of Example (one of allowed types) (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// Value of Example (one of allowed types) (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Value of Example (one of allowed types) (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Value of Example (one of allowed types) (ContactPoint)
    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: ContactPoint,
    /// Value of Example (one of allowed types) (Count)
    #[serde(rename = "valueCount")]
    pub value_count: Count,
    /// Value of Example (one of allowed types) (Distance)
    #[serde(rename = "valueDistance")]
    pub value_distance: Distance,
    /// Value of Example (one of allowed types) (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,
    /// Value of Example (one of allowed types) (HumanName)
    #[serde(rename = "valueHumanName")]
    pub value_human_name: HumanName,
    /// Value of Example (one of allowed types) (Identifier)
    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Identifier,
    /// Value of Example (one of allowed types) (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Money,
    /// Value of Example (one of allowed types) (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
    /// Value of Example (one of allowed types) (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Value of Example (one of allowed types) (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Value of Example (one of allowed types) (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// Value of Example (one of allowed types) (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Value of Example (one of allowed types) (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: SampledData,
    /// Value of Example (one of allowed types) (Signature)
    #[serde(rename = "valueSignature")]
    pub value_signature: Signature,
    /// Value of Example (one of allowed types) (Timing)
    #[serde(rename = "valueTiming")]
    pub value_timing: Timing,
    /// Value of Example (one of allowed types) (ContactDetail)
    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: ContactDetail,
    /// Value of Example (one of allowed types) (Contributor)
    #[serde(rename = "valueContributor")]
    pub value_contributor: Contributor,
    /// Value of Example (one of allowed types) (DataRequirement)
    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: DataRequirement,
    /// Value of Example (one of allowed types) (Expression)
    #[serde(rename = "valueExpression")]
    pub value_expression: Expression,
    /// Value of Example (one of allowed types) (ParameterDefinition)
    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: ParameterDefinition,
    /// Value of Example (one of allowed types) (RelatedArtifact)
    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: RelatedArtifact,
    /// Value of Example (one of allowed types) (TriggerDefinition)
    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: TriggerDefinition,
    /// Value of Example (one of allowed types) (UsageContext)
    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: UsageContext,
    /// Value of Example (one of allowed types) (Dosage)
    #[serde(rename = "valueDosage")]
    pub value_dosage: Dosage,
    /// Value of Example (one of allowed types) (Meta)
    #[serde(rename = "valueMeta")]
    pub value_meta: Meta,
}
/// ElementDefinition nested structure for the 'mapping' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionMapping {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to mapping declaration
    pub identity: StringType,
    /// Extension element for the 'identity' primitive field. Contains metadata and extensions.
    pub _identity: Option<Element>,
    /// Computable language of mapping
    pub language: Option<Mimetypes>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Details of the mapping
    pub map: StringType,
    /// Extension element for the 'map' primitive field. Contains metadata and extensions.
    pub _map: Option<Element>,
    /// Comments about the mapping or its use
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}
/// ElementDefinition nested structure for the 'binding' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionBinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// required | extensible | preferred | example
    pub strength: BindingStrength,
    /// Extension element for the 'strength' primitive field. Contains metadata and extensions.
    pub _strength: Option<Element>,
    /// Human explanation of the value set
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Source of value set
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
}
/// ElementDefinition nested structure for the 'slicing' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionSlicing {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Element values that are used to distinguish the slices
    pub discriminator: Option<Vec<Element>>,
    /// Text description of how slicing works (or not)
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// If elements must be in same order as slices
    pub ordered: Option<BooleanType>,
    /// Extension element for the 'ordered' primitive field. Contains metadata and extensions.
    pub _ordered: Option<Element>,
    /// closed | open | openAtEnd
    pub rules: ResourceSlicingRules,
    /// Extension element for the 'rules' primitive field. Contains metadata and extensions.
    pub _rules: Option<Element>,
}
/// ElementDefinition nested structure for the 'constraint' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinitionConstraint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Target of 'condition' reference above
    pub key: StringType,
    /// Extension element for the 'key' primitive field. Contains metadata and extensions.
    pub _key: Option<Element>,
    /// Why this constraint is necessary or appropriate
    pub requirements: Option<StringType>,
    /// Extension element for the 'requirements' primitive field. Contains metadata and extensions.
    pub _requirements: Option<Element>,
    /// error | warning
    pub severity: ConstraintSeverity,
    /// Extension element for the 'severity' primitive field. Contains metadata and extensions.
    pub _severity: Option<Element>,
    /// Human description of constraint
    pub human: StringType,
    /// Extension element for the 'human' primitive field. Contains metadata and extensions.
    pub _human: Option<Element>,
    /// FHIRPath expression of constraint
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// XPath expression of constraint
    pub xpath: Option<StringType>,
    /// Extension element for the 'xpath' primitive field. Contains metadata and extensions.
    pub _xpath: Option<Element>,
    /// Reference to original source of constraint
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
}

impl Default for ElementDefinition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: StringType::default(),
            _path: Default::default(),
            representation: Default::default(),
            _representation: Default::default(),
            slice_name: Default::default(),
            _slice_name: Default::default(),
            slice_is_constraining: Default::default(),
            _slice_is_constraining: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            code: Default::default(),
            slicing: Default::default(),
            short: Default::default(),
            _short: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            requirements: Default::default(),
            _requirements: Default::default(),
            alias: Default::default(),
            _alias: Default::default(),
            min: Default::default(),
            _min: Default::default(),
            max: Default::default(),
            _max: Default::default(),
            base_definition: Default::default(),
            content_reference: Default::default(),
            _content_reference: Default::default(),
            type_: Default::default(),
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
            meaning_when_missing: Default::default(),
            _meaning_when_missing: Default::default(),
            order_meaning: Default::default(),
            _order_meaning: Default::default(),
            fixed_base64_binary: Default::default(),
            fixed_boolean: Default::default(),
            fixed_canonical: Default::default(),
            fixed_code: Default::default(),
            fixed_date: Default::default(),
            fixed_date_time: Default::default(),
            fixed_decimal: Default::default(),
            fixed_id: Default::default(),
            fixed_instant: Default::default(),
            fixed_integer: Default::default(),
            fixed_markdown: Default::default(),
            fixed_oid: Default::default(),
            fixed_positive_int: Default::default(),
            fixed_string: Default::default(),
            fixed_time: Default::default(),
            fixed_unsigned_int: Default::default(),
            fixed_uri: Default::default(),
            fixed_url: Default::default(),
            fixed_uuid: Default::default(),
            fixed_address: Default::default(),
            fixed_age: Default::default(),
            fixed_annotation: Default::default(),
            fixed_attachment: Default::default(),
            fixed_codeable_concept: Default::default(),
            fixed_coding: Default::default(),
            fixed_contact_point: Default::default(),
            fixed_count: Default::default(),
            fixed_distance: Default::default(),
            fixed_duration: Default::default(),
            fixed_human_name: Default::default(),
            fixed_identifier: Default::default(),
            fixed_money: Default::default(),
            fixed_period: Default::default(),
            fixed_quantity: Default::default(),
            fixed_range: Default::default(),
            fixed_ratio: Default::default(),
            fixed_reference: Default::default(),
            fixed_sampled_data: Default::default(),
            fixed_signature: Default::default(),
            fixed_timing: Default::default(),
            fixed_contact_detail: Default::default(),
            fixed_contributor: Default::default(),
            fixed_data_requirement: Default::default(),
            fixed_expression: Default::default(),
            fixed_parameter_definition: Default::default(),
            fixed_related_artifact: Default::default(),
            fixed_trigger_definition: Default::default(),
            fixed_usage_context: Default::default(),
            fixed_dosage: Default::default(),
            fixed_meta: Default::default(),
            pattern_base64_binary: Default::default(),
            pattern_boolean: Default::default(),
            pattern_canonical: Default::default(),
            pattern_code: Default::default(),
            pattern_date: Default::default(),
            pattern_date_time: Default::default(),
            pattern_decimal: Default::default(),
            pattern_id: Default::default(),
            pattern_instant: Default::default(),
            pattern_integer: Default::default(),
            pattern_markdown: Default::default(),
            pattern_oid: Default::default(),
            pattern_positive_int: Default::default(),
            pattern_string: Default::default(),
            pattern_time: Default::default(),
            pattern_unsigned_int: Default::default(),
            pattern_uri: Default::default(),
            pattern_url: Default::default(),
            pattern_uuid: Default::default(),
            pattern_address: Default::default(),
            pattern_age: Default::default(),
            pattern_annotation: Default::default(),
            pattern_attachment: Default::default(),
            pattern_codeable_concept: Default::default(),
            pattern_coding: Default::default(),
            pattern_contact_point: Default::default(),
            pattern_count: Default::default(),
            pattern_distance: Default::default(),
            pattern_duration: Default::default(),
            pattern_human_name: Default::default(),
            pattern_identifier: Default::default(),
            pattern_money: Default::default(),
            pattern_period: Default::default(),
            pattern_quantity: Default::default(),
            pattern_range: Default::default(),
            pattern_ratio: Default::default(),
            pattern_reference: Default::default(),
            pattern_sampled_data: Default::default(),
            pattern_signature: Default::default(),
            pattern_timing: Default::default(),
            pattern_contact_detail: Default::default(),
            pattern_contributor: Default::default(),
            pattern_data_requirement: Default::default(),
            pattern_expression: Default::default(),
            pattern_parameter_definition: Default::default(),
            pattern_related_artifact: Default::default(),
            pattern_trigger_definition: Default::default(),
            pattern_usage_context: Default::default(),
            pattern_dosage: Default::default(),
            pattern_meta: Default::default(),
            example: Default::default(),
            min_value_date: Default::default(),
            min_value_date_time: Default::default(),
            min_value_instant: Default::default(),
            min_value_time: Default::default(),
            min_value_decimal: Default::default(),
            min_value_integer: Default::default(),
            min_value_positive_int: Default::default(),
            min_value_unsigned_int: Default::default(),
            min_value_quantity: Default::default(),
            max_value_date: Default::default(),
            max_value_date_time: Default::default(),
            max_value_instant: Default::default(),
            max_value_time: Default::default(),
            max_value_decimal: Default::default(),
            max_value_integer: Default::default(),
            max_value_positive_int: Default::default(),
            max_value_unsigned_int: Default::default(),
            max_value_quantity: Default::default(),
            max_length: Default::default(),
            _max_length: Default::default(),
            condition: Default::default(),
            _condition: Default::default(),
            constraint: Default::default(),
            must_support: Default::default(),
            _must_support: Default::default(),
            is_modifier: Default::default(),
            _is_modifier: Default::default(),
            is_modifier_reason: Default::default(),
            _is_modifier_reason: Default::default(),
            is_summary: Default::default(),
            _is_summary: Default::default(),
            binding: Default::default(),
            mapping: Default::default(),
        }
    }
}

impl Default for ElementDefinitionSlicingDiscriminator {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            path: Default::default(),
            _path: Default::default(),
        }
    }
}

impl Default for ElementDefinitionType {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: StringType::default(),
            _code: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            target_profile: Default::default(),
            _target_profile: Default::default(),
            aggregation: Default::default(),
            _aggregation: Default::default(),
            versioning: Default::default(),
            _versioning: Default::default(),
        }
    }
}

impl Default for ElementDefinitionBase {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: StringType::default(),
            _path: Default::default(),
            min: UnsignedIntType::default(),
            _min: Default::default(),
            max: StringType::default(),
            _max: Default::default(),
        }
    }
}

impl Default for ElementDefinitionExample {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            label: StringType::default(),
            _label: Default::default(),
            value_base64_binary: Default::default(),
            value_boolean: Default::default(),
            value_canonical: Default::default(),
            value_code: Default::default(),
            value_date: Default::default(),
            value_date_time: Default::default(),
            value_decimal: Default::default(),
            value_id: Default::default(),
            value_instant: Default::default(),
            value_integer: Default::default(),
            value_markdown: Default::default(),
            value_oid: Default::default(),
            value_positive_int: Default::default(),
            value_string: Default::default(),
            value_time: Default::default(),
            value_unsigned_int: Default::default(),
            value_uri: Default::default(),
            value_url: Default::default(),
            value_uuid: Default::default(),
            value_address: Default::default(),
            value_age: Default::default(),
            value_annotation: Default::default(),
            value_attachment: Default::default(),
            value_codeable_concept: Default::default(),
            value_coding: Default::default(),
            value_contact_point: Default::default(),
            value_count: Default::default(),
            value_distance: Default::default(),
            value_duration: Default::default(),
            value_human_name: Default::default(),
            value_identifier: Default::default(),
            value_money: Default::default(),
            value_period: Default::default(),
            value_quantity: Default::default(),
            value_range: Default::default(),
            value_ratio: Default::default(),
            value_reference: Default::default(),
            value_sampled_data: Default::default(),
            value_signature: Default::default(),
            value_timing: Default::default(),
            value_contact_detail: Default::default(),
            value_contributor: Default::default(),
            value_data_requirement: Default::default(),
            value_expression: Default::default(),
            value_parameter_definition: Default::default(),
            value_related_artifact: Default::default(),
            value_trigger_definition: Default::default(),
            value_usage_context: Default::default(),
            value_dosage: Default::default(),
            value_meta: Default::default(),
        }
    }
}

impl Default for ElementDefinitionMapping {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identity: StringType::default(),
            _identity: Default::default(),
            language: Default::default(),
            _language: Default::default(),
            map: StringType::default(),
            _map: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
        }
    }
}

impl Default for ElementDefinitionBinding {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            strength: BindingStrength::default(),
            _strength: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
        }
    }
}

impl Default for ElementDefinitionSlicing {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            discriminator: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            ordered: Default::default(),
            _ordered: Default::default(),
            rules: ResourceSlicingRules::default(),
            _rules: Default::default(),
        }
    }
}

impl Default for ElementDefinitionConstraint {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            key: StringType::default(),
            _key: Default::default(),
            requirements: Default::default(),
            _requirements: Default::default(),
            severity: ConstraintSeverity::default(),
            _severity: Default::default(),
            human: StringType::default(),
            _human: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            xpath: Default::default(),
            _xpath: Default::default(),
            source: Default::default(),
            _source: Default::default(),
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
    rh_foundation::Invariant::new("eld-1", rh_foundation::Severity::Error, "If there are no discriminators, there must be a definition", "discriminator.exists() or description.exists()").with_xpath("(f:discriminator) or (f:description)"),
    rh_foundation::Invariant::new("eld-11", rh_foundation::Severity::Error, "Binding can only be present for coded elements, string, and uri", "binding.empty() or type.code.empty() or type.select((code = 'code') or (code = 'Coding') or (code='CodeableConcept') or (code = 'Quantity') or (code = 'string') or (code = 'uri')).exists()").with_xpath("not(exists(f:binding)) or (count(f:type/f:code) = 0) or  f:type/f:code/@value=('code','Coding','CodeableConcept','Quantity','string', 'uri')"),
    rh_foundation::Invariant::new("eld-12", rh_foundation::Severity::Error, "ValueSet SHALL start with http:// or https:// or urn:", "valueSet.exists() implies (valueSet.startsWith('http:') or valueSet.startsWith('https') or valueSet.startsWith('urn:'))").with_xpath("(starts-with(string(f:valueSet/@value), 'http:') or starts-with(string(f:valueSet/@value), 'https:') or starts-with(string(f:valueSet/@value), 'urn:'))"),
    rh_foundation::Invariant::new("eld-13", rh_foundation::Severity::Error, "Types must be unique by code", "type.select(code).isDistinct()").with_xpath("not(exists(for $type in f:type return $type/preceding-sibling::f:type[f:code/@value=$type/f:code/@value]))"),
    rh_foundation::Invariant::new("eld-14", rh_foundation::Severity::Error, "Constraints must be unique by key", "constraint.select(key).isDistinct()").with_xpath("count(f:constraint) = count(distinct-values(f:constraint/f:key/@value))"),
    rh_foundation::Invariant::new("eld-15", rh_foundation::Severity::Error, "default value and meaningWhenMissing are mutually exclusive", "defaultValue.empty() or meaningWhenMissing.empty()").with_xpath("not(exists(f:*[starts-with(local-name(.), 'fixed')])) or not(exists(f:meaningWhenMissing))"),
    rh_foundation::Invariant::new("eld-16", rh_foundation::Severity::Error, "sliceName must be composed of proper tokens separated by \"/\"", "sliceName.empty() or sliceName.matches('^[a-zA-Z0-9\\\\/\\\\-_\\\\[\\\\]\\\\@]+$')").with_xpath("not(exists(f:sliceName/@value)) or matches(f:sliceName/@value, '^[a-zA-Z0-9\\/\\-_\\[\\]\\@]+$')"),
    rh_foundation::Invariant::new("eld-17", rh_foundation::Severity::Error, "targetProfile is only allowed if the type is Reference or canonical", "(code='Reference' or code = 'canonical') or targetProfile.empty()").with_xpath("not(exists(f:targetProfile)) or (f:code/@value = 'Reference')"),
    rh_foundation::Invariant::new("eld-18", rh_foundation::Severity::Error, "Must have a modifier reason if isModifier = true", "(isModifier.exists() and isModifier) implies isModifierReason.exists()").with_xpath("not(f:isModifier/@value = 'true') or exists(f:isModifierReason)"),
    rh_foundation::Invariant::new("eld-19", rh_foundation::Severity::Error, "Element names cannot include some special characters", "path.matches('[^\\\\s\\\\.,:;\\\\\\'\"\\\\/|?!@#$%&*()\\\\[\\\\]{}]{1,64}(\\\\.[^\\\\s\\\\.,:;\\\\\\'\"\\\\/|?!@#$%&*()\\\\[\\\\]{}]{1,64}(\\\\[x\\\\])?(\\\\:[^\\\\s\\\\.]+)?)*')").with_xpath("matches(path/@value, '[^\\s\\.,:;\\'&quot;\\/|?!@#$%&amp;*()\\[\\]{}]{1,64}(\\.[^\\s\\.,:;\\'&quot;\\/|?!@#$%&amp;*()\\[\\]{}]{1,64}(\\[x\\])?(\\:[^\\s\\.]+)?)*')"),
    rh_foundation::Invariant::new("eld-2", rh_foundation::Severity::Error, "Min <= Max", "min.empty() or max.empty() or (max = '*') or iif(max != '*', min <= max.toInteger())").with_xpath("not(exists(f:min)) or not(exists(f:max)) or (not(f:max/@value) and not(f:min/@value)) or (f:max/@value = '*') or (number(f:max/@value) >= f:min/@value)"),
    rh_foundation::Invariant::new("eld-20", rh_foundation::Severity::Warning, "Element names should be simple alphanumerics with a max of 64 characters, or code generation tools may be broken", "path.matches('[A-Za-z][A-Za-z0-9]*(\\\\.[a-z][A-Za-z0-9]*(\\\\[x])?)*')").with_xpath("matches(path/@value, '[A-Za-z][A-Za-z0-9]*(\\.[a-z][A-Za-z0-9]*(\\[x])?)*')"),
    rh_foundation::Invariant::new("eld-21", rh_foundation::Severity::Warning, "Constraints should have an expression or else validators will not be able to enforce them", "expression.exists()").with_xpath("exists(f:expression/@value)"),
    rh_foundation::Invariant::new("eld-22", rh_foundation::Severity::Error, "sliceIsConstraining can only appear if slicename is present", "sliceIsConstraining.exists() implies sliceName.exists()").with_xpath("exists(f:sliceName) or not(exists(f:sliceIsConstraining))"),
    rh_foundation::Invariant::new("eld-3", rh_foundation::Severity::Error, "Max SHALL be a number or \"*\"", "empty() or ($this = '*') or (toInteger() >= 0)").with_xpath("@value='*' or (normalize-space(@value)!='' and normalize-space(translate(@value, '0123456789',''))='')"),
    rh_foundation::Invariant::new("eld-4", rh_foundation::Severity::Error, "Aggregation may only be specified if one of the allowed types for the element is a reference", "aggregation.empty() or (code = 'Reference') or (code = 'canonical')").with_xpath("not(exists(f:aggregation)) or exists(f:code[@value = 'Reference']) or exists(f:code[@value = 'canonical'])"),
    rh_foundation::Invariant::new("eld-5", rh_foundation::Severity::Error, "if the element definition has a contentReference, it cannot have type, defaultValue, fixed, pattern, example, minValue, maxValue, maxLength, or binding", "contentReference.empty() or (type.empty() and defaultValue.empty() and fixed.empty() and pattern.empty() and example.empty() and minValue.empty() and maxValue.empty() and maxLength.empty() and binding.empty())").with_xpath("not(exists(f:contentReference) and (exists(f:type) or exists(f:*[starts-with(local-name(.), 'value')]) or exists(f:*[starts-with(local-name(.), 'defaultValue')])  or exists(f:*[starts-with(local-name(.), 'fixed')]) or exists(f:*[starts-with(local-name(.), 'pattern')]) or exists(f:*[starts-with(local-name(.), 'example')]) or exists(f:*[starts-with(local-name(.), 'f:minValue')]) or exists(f:*[starts-with(local-name(.), 'f:maxValue')]) or exists(f:maxLength) or exists(f:binding)))"),
    rh_foundation::Invariant::new("eld-6", rh_foundation::Severity::Error, "Fixed value may only be specified if there is one type", "fixed.empty() or (type.count()  <= 1)").with_xpath("not(exists(f:*[starts-with(local-name(.), 'fixed')])) or (count(f:type)<=1)"),
    rh_foundation::Invariant::new("eld-7", rh_foundation::Severity::Error, "Pattern may only be specified if there is one type", "pattern.empty() or (type.count() <= 1)").with_xpath("not(exists(f:*[starts-with(local-name(.), 'pattern')])) or (count(f:type)<=1)"),
    rh_foundation::Invariant::new("eld-8", rh_foundation::Severity::Error, "Pattern and fixed are mutually exclusive", "pattern.empty() or fixed.empty()").with_xpath("not(exists(f:*[starts-with(local-name(.), 'pattern')])) or not(exists(f:*[starts-with(local-name(.), 'fixed')]))"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("ElementDefinition.binding.strength", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/binding-strength|4.0.1").with_description("Indication of the degree of conformance expectations associated with a binding."),
    rh_foundation::ElementBinding::new("ElementDefinition.constraint.severity", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/constraint-severity|4.0.1").with_description("SHALL applications comply with this constraint?"),
    rh_foundation::ElementBinding::new("ElementDefinition.mapping.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1").with_description("The mime type of an attachment. Any valid mime type is allowed."),
    rh_foundation::ElementBinding::new("ElementDefinition.representation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/property-representation|4.0.1").with_description("How a property is represented when serialized."),
    rh_foundation::ElementBinding::new("ElementDefinition.slicing.discriminator.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/discriminator-type|4.0.1").with_description("How an element value is interpreted when discrimination is evaluated."),
    rh_foundation::ElementBinding::new("ElementDefinition.slicing.rules", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-slicing-rules|4.0.1").with_description("How slices are interpreted when evaluating an instance."),
    rh_foundation::ElementBinding::new("ElementDefinition.type.aggregation", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/resource-aggregation-mode|4.0.1").with_description("How resource references can be aggregated."),
    rh_foundation::ElementBinding::new("ElementDefinition.type.versioning", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/reference-version-rules|4.0.1").with_description("Whether a reference needs to be version specific or version independent, or whether either can be used."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ElementDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.path", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.representation", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.sliceName", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.sliceIsConstraining",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.code", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.discriminator.path",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.slicing.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.ordered", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.slicing.rules", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.short", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.requirements", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.alias", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.min", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.max", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.path", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.min", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.base.max", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.contentReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.type", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.profile", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.targetProfile", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.aggregation", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.type.versioning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.defaultValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.meaningWhenMissing",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.orderMeaning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.fixed[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.pattern[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.example", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.extension", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.example.label", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.example.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.minValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.maxValue[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.maxLength", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.condition", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.constraint.key", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.requirements",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.severity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.human",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.xpath",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.constraint.source",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mustSupport", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.isModifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.isModifierReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.isSummary", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.binding.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.strength",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.binding.valueSet",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping", 0, None),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.mapping.identity",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ElementDefinition.mapping.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.map", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ElementDefinition.mapping.comment", 0, Some(1)),
        ]
    });

impl crate::validation::ValidatableResource for ElementDefinition {
    fn resource_type(&self) -> &'static str {
        "ElementDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ElementDefinition")
    }
}
