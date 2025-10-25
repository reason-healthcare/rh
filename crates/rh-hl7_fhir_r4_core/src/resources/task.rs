use crate::bindings::request_priority::RequestPriority;
use crate::bindings::task_intent::TaskIntent;
use crate::bindings::task_status::TaskStatus;
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
use crate::datatypes::extension::Extension;
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
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Task
///
/// A task to be performed.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Task
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Task
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Task Instance Identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Formal definition of task
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<StringType>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Formal definition of task
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<StringType>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Request fulfilled by this task
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Requisition or grouper id
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// Composite task
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// draft | requested | received | accepted | +
    pub status: TaskStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    ///
    /// Binding: example (Codes to identify the reason for current status.  These will typically be specific to a particular workflow.)
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// E.g. "Specimen collected", "IV prepped"
    ///
    /// Binding: example (The domain-specific business-contextual sub-state of the task.  For example: "Blood drawn", "IV inserted", "Awaiting physician signature", etc.)
    #[serde(rename = "businessStatus")]
    pub business_status: Option<CodeableConcept>,
    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: TaskIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Task Type
    ///
    /// Binding: example (Codes to identify what the task involves.  These will typically be specific to a particular workflow.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/task-code
    pub code: Option<CodeableConcept>,
    /// Human-readable explanation of task
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// What task is acting on
    pub focus: Option<Reference>,
    /// Beneficiary of the Task
    #[serde(rename = "for")]
    pub for_: Option<Reference>,
    /// Healthcare event during which this task originated
    pub encounter: Option<Reference>,
    /// Start and end time of execution
    #[serde(rename = "executionPeriod")]
    pub execution_period: Option<Period>,
    /// Task Creation Date
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Task Last Modified Date
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTimeType>,
    /// Extension element for the 'lastModified' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastModified")]
    pub _last_modified: Option<Element>,
    /// Who is asking for task to be done
    pub requester: Option<Reference>,
    /// Requested performer
    ///
    /// Binding: preferred (The type(s) of task performers allowed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/performer-role
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,
    /// Responsible individual
    pub owner: Option<Reference>,
    /// Where task occurs
    pub location: Option<Reference>,
    /// Why task is needed
    ///
    /// Binding: example (Indicates why the task is needed.  E.g. Suspended because patient admitted to hospital.)
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<CodeableConcept>,
    /// Why task is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Reference>,
    /// Associated insurance coverage
    pub insurance: Option<Vec<Reference>>,
    /// Comments made about the task
    pub note: Option<Vec<Annotation>>,
    /// Key events in history of the Task
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Reference>>,
    /// Constraints on fulfillment tasks
    pub restriction: Option<TaskRestriction>,
    /// Information used to perform task
    pub input: Option<Vec<TaskInput>>,
    /// Information produced as part of task
    pub output: Option<Vec<TaskOutput>>,
}
/// replaces
///
/// Completed or terminated task(s) whose function is taken by this new task.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/task-replaces
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReplaces {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Task nested structure for the 'restriction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRestriction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// How many times to repeat
    pub repetitions: Option<PositiveIntType>,
    /// Extension element for the 'repetitions' primitive field. Contains metadata and extensions.
    pub _repetitions: Option<Element>,
    /// When fulfillment sought
    pub period: Option<Period>,
    /// For whom is fulfillment sought?
    pub recipient: Option<Vec<Reference>>,
}
/// Task nested structure for the 'input' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for the input
    ///
    /// Binding: example (Codes to identify types of input parameters.  These will typically be specific to a particular workflow.  E.g. "Comparison source", "Applicable consent", "Concomitent Medications", etc.)
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Content to use in performing the task (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
    /// Content to use in performing the task (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Content to use in performing the task (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: StringType,
    /// Content to use in performing the task (code)
    #[serde(rename = "valueCode")]
    pub value_code: StringType,
    /// Content to use in performing the task (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Content to use in performing the task (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Content to use in performing the task (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Content to use in performing the task (id)
    #[serde(rename = "valueId")]
    pub value_id: StringType,
    /// Content to use in performing the task (instant)
    #[serde(rename = "valueInstant")]
    pub value_instant: InstantType,
    /// Content to use in performing the task (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Content to use in performing the task (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: StringType,
    /// Content to use in performing the task (oid)
    #[serde(rename = "valueOid")]
    pub value_oid: StringType,
    /// Content to use in performing the task (positiveInt)
    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: PositiveIntType,
    /// Content to use in performing the task (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Content to use in performing the task (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Content to use in performing the task (unsignedInt)
    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: UnsignedIntType,
    /// Content to use in performing the task (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// Content to use in performing the task (url)
    #[serde(rename = "valueUrl")]
    pub value_url: StringType,
    /// Content to use in performing the task (uuid)
    #[serde(rename = "valueUuid")]
    pub value_uuid: StringType,
    /// Content to use in performing the task (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Address,
    /// Content to use in performing the task (Age)
    #[serde(rename = "valueAge")]
    pub value_age: Age,
    /// Content to use in performing the task (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,
    /// Content to use in performing the task (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// Content to use in performing the task (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Content to use in performing the task (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Content to use in performing the task (ContactPoint)
    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: ContactPoint,
    /// Content to use in performing the task (Count)
    #[serde(rename = "valueCount")]
    pub value_count: Count,
    /// Content to use in performing the task (Distance)
    #[serde(rename = "valueDistance")]
    pub value_distance: Distance,
    /// Content to use in performing the task (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,
    /// Content to use in performing the task (HumanName)
    #[serde(rename = "valueHumanName")]
    pub value_human_name: HumanName,
    /// Content to use in performing the task (Identifier)
    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Identifier,
    /// Content to use in performing the task (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Money,
    /// Content to use in performing the task (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
    /// Content to use in performing the task (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Content to use in performing the task (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Content to use in performing the task (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// Content to use in performing the task (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Content to use in performing the task (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: SampledData,
    /// Content to use in performing the task (Signature)
    #[serde(rename = "valueSignature")]
    pub value_signature: Signature,
    /// Content to use in performing the task (Timing)
    #[serde(rename = "valueTiming")]
    pub value_timing: Timing,
    /// Content to use in performing the task (ContactDetail)
    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: ContactDetail,
    /// Content to use in performing the task (Contributor)
    #[serde(rename = "valueContributor")]
    pub value_contributor: Contributor,
    /// Content to use in performing the task (DataRequirement)
    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: DataRequirement,
    /// Content to use in performing the task (Expression)
    #[serde(rename = "valueExpression")]
    pub value_expression: Expression,
    /// Content to use in performing the task (ParameterDefinition)
    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: ParameterDefinition,
    /// Content to use in performing the task (RelatedArtifact)
    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: RelatedArtifact,
    /// Content to use in performing the task (TriggerDefinition)
    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: TriggerDefinition,
    /// Content to use in performing the task (UsageContext)
    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: UsageContext,
    /// Content to use in performing the task (Dosage)
    #[serde(rename = "valueDosage")]
    pub value_dosage: Dosage,
    /// Content to use in performing the task (Meta)
    #[serde(rename = "valueMeta")]
    pub value_meta: Meta,
}
/// Task nested structure for the 'output' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for output
    ///
    /// Binding: example (Codes to identify types of input parameters.  These will typically be specific to a particular workflow.  E.g. "Identified issues", "Preliminary results", "Filler order", "Final results", etc.)
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Result of output (base64Binary)
    #[serde(rename = "valueBase64Binary")]
    pub value_base64_binary: Base64BinaryType,
    /// Result of output (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Result of output (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: StringType,
    /// Result of output (code)
    #[serde(rename = "valueCode")]
    pub value_code: StringType,
    /// Result of output (date)
    #[serde(rename = "valueDate")]
    pub value_date: DateType,
    /// Result of output (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Result of output (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
    /// Result of output (id)
    #[serde(rename = "valueId")]
    pub value_id: StringType,
    /// Result of output (instant)
    #[serde(rename = "valueInstant")]
    pub value_instant: InstantType,
    /// Result of output (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Result of output (markdown)
    #[serde(rename = "valueMarkdown")]
    pub value_markdown: StringType,
    /// Result of output (oid)
    #[serde(rename = "valueOid")]
    pub value_oid: StringType,
    /// Result of output (positiveInt)
    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: PositiveIntType,
    /// Result of output (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Result of output (time)
    #[serde(rename = "valueTime")]
    pub value_time: TimeType,
    /// Result of output (unsignedInt)
    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: UnsignedIntType,
    /// Result of output (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// Result of output (url)
    #[serde(rename = "valueUrl")]
    pub value_url: StringType,
    /// Result of output (uuid)
    #[serde(rename = "valueUuid")]
    pub value_uuid: StringType,
    /// Result of output (Address)
    #[serde(rename = "valueAddress")]
    pub value_address: Address,
    /// Result of output (Age)
    #[serde(rename = "valueAge")]
    pub value_age: Age,
    /// Result of output (Annotation)
    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,
    /// Result of output (Attachment)
    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
    /// Result of output (CodeableConcept)
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
    /// Result of output (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Result of output (ContactPoint)
    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: ContactPoint,
    /// Result of output (Count)
    #[serde(rename = "valueCount")]
    pub value_count: Count,
    /// Result of output (Distance)
    #[serde(rename = "valueDistance")]
    pub value_distance: Distance,
    /// Result of output (Duration)
    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,
    /// Result of output (HumanName)
    #[serde(rename = "valueHumanName")]
    pub value_human_name: HumanName,
    /// Result of output (Identifier)
    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Identifier,
    /// Result of output (Money)
    #[serde(rename = "valueMoney")]
    pub value_money: Money,
    /// Result of output (Period)
    #[serde(rename = "valuePeriod")]
    pub value_period: Period,
    /// Result of output (Quantity)
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,
    /// Result of output (Range)
    #[serde(rename = "valueRange")]
    pub value_range: Range,
    /// Result of output (Ratio)
    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,
    /// Result of output (Reference)
    #[serde(rename = "valueReference")]
    pub value_reference: Reference,
    /// Result of output (SampledData)
    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: SampledData,
    /// Result of output (Signature)
    #[serde(rename = "valueSignature")]
    pub value_signature: Signature,
    /// Result of output (Timing)
    #[serde(rename = "valueTiming")]
    pub value_timing: Timing,
    /// Result of output (ContactDetail)
    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: ContactDetail,
    /// Result of output (Contributor)
    #[serde(rename = "valueContributor")]
    pub value_contributor: Contributor,
    /// Result of output (DataRequirement)
    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: DataRequirement,
    /// Result of output (Expression)
    #[serde(rename = "valueExpression")]
    pub value_expression: Expression,
    /// Result of output (ParameterDefinition)
    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: ParameterDefinition,
    /// Result of output (RelatedArtifact)
    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: RelatedArtifact,
    /// Result of output (TriggerDefinition)
    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: TriggerDefinition,
    /// Result of output (UsageContext)
    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: UsageContext,
    /// Result of output (Dosage)
    #[serde(rename = "valueDosage")]
    pub value_dosage: Dosage,
    /// Result of output (Meta)
    #[serde(rename = "valueMeta")]
    pub value_meta: Meta,
}
/// Candidate List
///
/// Identifies the individuals who are candidates for being the owner of the task.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/task-candidateList
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCandidateList {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            group_identifier: Default::default(),
            part_of: Default::default(),
            status: TaskStatus::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            business_status: Default::default(),
            intent: TaskIntent::default(),
            _intent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            focus: Default::default(),
            for_: Default::default(),
            encounter: Default::default(),
            execution_period: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            last_modified: Default::default(),
            _last_modified: Default::default(),
            requester: Default::default(),
            performer_type: Default::default(),
            owner: Default::default(),
            location: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            insurance: Default::default(),
            note: Default::default(),
            relevant_history: Default::default(),
            restriction: Default::default(),
            input: Default::default(),
            output: Default::default(),
        }
    }
}

impl Default for TaskReplaces {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for TaskRestriction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            repetitions: Default::default(),
            _repetitions: Default::default(),
            period: Default::default(),
            recipient: Default::default(),
        }
    }
}

impl Default for TaskInput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
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

impl Default for TaskOutput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
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

impl Default for TaskCandidateList {
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
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("inv-1", rh_foundation::Severity::Error, "Last modified date must be greater than or equal to authored-on date.", "lastModified.exists().not() or authoredOn.exists().not() or lastModified >= authoredOn").with_xpath("not(exists(f:lastModified/@value)) or not(exists(f:authoredOn/@value)) or f:lastModified/@value >= f:authoredOn/@value"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "Task.intent",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/task-intent|4.0.1",
            )
            .with_description("Distinguishes whether the task is a proposal, plan or full order."),
            rh_foundation::ElementBinding::new(
                "Task.priority",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/request-priority|4.0.1",
            )
            .with_description("The task's priority."),
            rh_foundation::ElementBinding::new(
                "Task.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/task-status|4.0.1",
            )
            .with_description("The current status of the task."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Task.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.contained", 0, None),
            rh_foundation::ElementCardinality::new("Task.extension", 0, None),
            rh_foundation::ElementCardinality::new("Task.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Task.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Task.instantiatesCanonical", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.instantiatesUri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Task.groupIdentifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.partOf", 0, None),
            rh_foundation::ElementCardinality::new("Task.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Task.statusReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.businessStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Task.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.focus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.for", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.executionPeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.lastModified", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.requester", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.performerType", 0, None),
            rh_foundation::ElementCardinality::new("Task.owner", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.reasonCode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.reasonReference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.insurance", 0, None),
            rh_foundation::ElementCardinality::new("Task.note", 0, None),
            rh_foundation::ElementCardinality::new("Task.relevantHistory", 0, None),
            rh_foundation::ElementCardinality::new("Task.restriction", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.restriction.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.restriction.extension", 0, None),
            rh_foundation::ElementCardinality::new("Task.restriction.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Task.restriction.repetitions", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.restriction.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.restriction.recipient", 0, None),
            rh_foundation::ElementCardinality::new("Task.input", 0, None),
            rh_foundation::ElementCardinality::new("Task.input.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.input.extension", 0, None),
            rh_foundation::ElementCardinality::new("Task.input.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Task.input.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Task.input.value[x]", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Task.output", 0, None),
            rh_foundation::ElementCardinality::new("Task.output.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Task.output.extension", 0, None),
            rh_foundation::ElementCardinality::new("Task.output.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Task.output.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Task.output.value[x]", 1, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Task {
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

impl crate::traits::resource::ResourceMutators for Task {
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

impl crate::traits::resource::ResourceExistence for Task {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Task {
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

impl crate::traits::domain_resource::DomainResourceMutators for Task {
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

impl crate::traits::domain_resource::DomainResourceExistence for Task {
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

impl crate::traits::task::TaskAccessors for Task {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> Option<StringType> {
        self.instantiates_canonical.clone()
    }
    fn instantiates_uri(&self) -> Option<StringType> {
        self.instantiates_uri.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> TaskStatus {
        self.status.clone()
    }
    fn status_reason(&self) -> Option<CodeableConcept> {
        self.status_reason.clone()
    }
    fn business_status(&self) -> Option<CodeableConcept> {
        self.business_status.clone()
    }
    fn intent(&self) -> TaskIntent {
        self.intent.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn focus(&self) -> Option<Reference> {
        self.focus.clone()
    }
    fn for_(&self) -> Option<Reference> {
        self.for_.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn execution_period(&self) -> Option<Period> {
        self.execution_period.clone()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn last_modified(&self) -> Option<DateTimeType> {
        self.last_modified.clone()
    }
    fn requester(&self) -> Option<Reference> {
        self.requester.clone()
    }
    fn performer_type(&self) -> &[CodeableConcept] {
        self.performer_type.as_deref().unwrap_or(&[])
    }
    fn owner(&self) -> Option<Reference> {
        self.owner.clone()
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn reason_code(&self) -> Option<CodeableConcept> {
        self.reason_code.clone()
    }
    fn reason_reference(&self) -> Option<Reference> {
        self.reason_reference.clone()
    }
    fn insurance(&self) -> &[Reference] {
        self.insurance.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn relevant_history(&self) -> &[Reference] {
        self.relevant_history.as_deref().unwrap_or(&[])
    }
    fn restriction(&self) -> Option<TaskRestriction> {
        self.restriction.clone()
    }
    fn input(&self) -> &[TaskInput] {
        self.input.as_deref().unwrap_or(&[])
    }
    fn output(&self) -> &[TaskOutput] {
        self.output.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::task::TaskMutators for Task {
    fn new() -> Self {
        Self::default()
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
    fn set_instantiates_canonical(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn set_instantiates_uri(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_part_of(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.part_of = Some(value);
        resource
    }
    fn add_part_of(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.part_of.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: TaskStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_status_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_business_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.business_status = Some(value);
        resource
    }
    fn set_intent(self, value: TaskIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
        resource
    }
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_focus(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.focus = Some(value);
        resource
    }
    fn set_for_(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.for_ = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_execution_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.execution_period = Some(value);
        resource
    }
    fn set_authored_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored_on = Some(value);
        resource
    }
    fn set_last_modified(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.last_modified = Some(value);
        resource
    }
    fn set_requester(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.requester = Some(value);
        resource
    }
    fn set_performer_type(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.performer_type = Some(value);
        resource
    }
    fn add_performer_type(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .performer_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_owner(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.owner = Some(value);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_reason_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn set_reason_reference(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn set_insurance(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.insurance = Some(value);
        resource
    }
    fn add_insurance(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.insurance.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_relevant_history(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.relevant_history = Some(value);
        resource
    }
    fn add_relevant_history(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .relevant_history
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_restriction(self, value: TaskRestriction) -> Self {
        let mut resource = self.clone();
        resource.restriction = Some(value);
        resource
    }
    fn set_input(self, value: Vec<TaskInput>) -> Self {
        let mut resource = self.clone();
        resource.input = Some(value);
        resource
    }
    fn add_input(self, item: TaskInput) -> Self {
        let mut resource = self.clone();
        resource.input.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_output(self, value: Vec<TaskOutput>) -> Self {
        let mut resource = self.clone();
        resource.output = Some(value);
        resource
    }
    fn add_output(self, item: TaskOutput) -> Self {
        let mut resource = self.clone();
        resource.output.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::task::TaskExistence for Task {
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
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical.is_some()
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri.is_some()
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_business_status(&self) -> bool {
        self.business_status.is_some()
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_focus(&self) -> bool {
        self.focus.is_some()
    }
    fn has_for_(&self) -> bool {
        self.for_.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_execution_period(&self) -> bool {
        self.execution_period.is_some()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_last_modified(&self) -> bool {
        self.last_modified.is_some()
    }
    fn has_requester(&self) -> bool {
        self.requester.is_some()
    }
    fn has_performer_type(&self) -> bool {
        self.performer_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_reason_code(&self) -> bool {
        self.reason_code.is_some()
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference.is_some()
    }
    fn has_insurance(&self) -> bool {
        self.insurance.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_relevant_history(&self) -> bool {
        self.relevant_history
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_restriction(&self) -> bool {
        self.restriction.is_some()
    }
    fn has_input(&self) -> bool {
        self.input.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_output(&self) -> bool {
        self.output.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Task {
    fn resource_type(&self) -> &'static str {
        "Task"
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
        Some("http://hl7.org/fhir/StructureDefinition/Task")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::task::{TaskAccessors, TaskExistence, TaskMutators};
