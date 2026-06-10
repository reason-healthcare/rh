use crate::bindings::action_cardinality_behavior::ActionCardinalityBehavior;
use crate::bindings::action_condition_kind::ActionConditionKind;
use crate::bindings::action_grouping_behavior::ActionGroupingBehavior;
use crate::bindings::action_participant_type::ActionParticipantType;
use crate::bindings::action_precheck_behavior::ActionPrecheckBehavior;
use crate::bindings::action_relationship_type::ActionRelationshipType;
use crate::bindings::action_required_behavior::ActionRequiredBehavior;
use crate::bindings::action_selection_behavior::ActionSelectionBehavior;
use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::age::Age;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::data_requirement::DataRequirement;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::timing::Timing;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// RequestOrchestration
///
/// A set of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: RequestOrchestration
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestration {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesCanonical' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesCanonical")]
    pub _instantiates_canonical: Option<Element>,
    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<StringType>>,
    /// Extension element for the 'instantiatesUri' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instantiatesUri")]
    pub _instantiates_uri: Option<Element>,
    /// Fulfills plan, proposal, or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// Request(s) replaced by this request
    pub replaces: Option<Vec<Reference>>,
    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: RequestIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// What's being requested/ordered
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-code
    pub code: Option<CodeableConcept>,
    /// Who the request orchestration is about
    pub subject: Option<Reference>,
    /// Created as part of
    pub encounter: Option<Reference>,
    /// When the request orchestration was authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Device or practitioner that authored the request orchestration
    pub author: Option<Reference>,
    /// Why the request orchestration is needed
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-reason-code
    pub reason: Option<Vec<CodeableReference>>,
    /// What goals
    pub goal: Option<Vec<Reference>>,
    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,
    /// Proposed actions, if any
    pub action: Option<Vec<RequestOrchestrationAction>>,
}
/// RequestOrchestration nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Who should perform the action
    pub participant: Option<Vec<RequestOrchestrationActionParticipant>>,
    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<RequestOrchestrationActionDynamicvalue>>,
    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<RequestOrchestrationActionRelatedaction>>,
    /// Input data requirements
    pub input: Option<Vec<RequestOrchestrationActionInput>>,
    /// Whether or not the action is applicable
    pub condition: Option<Vec<RequestOrchestrationActionCondition>>,
    /// Output data definition
    pub output: Option<Vec<RequestOrchestrationActionOutput>>,
    /// Pointer to specific item from the PlanDefinition
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
    /// Short description of the action
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
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-code
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<RelatedArtifact>>,
    /// What goals
    pub goal: Option<Vec<Reference>>,
    /// When the action should take place (dateTime)
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<DateTimeType>,
    /// When the action should take place (Age)
    #[serde(rename = "timingAge")]
    pub timing_age: Option<Age>,
    /// When the action should take place (Period)
    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,
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
    /// The target of the action
    pub resource: Option<Reference>,
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
    /// Sub action
    pub action: Option<Vec<StringType>>,
}
/// RequestOrchestrationAction nested structure for the 'input' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionInput {
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
/// RequestOrchestrationAction nested structure for the 'output' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionOutput {
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
/// RequestOrchestrationAction nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionParticipant {
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
    /// E.g. Nurse, Surgeon, Parent, etc
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
    /// Who/what is participating? (canonical)
    #[serde(rename = "actorCanonical")]
    pub actor_canonical: Option<StringType>,
    /// Who/what is participating? (Reference)
    #[serde(rename = "actorReference")]
    pub actor_reference: Option<Reference>,
}
/// RequestOrchestrationAction nested structure for the 'dynamicValue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionDynamicvalue {
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
/// RequestOrchestrationAction nested structure for the 'condition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionCondition {
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
/// RequestOrchestrationAction nested structure for the 'relatedAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestOrchestrationActionRelatedaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What action this is related to
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

impl Default for RequestOrchestration {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            based_on: Default::default(),
            replaces: Default::default(),
            group_identifier: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: RequestIntent::default(),
            _intent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            code: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            authored_on: Default::default(),
            _authored_on: Default::default(),
            author: Default::default(),
            reason: Default::default(),
            goal: Default::default(),
            note: Default::default(),
            action: Default::default(),
        }
    }
}

impl Default for RequestOrchestrationAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            participant: Default::default(),
            dynamic_value: Default::default(),
            related_action: Default::default(),
            input: Default::default(),
            condition: Default::default(),
            output: Default::default(),
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
            documentation: Default::default(),
            goal: Default::default(),
            timing_date_time: Default::default(),
            timing_age: Default::default(),
            timing_period: Default::default(),
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
            resource: Default::default(),
            definition_canonical: Default::default(),
            definition_uri: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            action: Default::default(),
        }
    }
}

impl Default for RequestOrchestrationActionInput {
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

impl Default for RequestOrchestrationActionOutput {
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

impl Default for RequestOrchestrationActionParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            type_canonical: Default::default(),
            _type_canonical: Default::default(),
            type_reference: Default::default(),
            role: Default::default(),
            function: Default::default(),
            actor_canonical: Default::default(),
            actor_reference: Default::default(),
        }
    }
}

impl Default for RequestOrchestrationActionDynamicvalue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for RequestOrchestrationActionCondition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            kind: Default::default(),
            _kind: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for RequestOrchestrationActionRelatedaction {
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

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
    rh_foundation::Invariant::new("pld-0", rh_foundation::Severity::Error, "Input data elements must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("pld-1", rh_foundation::Severity::Error, "Output data element must have a requirement or a relatedData, but not both", "requirement.exists() xor relatedData.exists()"),
    rh_foundation::Invariant::new("rqg-1", rh_foundation::Severity::Error, "Must have resource or action but not both", "resource.exists() != action.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("RequestOrchestration.action.cardinalityBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-cardinality-behavior|5.0.0").with_description("Defines behavior for an action or a group for how many times that item may be repeated."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.condition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-condition-kind|5.0.0").with_description("The kind of condition for the action."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.groupingBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-grouping-behavior|5.0.0").with_description("Defines organization behavior of a group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|5.0.0").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.precheckBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-precheck-behavior|5.0.0").with_description("Defines selection frequency behavior for an action or group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.relatedAction.endRelationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.relatedAction.relationship", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-relationship-type|5.0.0").with_description("Defines the types of relationships between actions."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.requiredBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-required-behavior|5.0.0").with_description("Defines expectations around whether an action or action group is required."),
    rh_foundation::ElementBinding::new("RequestOrchestration.action.selectionBehavior", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-selection-behavior|5.0.0").with_description("Defines selection behavior of a group."),
    rh_foundation::ElementBinding::new("RequestOrchestration.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|5.0.0").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("RequestOrchestration.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|5.0.0").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("RequestOrchestration.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|5.0.0").with_description("Codes identifying the lifecycle stage of a request."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("RequestOrchestration.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.contained", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.identifier", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.replaces", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.groupIdentifier",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.authoredOn", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("RequestOrchestration.reason", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.goal", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.note", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action", 0, None),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.linkId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.prefix",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.textEquivalent",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.priority",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.code", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.documentation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.goal", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.kind",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.condition.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.input", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.input.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.output", 0, None),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.title",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.requirement",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.output.relatedData",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.targetId",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.relationship",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.endRelationship",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.relatedAction.offset[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.timing[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.location",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.typeCanonical",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.typeReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.function",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.participant.actor[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.groupingBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.selectionBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.requiredBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.precheckBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.cardinalityBehavior",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.definition[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.transform",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.path",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "RequestOrchestration.action.dynamicValue.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("RequestOrchestration.action.action", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RequestOrchestration {
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

impl crate::traits::resource::ResourceMutators for RequestOrchestration {
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

impl crate::traits::resource::ResourceExistence for RequestOrchestration {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RequestOrchestration {
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

impl crate::traits::domain_resource::DomainResourceMutators for RequestOrchestration {
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

impl crate::traits::domain_resource::DomainResourceExistence for RequestOrchestration {
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

impl crate::traits::request_orchestration::RequestOrchestrationAccessors for RequestOrchestration {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn instantiates_canonical(&self) -> &[StringType] {
        self.instantiates_canonical.as_deref().unwrap_or(&[])
    }
    fn instantiates_uri(&self) -> &[StringType] {
        self.instantiates_uri.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_deref().unwrap_or(&[])
    }
    fn group_identifier(&self) -> Option<Identifier> {
        self.group_identifier.clone()
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> RequestIntent {
        self.intent.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn authored_on(&self) -> Option<DateTimeType> {
        self.authored_on.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn goal(&self) -> &[Reference] {
        self.goal.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn action(&self) -> &[RequestOrchestrationAction] {
        self.action.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::request_orchestration::RequestOrchestrationMutators for RequestOrchestration {
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
    fn set_instantiates_canonical(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_canonical = Some(value);
        resource
    }
    fn add_instantiates_canonical(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_canonical
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_instantiates_uri(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates_uri = Some(value);
        resource
    }
    fn add_instantiates_uri(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates_uri
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_replaces(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.replaces = Some(value);
        resource
    }
    fn add_replaces(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.replaces.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_group_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.group_identifier = Some(value);
        resource
    }
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
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
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_authored_on(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.authored_on = Some(value);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_goal(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.goal = Some(value);
        resource
    }
    fn add_goal(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.goal.get_or_insert_with(Vec::new).push(item);
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
    fn set_action(self, value: Vec<RequestOrchestrationAction>) -> Self {
        let mut resource = self.clone();
        resource.action = Some(value);
        resource
    }
    fn add_action(self, item: RequestOrchestrationAction) -> Self {
        let mut resource = self.clone();
        resource.action.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::request_orchestration::RequestOrchestrationExistence for RequestOrchestration {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_canonical(&self) -> bool {
        self.instantiates_canonical
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_instantiates_uri(&self) -> bool {
        self.instantiates_uri
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_group_identifier(&self) -> bool {
        self.group_identifier.is_some()
    }
    fn has_status(&self) -> bool {
        true
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
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_authored_on(&self) -> bool {
        self.authored_on.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_goal(&self) -> bool {
        self.goal.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_action(&self) -> bool {
        self.action.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for RequestOrchestration {
    fn resource_type(&self) -> &'static str {
        "RequestOrchestration"
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
        Some("http://hl7.org/fhir/StructureDefinition/RequestOrchestration")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::request_orchestration::{
    RequestOrchestrationAccessors, RequestOrchestrationExistence, RequestOrchestrationMutators,
};
