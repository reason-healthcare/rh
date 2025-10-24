use crate::bindings::goal_status::GoalStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::ratio::Ratio;
use crate::datatypes::reference::Reference;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Goal
///
/// Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Goal
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Goal
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this goal
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    #[serde(rename = "lifecycleStatus")]
    pub lifecycle_status: GoalStatus,
    /// Extension element for the 'lifecycleStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lifecycleStatus")]
    pub _lifecycle_status: Option<Element>,
    /// in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    ///
    /// Binding: preferred (Indicates the progression, or lack thereof, towards the goal against the target.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/goal-achievement
    #[serde(rename = "achievementStatus")]
    pub achievement_status: Option<CodeableConcept>,
    /// E.g. Treatment, dietary, behavioral, etc.
    ///
    /// Binding: example (Codes for grouping and sorting goals.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/goal-category
    pub category: Option<Vec<CodeableConcept>>,
    /// high-priority | medium-priority | low-priority
    ///
    /// Binding: preferred (The level of importance associated with a goal.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/goal-priority
    pub priority: Option<CodeableConcept>,
    /// Code or text describing goal
    ///
    /// Binding: example (Codes providing the details of a particular goal.  This will generally be system or implementation guide-specific.  In many systems, only the text element will be used.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    pub description: CodeableConcept,
    /// Who this goal is intended for
    pub subject: Reference,
    /// When goal pursuit begins (date)
    #[serde(rename = "startDate")]
    pub start_date: Option<DateType>,
    /// When goal pursuit begins (CodeableConcept)
    #[serde(rename = "startCodeableConcept")]
    pub start_codeable_concept: Option<CodeableConcept>,
    /// Target outcome for the goal
    pub target: Option<Vec<GoalTarget>>,
    /// When goal status took effect
    #[serde(rename = "statusDate")]
    pub status_date: Option<DateType>,
    /// Extension element for the 'statusDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusDate")]
    pub _status_date: Option<Element>,
    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<StringType>,
    /// Extension element for the 'statusReason' primitive field. Contains metadata and extensions.
    #[serde(rename = "_statusReason")]
    pub _status_reason: Option<Element>,
    /// Who's responsible for creating Goal?
    #[serde(rename = "expressedBy")]
    pub expressed_by: Option<Reference>,
    /// Issues addressed by this goal
    pub addresses: Option<Vec<Reference>>,
    /// Comments about the goal
    pub note: Option<Vec<Annotation>>,
    /// What result was achieved regarding the goal?
    ///
    /// Binding: example (The result of the goal; e.g. "25% increase in shoulder mobility", "Anxiety reduced to moderate levels".  "15 kg weight loss sustained over 6 months".)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    #[serde(rename = "outcomeCode")]
    pub outcome_code: Option<Vec<CodeableConcept>>,
    /// Observation that resulted from goal
    #[serde(rename = "outcomeReference")]
    pub outcome_reference: Option<Vec<Reference>>,
}
/// Goal nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The parameter whose value is being tracked
    ///
    /// Binding: example (Codes to identify the value being tracked, e.g. body weight, blood pressure, or hemoglobin A1c level.)
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
    /// Reach goal on or before (date)
    #[serde(rename = "dueDate")]
    pub due_date: Option<DateType>,
    /// Reach goal on or before (Duration)
    #[serde(rename = "dueDuration")]
    pub due_duration: Option<Duration>,
}
/// reason rejected
///
/// The reason the goal was not accepted. Applies only if the status of the goal is rejected.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-reasonRejected
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalReasonRejected {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Goal acceptance
///
/// Information about the acceptance and relative priority assigned to the goal by the patient, practitioners and other stake-holders.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-acceptance
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAcceptance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// related goal
///
/// Establishes a relationship between this goal and other goals.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/goal-relationship
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRelationship {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Goal {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            lifecycle_status: GoalStatus::default(),
            _lifecycle_status: Default::default(),
            achievement_status: Default::default(),
            category: Default::default(),
            priority: Default::default(),
            description: CodeableConcept::default(),
            subject: Reference::default(),
            start_date: Default::default(),
            start_codeable_concept: Default::default(),
            target: Default::default(),
            status_date: Default::default(),
            _status_date: Default::default(),
            status_reason: Default::default(),
            _status_reason: Default::default(),
            expressed_by: Default::default(),
            addresses: Default::default(),
            note: Default::default(),
            outcome_code: Default::default(),
            outcome_reference: Default::default(),
        }
    }
}

impl Default for GoalTarget {
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
            due_date: Default::default(),
            due_duration: Default::default(),
        }
    }
}

impl Default for GoalReasonRejected {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for GoalAcceptance {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for GoalRelationship {
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
    rh_foundation::Invariant::new("gol-1", rh_foundation::Severity::Error, "Goal.target.measure is required if Goal.target.detail is populated", "(detail.exists() and measure.exists()) or detail.exists().not()").with_xpath("(exists(f:*[starts-with(local-name(.), 'detail')]) and exists(f:measure)) or not(exists(f:*[starts-with(local-name(.), 'detail')]))"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Goal {
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

impl crate::traits::resource::ResourceMutators for Goal {
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

impl crate::traits::resource::ResourceExistence for Goal {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Goal {
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

impl crate::traits::domain_resource::DomainResourceMutators for Goal {
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

impl crate::traits::domain_resource::DomainResourceExistence for Goal {
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

impl crate::traits::goal::GoalAccessors for Goal {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn lifecycle_status(&self) -> GoalStatus {
        self.lifecycle_status.clone()
    }
    fn achievement_status(&self) -> Option<CodeableConcept> {
        self.achievement_status.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn description(&self) -> CodeableConcept {
        self.description.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn target(&self) -> &[GoalTarget] {
        self.target.as_deref().unwrap_or(&[])
    }
    fn status_date(&self) -> Option<DateType> {
        self.status_date.clone()
    }
    fn status_reason(&self) -> Option<StringType> {
        self.status_reason.clone()
    }
    fn expressed_by(&self) -> Option<Reference> {
        self.expressed_by.clone()
    }
    fn addresses(&self) -> &[Reference] {
        self.addresses.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn outcome_code(&self) -> &[CodeableConcept] {
        self.outcome_code.as_deref().unwrap_or(&[])
    }
    fn outcome_reference(&self) -> &[Reference] {
        self.outcome_reference.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::goal::GoalMutators for Goal {
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
    fn set_lifecycle_status(self, value: GoalStatus) -> Self {
        let mut resource = self.clone();
        resource.lifecycle_status = value;
        resource
    }
    fn set_achievement_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.achievement_status = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_description(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.description = value;
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_target(self, value: Vec<GoalTarget>) -> Self {
        let mut resource = self.clone();
        resource.target = Some(value);
        resource
    }
    fn add_target(self, item: GoalTarget) -> Self {
        let mut resource = self.clone();
        resource.target.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_date = Some(value);
        resource
    }
    fn set_status_reason(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.status_reason = Some(value);
        resource
    }
    fn set_expressed_by(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.expressed_by = Some(value);
        resource
    }
    fn set_addresses(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.addresses = Some(value);
        resource
    }
    fn add_addresses(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.addresses.get_or_insert_with(Vec::new).push(item);
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
    fn set_outcome_code(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.outcome_code = Some(value);
        resource
    }
    fn add_outcome_code(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .outcome_code
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_outcome_reference(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.outcome_reference = Some(value);
        resource
    }
    fn add_outcome_reference(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .outcome_reference
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::goal::GoalExistence for Goal {
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
    fn has_start(&self) -> bool {
        self.start_date.is_some() || self.start_codeable_concept.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_lifecycle_status(&self) -> bool {
        true
    }
    fn has_achievement_status(&self) -> bool {
        self.achievement_status.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_description(&self) -> bool {
        true
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_target(&self) -> bool {
        self.target.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status_date(&self) -> bool {
        self.status_date.is_some()
    }
    fn has_status_reason(&self) -> bool {
        self.status_reason.is_some()
    }
    fn has_expressed_by(&self) -> bool {
        self.expressed_by.is_some()
    }
    fn has_addresses(&self) -> bool {
        self.addresses.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_outcome_code(&self) -> bool {
        self.outcome_code.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_outcome_reference(&self) -> bool {
        self.outcome_reference
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Goal {
    fn resource_type(&self) -> &'static str {
        "Goal"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/Goal")
    }
}
