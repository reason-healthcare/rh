use crate::bindings::action_cardinality_behavior::ActionCardinalityBehavior;
use crate::bindings::action_condition_kind::ActionConditionKind;
use crate::bindings::action_grouping_behavior::ActionGroupingBehavior;
use crate::bindings::action_participant_type::ActionParticipantType;
use crate::bindings::action_precheck_behavior::ActionPrecheckBehavior;
use crate::bindings::action_relationship_type::ActionRelationshipType;
use crate::bindings::action_required_behavior::ActionRequiredBehavior;
use crate::bindings::action_selection_behavior::ActionSelectionBehavior;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::request_priority::RequestPriority;
use crate::datatypes::age::Age;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::timing::Timing;
use crate::datatypes::trigger_definition::TriggerDefinition;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// PlanDefinition
///
/// This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact. The resource is general enough to support the description of a broad range of clinical and non-clinical artifacts such as clinical decision support rules, order sets, protocols, and drug quality specifications.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the plan definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the plan definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this plan definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this plan definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate title of the plan definition
    pub subtitle: Option<StringType>,
    /// Extension element for the 'subtitle' primitive field. Contains metadata and extensions.
    pub _subtitle: Option<Element>,
    /// order-set | clinical-protocol | eca-rule | workflow-definition
    ///
    /// Binding: extensible (The type of PlanDefinition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/plan-definition-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Type of individual the plan definition is focused on (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// Type of individual the plan definition is focused on (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Type of individual the plan definition is focused on (canonical)
    #[serde(rename = "subjectCanonical")]
    pub subject_canonical: Option<StringType>,
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
    /// Natural language description of the plan definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for plan definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this plan definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Describes the clinical usage of the plan
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
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
    /// When the plan definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the plan definition was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the plan definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment
    ///
    /// Binding: example (High-level categorization of the definition, used for searching, sorting, and filtering.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Logic used by the plan definition
    pub library: Option<Vec<StringType>>,
    /// Extension element for the 'library' primitive field. Contains metadata and extensions.
    pub _library: Option<Element>,
    /// What the plan is trying to accomplish
    pub goal: Option<Vec<PlanDefinitionGoal>>,
    /// Actors within the plan
    pub actor: Option<Vec<PlanDefinitionActor>>,
    /// Action defined by the plan
    pub action: Option<Vec<PlanDefinitionAction>>,
    /// Preconditions for service (boolean)
    #[serde(rename = "asNeededBoolean")]
    pub as_needed_boolean: Option<BooleanType>,
    /// Preconditions for service (CodeableConcept)
    #[serde(rename = "asNeededCodeableConcept")]
    pub as_needed_codeable_concept: Option<CodeableConcept>,
}
/// PlanDefinition nested structure for the 'goal' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionGoal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Target outcome for the goal
    pub target: Option<Vec<PlanDefinitionGoalTarget>>,
    /// E.g. Treatment, dietary, behavioral
    ///
    /// Binding: example (Example codes for grouping goals for filtering or presentation.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/goal-category
    pub category: Option<CodeableConcept>,
    /// Code or text describing the goal
    ///
    /// Binding: example (Describes goals that can be achieved.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub description: CodeableConcept,
    /// high-priority | medium-priority | low-priority
    ///
    /// Binding: preferred (Indicates the level of importance associated with reaching or sustaining a goal.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/goal-priority
    pub priority: Option<CodeableConcept>,
    /// When goal pursuit begins
    ///
    /// Binding: example (Identifies the types of events that might trigger the start of a goal.)
    ///
    /// Available values:
    /// - `32485007`: Admission to hospital
    /// - `308283009`: Discharge from hospital
    /// - `442137000`: Completion time of procedure
    /// - `386216000`: Childbirth
    pub start: Option<CodeableConcept>,
    /// What does the goal address
    ///
    /// Binding: example (Identifies problems, conditions, issues, or concerns that goals may address.)
    ///
    /// Available values:
    /// - `160245001`: No current problems or disability
    pub addresses: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the goal
    pub documentation: Option<Vec<RelatedArtifact>>,
}
/// PlanDefinitionAction nested structure for the 'relatedAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionRelatedaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What action is this related to
    #[serde(rename = "targetId")]
    pub target_id: StringType,
    /// Extension element for the 'targetId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetId")]
    pub _target_id: Option<Element>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: ActionRelationshipType,
    /// Extension element for the 'relationship' primitive field. Contains metadata and extensions.
    pub _relationship: Option<Element>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[serde(rename = "endRelationship")]
    pub end_relationship: Option<ActionRelationshipType>,
    /// Extension element for the 'endRelationship' primitive field. Contains metadata and extensions.
    #[serde(rename = "_endRelationship")]
    pub _end_relationship: Option<Element>,
    /// Time offset for the relationship (Duration)
    #[serde(rename = "offsetDuration")]
    pub offset_duration: Option<Duration>,
    /// Time offset for the relationship (Range)
    #[serde(rename = "offsetRange")]
    pub offset_range: Option<Range>,
}
/// PlanDefinitionAction nested structure for the 'dynamicValue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionDynamicvalue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The path to the element to be set dynamically
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// An expression that provides the dynamic value for the customization
    pub expression: Option<Expression>,
}
/// PlanDefinitionAction nested structure for the 'output' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionOutput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// User-visible title
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// What data is provided
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[serde(rename = "relatedData")]
    pub related_data: Option<StringType>,
    /// Extension element for the 'relatedData' primitive field. Contains metadata and extensions.
    #[serde(rename = "_relatedData")]
    pub _related_data: Option<Element>,
}
/// PlanDefinitionGoal nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionGoalTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The parameter whose value is to be tracked
    ///
    /// Binding: example (Identifies types of parameters that can be tracked to determine goal achievement.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/observation-codes
    pub measure: Option<CodeableConcept>,
    /// The target value to be achieved (Quantity)
    #[serde(rename = "detailQuantity")]
    pub detail_quantity: Option<Quantity>,
    /// The target value to be achieved (Range)
    #[serde(rename = "detailRange")]
    pub detail_range: Option<Range>,
    /// The target value to be achieved (CodeableConcept)
    #[serde(rename = "detailCodeableConcept")]
    pub detail_codeable_concept: Option<CodeableConcept>,
    /// The target value to be achieved (string)
    #[serde(rename = "detailString")]
    pub detail_string: Option<StringType>,
    /// The target value to be achieved (boolean)
    #[serde(rename = "detailBoolean")]
    pub detail_boolean: Option<BooleanType>,
    /// The target value to be achieved (integer)
    #[serde(rename = "detailInteger")]
    pub detail_integer: Option<IntegerType>,
    /// The target value to be achieved (Ratio)
    #[serde(rename = "detailRatio")]
    pub detail_ratio: Option<Ratio>,
    /// Reach goal within
    pub due: Option<Duration>,
}
/// PlanDefinitionAction nested structure for the 'condition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionCondition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// applicability | start | stop
    pub kind: ActionConditionKind,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// Boolean-valued expression
    pub expression: Option<Expression>,
}
/// PlanDefinition nested structure for the 'actor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who or what can be this actor
    pub option: Vec<PlanDefinitionActorOption>,
    /// User-visible title
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Describes the actor
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// PlanDefinition nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Output data definition
    pub output: Option<Vec<PlanDefinitionActionOutput>>,
    /// Input data requirements
    pub input: Option<Vec<PlanDefinitionActionInput>>,
    /// Who should participate in the action
    pub participant: Option<Vec<PlanDefinitionActionParticipant>>,
    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<PlanDefinitionActionRelatedaction>>,
    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<PlanDefinitionActionDynamicvalue>>,
    /// Whether or not the action is applicable
    pub condition: Option<Vec<PlanDefinitionActionCondition>>,
    /// Unique id for the action in the PlanDefinition
    #[serde(rename = "linkId")]
    pub link_id: Option<StringType>,
    /// Extension element for the 'linkId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_linkId")]
    pub _link_id: Option<Element>,
    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<StringType>,
    /// Extension element for the 'prefix' primitive field. Contains metadata and extensions.
    pub _prefix: Option<Element>,
    /// User-visible title
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Brief description of the action
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    #[serde(rename = "textEquivalent")]
    pub text_equivalent: Option<StringType>,
    /// Extension element for the 'textEquivalent' primitive field. Contains metadata and extensions.
    #[serde(rename = "_textEquivalent")]
    pub _text_equivalent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// Code representing the meaning of the action or sub-actions
    ///
    /// Binding: example (Provides examples of actions to be performed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-code
    pub code: Option<CodeableConcept>,
    /// Why the action should be performed
    ///
    /// Binding: example (Provides examples of reasons for actions to be performed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-reason-code
    pub reason: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<RelatedArtifact>>,
    /// What goals this action supports
    #[serde(rename = "goalId")]
    pub goal_id: Option<Vec<StringType>>,
    /// Extension element for the 'goalId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_goalId")]
    pub _goal_id: Option<Element>,
    /// Type of individual the action is focused on (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// Type of individual the action is focused on (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Type of individual the action is focused on (canonical)
    #[serde(rename = "subjectCanonical")]
    pub subject_canonical: Option<StringType>,
    /// When the action should be triggered
    pub trigger: Option<Vec<TriggerDefinition>>,
    /// When the action should take place (Age)
    #[serde(rename = "timingAge")]
    pub timing_age: Option<Age>,
    /// When the action should take place (Duration)
    #[serde(rename = "timingDuration")]
    pub timing_duration: Option<Duration>,
    /// When the action should take place (Range)
    #[serde(rename = "timingRange")]
    pub timing_range: Option<Range>,
    /// When the action should take place (Timing)
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,
    /// Where it should happen
    pub location: Option<CodeableReference>,
    /// create | update | remove | fire-event
    ///
    /// Binding: extensible (The type of action to be performed.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// visual-group | logical-group | sentence-group
    #[serde(rename = "groupingBehavior")]
    pub grouping_behavior: Option<ActionGroupingBehavior>,
    /// Extension element for the 'groupingBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_groupingBehavior")]
    pub _grouping_behavior: Option<Element>,
    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    #[serde(rename = "selectionBehavior")]
    pub selection_behavior: Option<ActionSelectionBehavior>,
    /// Extension element for the 'selectionBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_selectionBehavior")]
    pub _selection_behavior: Option<Element>,
    /// must | could | must-unless-documented
    #[serde(rename = "requiredBehavior")]
    pub required_behavior: Option<ActionRequiredBehavior>,
    /// Extension element for the 'requiredBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requiredBehavior")]
    pub _required_behavior: Option<Element>,
    /// yes | no
    #[serde(rename = "precheckBehavior")]
    pub precheck_behavior: Option<ActionPrecheckBehavior>,
    /// Extension element for the 'precheckBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_precheckBehavior")]
    pub _precheck_behavior: Option<Element>,
    /// single | multiple
    #[serde(rename = "cardinalityBehavior")]
    pub cardinality_behavior: Option<ActionCardinalityBehavior>,
    /// Extension element for the 'cardinalityBehavior' primitive field. Contains metadata and extensions.
    #[serde(rename = "_cardinalityBehavior")]
    pub _cardinality_behavior: Option<Element>,
    /// Description of the activity to be performed (canonical)
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<StringType>,
    /// Description of the activity to be performed (uri)
    #[serde(rename = "definitionUri")]
    pub definition_uri: Option<StringType>,
    /// Transform to apply the template
    pub transform: Option<StringType>,
    /// Extension element for the 'transform' primitive field. Contains metadata and extensions.
    pub _transform: Option<Element>,
    /// A sub-action
    pub action: Option<Vec<StringType>>,
}
/// PlanDefinitionActor nested structure for the 'option' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActorOption {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[serde(rename = "type")]
    pub type_: Option<ActionParticipantType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Who or what can participate
    #[serde(rename = "typeCanonical")]
    pub type_canonical: Option<StringType>,
    /// Extension element for the 'typeCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_typeCanonical")]
    pub _type_canonical: Option<Element>,
    /// Who or what can participate
    #[serde(rename = "typeReference")]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/action-participant-role
    pub role: Option<CodeableConcept>,
}
/// PlanDefinitionAction nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What actor
    #[serde(rename = "actorId")]
    pub actor_id: Option<StringType>,
    /// Extension element for the 'actorId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_actorId")]
    pub _actor_id: Option<Element>,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[serde(rename = "type")]
    pub type_: Option<ActionParticipantType>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Who or what can participate
    #[serde(rename = "typeCanonical")]
    pub type_canonical: Option<StringType>,
    /// Extension element for the 'typeCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_typeCanonical")]
    pub _type_canonical: Option<Element>,
    /// Who or what can participate
    #[serde(rename = "typeReference")]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent
    ///
    /// Binding: example (Defines roles played by participants for the action.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/action-participant-role
    pub role: Option<CodeableConcept>,
    /// E.g. Author, Reviewer, Witness, etc
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-participant-function
    pub function: Option<CodeableConcept>,
}
/// PlanDefinitionAction nested structure for the 'input' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDefinitionActionInput {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// User-visible title
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// What data is provided
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[serde(rename = "relatedData")]
    pub related_data: Option<StringType>,
    /// Extension element for the 'relatedData' primitive field. Contains metadata and extensions.
    #[serde(rename = "_relatedData")]
    pub _related_data: Option<Element>,
}

impl Default for PlanDefinition {
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
            subtitle: Default::default(),
            _subtitle: Default::default(),
            type_: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
            subject_canonical: Default::default(),
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
            usage: Default::default(),
            _usage: Default::default(),
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
            library: Default::default(),
            _library: Default::default(),
            goal: Default::default(),
            actor: Default::default(),
            action: Default::default(),
            as_needed_boolean: Default::default(),
            as_needed_codeable_concept: Default::default(),
        }
    }
}

impl Default for PlanDefinitionGoal {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target: Default::default(),
            category: Default::default(),
            description: CodeableConcept::default(),
            priority: Default::default(),
            start: Default::default(),
            addresses: Default::default(),
            documentation: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionRelatedaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target_id: Default::default(),
            _target_id: Default::default(),
            relationship: Default::default(),
            _relationship: Default::default(),
            end_relationship: Default::default(),
            _end_relationship: Default::default(),
            offset_duration: Default::default(),
            offset_range: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionDynamicvalue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionOutput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            requirement: Default::default(),
            related_data: Default::default(),
            _related_data: Default::default(),
        }
    }
}

impl Default for PlanDefinitionGoalTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            measure: Default::default(),
            detail_quantity: Default::default(),
            detail_range: Default::default(),
            detail_codeable_concept: Default::default(),
            detail_string: Default::default(),
            detail_boolean: Default::default(),
            detail_integer: Default::default(),
            detail_ratio: Default::default(),
            due: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionCondition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            kind: Default::default(),
            _kind: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            option: Vec::new(),
            title: Default::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for PlanDefinitionAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            output: Default::default(),
            input: Default::default(),
            participant: Default::default(),
            related_action: Default::default(),
            dynamic_value: Default::default(),
            condition: Default::default(),
            link_id: Default::default(),
            _link_id: Default::default(),
            prefix: Default::default(),
            _prefix: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            text_equivalent: Default::default(),
            _text_equivalent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            code: Default::default(),
            reason: Default::default(),
            documentation: Default::default(),
            goal_id: Default::default(),
            _goal_id: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
            subject_canonical: Default::default(),
            trigger: Default::default(),
            timing_age: Default::default(),
            timing_duration: Default::default(),
            timing_range: Default::default(),
            timing_timing: Default::default(),
            location: Default::default(),
            type_: Default::default(),
            grouping_behavior: Default::default(),
            _grouping_behavior: Default::default(),
            selection_behavior: Default::default(),
            _selection_behavior: Default::default(),
            required_behavior: Default::default(),
            _required_behavior: Default::default(),
            precheck_behavior: Default::default(),
            _precheck_behavior: Default::default(),
            cardinality_behavior: Default::default(),
            _cardinality_behavior: Default::default(),
            definition_canonical: Default::default(),
            definition_uri: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            action: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActorOption {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            type_canonical: Default::default(),
            _type_canonical: Default::default(),
            type_reference: Default::default(),
            role: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            actor_id: Default::default(),
            _actor_id: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            type_canonical: Default::default(),
            _type_canonical: Default::default(),
            type_reference: Default::default(),
            role: Default::default(),
            function: Default::default(),
        }
    }
}

impl Default for PlanDefinitionActionInput {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            requirement: Default::default(),
            related_data: Default::default(),
            _related_data: Default::default(),
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
    rh_foundation::Invariant::new("pld-0", rh_foundation::Severity::Error, "Input data elements must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-1", rh_foundation::Severity::Error, "Output data element must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-3", rh_foundation::Severity::Warning, "goalid should reference the id of a goal definition", "%context.repeat(action).where((goalId in %context.goal.id).not()).exists().not()"),
    rh_foundation::Invariant::new("pld-4", rh_foundation::Severity::Warning, "targetId should reference the id of an action", "%context.repeat(action).relatedAction.where((targetId in %context.repeat(action).id).not()).exists().not()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("PlanDefinition.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|5.0.0").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|5.0.0").with_description("Defines the kinds of conditions that can appear on actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|5.0.0").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|5.0.0").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.relatedAction.endRelationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|5.0.0").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("PlanDefinition.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|5.0.0").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("PlanDefinition.actor.option.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("PlanDefinition.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("PlanDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("PlanDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.category", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.start", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.addresses", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.documentation", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.measure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.goal.target.detail[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.goal.target.due", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option", 1, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.actor.option.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.actor.option.role", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.linkId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.prefix", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.reason", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.documentation", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.goalId", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.trigger", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.condition", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.kind",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.condition.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.input.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.input.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.output", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.output.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.output.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.relatedAction", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.targetId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.endRelationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.participant", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.actorId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.groupingBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.selectionBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.requiredBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.precheckBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.cardinalityBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.definition[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.transform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.dynamicValue", 0, None),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.path",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "PlanDefinition.action.dynamicValue.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("PlanDefinition.action.action", 0, None),
            rh_foundation::ElementCardinality::new("PlanDefinition.asNeeded[x]", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for PlanDefinition {
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

impl crate::traits::resource::ResourceMutators for PlanDefinition {
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

impl crate::traits::resource::ResourceExistence for PlanDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for PlanDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for PlanDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for PlanDefinition {
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

impl crate::traits::plan_definition::PlanDefinitionAccessors for PlanDefinition {
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
    fn subtitle(&self) -> Option<StringType> {
        self.subtitle.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
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
    fn usage(&self) -> Option<StringType> {
        self.usage.clone()
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
    fn library(&self) -> &[StringType] {
        self.library.as_deref().unwrap_or(&[])
    }
    fn goal(&self) -> &[PlanDefinitionGoal] {
        self.goal.as_deref().unwrap_or(&[])
    }
    fn actor(&self) -> &[PlanDefinitionActor] {
        self.actor.as_deref().unwrap_or(&[])
    }
    fn action(&self) -> &[PlanDefinitionAction] {
        self.action.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::plan_definition::PlanDefinitionMutators for PlanDefinition {
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
    fn set_subtitle(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.subtitle = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
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
    fn set_usage(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage = Some(value);
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
    fn set_library(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.library = Some(value);
        resource
    }
    fn add_library(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.library.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_goal(self, value: Vec<PlanDefinitionGoal>) -> Self {
        let mut resource = self.clone();
        resource.goal = Some(value);
        resource
    }
    fn add_goal(self, item: PlanDefinitionGoal) -> Self {
        let mut resource = self.clone();
        resource.goal.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_actor(self, value: Vec<PlanDefinitionActor>) -> Self {
        let mut resource = self.clone();
        resource.actor = Some(value);
        resource
    }
    fn add_actor(self, item: PlanDefinitionActor) -> Self {
        let mut resource = self.clone();
        resource.actor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_action(self, value: Vec<PlanDefinitionAction>) -> Self {
        let mut resource = self.clone();
        resource.action = Some(value);
        resource
    }
    fn add_action(self, item: PlanDefinitionAction) -> Self {
        let mut resource = self.clone();
        resource.action.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::plan_definition::PlanDefinitionExistence for PlanDefinition {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_as_needed(&self) -> bool {
        self.as_needed_boolean.is_some() || self.as_needed_codeable_concept.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject_codeable_concept.is_some()
            || self.subject_reference.is_some()
            || self.subject_canonical.is_some()
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
    fn has_subtitle(&self) -> bool {
        self.subtitle.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
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
    fn has_usage(&self) -> bool {
        self.usage.is_some()
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
    fn has_library(&self) -> bool {
        self.library.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_goal(&self) -> bool {
        self.goal.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_actor(&self) -> bool {
        self.actor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_action(&self) -> bool {
        self.action.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for PlanDefinition {
    fn resource_type(&self) -> &'static str {
        "PlanDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/PlanDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::plan_definition::{
    PlanDefinitionAccessors, PlanDefinitionExistence, PlanDefinitionMutators,
};
