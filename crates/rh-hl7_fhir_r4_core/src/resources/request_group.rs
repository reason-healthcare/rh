use crate::bindings::action_cardinality_behavior::ActionCardinalityBehavior;
use crate::bindings::action_condition_kind::ActionConditionKind;
use crate::bindings::action_grouping_behavior::ActionGroupingBehavior;
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
/// RequestGroup
///
/// A group of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: RequestGroup
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGroup {
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
    pub code: Option<CodeableConcept>,
    /// Who the request group is about
    pub subject: Option<Reference>,
    /// Created as part of
    pub encounter: Option<Reference>,
    /// When the request group was authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<DateTimeType>,
    /// Extension element for the 'authoredOn' primitive field. Contains metadata and extensions.
    #[serde(rename = "_authoredOn")]
    pub _authored_on: Option<Element>,
    /// Device or practitioner that authored the request group
    pub author: Option<Reference>,
    /// Why the request group is needed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why the request group is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,
    /// Proposed actions, if any
    pub action: Option<Vec<RequestGroupAction>>,
}
/// RequestGroupAction nested structure for the 'condition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGroupActionCondition {
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
/// RequestGroupAction nested structure for the 'relatedAction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGroupActionRelatedaction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What action this is related to
    #[serde(rename = "actionId")]
    pub action_id: StringType,
    /// Extension element for the 'actionId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_actionId")]
    pub _action_id: Option<Element>,
    /// before-start | before | before-end | concurrent-with-start | concurrent | concurrent-with-end | after-start | after | after-end
    pub relationship: ActionRelationshipType,
    /// Extension element for the 'relationship' primitive field. Contains metadata and extensions.
    pub _relationship: Option<Element>,
    /// Time offset for the relationship (Duration)
    #[serde(rename = "offsetDuration")]
    pub offset_duration: Option<Duration>,
    /// Time offset for the relationship (Range)
    #[serde(rename = "offsetRange")]
    pub offset_range: Option<Range>,
}
/// RequestGroup nested structure for the 'action' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGroupAction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<RequestGroupActionRelatedaction>>,
    /// Whether or not the action is applicable
    pub condition: Option<Vec<RequestGroupActionCondition>>,
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
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<RelatedArtifact>>,
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
    /// Who should perform the action
    pub participant: Option<Vec<Reference>>,
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
    /// Sub action
    pub action: Option<Vec<StringType>>,
}

impl Default for RequestGroup {
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
            reason_code: Default::default(),
            reason_reference: Default::default(),
            note: Default::default(),
            action: Default::default(),
        }
    }
}

impl Default for RequestGroupActionCondition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            kind: Default::default(),
            _kind: Default::default(),
            expression: Default::default(),
        }
    }
}

impl Default for RequestGroupActionRelatedaction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            action_id: Default::default(),
            _action_id: Default::default(),
            relationship: Default::default(),
            _relationship: Default::default(),
            offset_duration: Default::default(),
            offset_range: Default::default(),
        }
    }
}

impl Default for RequestGroupAction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            related_action: Default::default(),
            condition: Default::default(),
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
            timing_date_time: Default::default(),
            timing_age: Default::default(),
            timing_period: Default::default(),
            timing_duration: Default::default(),
            timing_range: Default::default(),
            timing_timing: Default::default(),
            participant: Default::default(),
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
            action: Default::default(),
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
    rh_foundation::Invariant::new("rqg-1", rh_foundation::Severity::Error, "Must have resource or action but not both", "resource.exists() != action.exists()").with_xpath("exists(f:resource) != exists(f:action)"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for RequestGroup {
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

impl crate::traits::resource::ResourceMutators for RequestGroup {
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

impl crate::traits::resource::ResourceExistence for RequestGroup {
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

impl crate::traits::domain_resource::DomainResourceAccessors for RequestGroup {
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

impl crate::traits::domain_resource::DomainResourceMutators for RequestGroup {
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

impl crate::traits::domain_resource::DomainResourceExistence for RequestGroup {
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

impl crate::traits::request_group::RequestGroupAccessors for RequestGroup {
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
    fn reason_code(&self) -> &[CodeableConcept] {
        self.reason_code.as_deref().unwrap_or(&[])
    }
    fn reason_reference(&self) -> &[Reference] {
        self.reason_reference.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn action(&self) -> &[RequestGroupAction] {
        self.action.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::request_group::RequestGroupMutators for RequestGroup {
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
    fn set_reason_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.reason_code = Some(value);
        resource
    }
    fn add_reason_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.reason_code.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_reason_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.reason_reference = Some(value);
        resource
    }
    fn add_reason_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .reason_reference
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_action(self, value: Vec<RequestGroupAction>) -> Self {
        let mut resource = self.clone();
        resource.action = Some(value);
        resource
    }
    fn add_action(self, item: RequestGroupAction) -> Self {
        let mut resource = self.clone();
        resource.action.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::request_group::RequestGroupExistence for RequestGroup {
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
    fn has_reason_code(&self) -> bool {
        self.reason_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_reason_reference(&self) -> bool {
        self.reason_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_action(&self) -> bool {
        self.action.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for RequestGroup {
    fn resource_type(&self) -> &'static str {
        "RequestGroup"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/RequestGroup")
    }
}
