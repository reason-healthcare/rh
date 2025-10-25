use crate::bindings::care_plan_activity_kind::CarePlanActivityKind;
use crate::bindings::care_plan_activity_status::CarePlanActivityStatus;
use crate::bindings::care_plan_intent::CarePlanIntent;
use crate::bindings::request_status::RequestStatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::reference::Reference;
use crate::datatypes::timing::Timing;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CarePlan
///
/// Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CarePlan
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CarePlan
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlan {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this plan
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
    /// Fulfills CarePlan
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// CarePlan replaced by this CarePlan
    pub replaces: Option<Vec<Reference>>,
    /// Part of referenced CarePlan
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Reference>>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: RequestStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// proposal | plan | order | option
    pub intent: CarePlanIntent,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// Type of plan
    ///
    /// Binding: example (Identifies what "kind" of plan this is to support differentiation between multiple co-existing plans; e.g. "Home health", "psychiatric", "asthma", "disease management", etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/care-plan-category
    pub category: Option<Vec<CodeableConcept>>,
    /// Human-friendly name for the care plan
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Summary of nature of plan
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Who the care plan is for
    pub subject: Reference,
    /// Encounter created as part of
    pub encounter: Option<Reference>,
    /// Time period plan covers
    pub period: Option<Period>,
    /// Date record was first recorded
    pub created: Option<DateTimeType>,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Who is the designated responsible party
    pub author: Option<Reference>,
    /// Who provided the content of the care plan
    pub contributor: Option<Vec<Reference>>,
    /// Who's involved in plan?
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<Reference>>,
    /// Health issues this plan addresses
    pub addresses: Option<Vec<Reference>>,
    /// Information considered as part of plan
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Desired outcome of plan
    pub goal: Option<Vec<Reference>>,
    /// Action to occur as part of plan
    pub activity: Option<Vec<CarePlanActivity>>,
    /// Comments about the plan
    pub note: Option<Vec<Annotation>>,
}
/// CarePlanActivity nested structure for the 'detail' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlanActivityDetail {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Appointment | CommunicationRequest | DeviceRequest | MedicationRequest | NutritionOrder | Task | ServiceRequest | VisionPrescription
    pub kind: Option<CarePlanActivityKind>,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
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
    /// Detail type of activity
    ///
    /// Binding: example (Detailed description of the type of activity; e.g. What lab test, what procedure, what kind of encounter.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableConcept>,
    /// Why activity should be done or why activity was prohibited
    ///
    /// Binding: example (Identifies why a care plan activity is needed.  Can include any health condition codes as well as such concepts as "general wellness", prophylaxis, surgical preparation, etc.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-findings
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,
    /// Why activity is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Reference>>,
    /// Goals this activity relates to
    pub goal: Option<Vec<Reference>>,
    /// not-started | scheduled | in-progress | on-hold | completed | cancelled | stopped | unknown | entered-in-error
    pub status: CarePlanActivityStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,
    /// If true, activity is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// When activity is to occur (Timing)
    #[serde(rename = "scheduledTiming")]
    pub scheduled_timing: Option<Timing>,
    /// When activity is to occur (Period)
    #[serde(rename = "scheduledPeriod")]
    pub scheduled_period: Option<Period>,
    /// When activity is to occur (string)
    #[serde(rename = "scheduledString")]
    pub scheduled_string: Option<StringType>,
    /// Where it should happen
    pub location: Option<Reference>,
    /// Who will be responsible?
    pub performer: Option<Vec<Reference>>,
    /// What is to be administered/supplied (CodeableConcept)
    #[serde(rename = "productCodeableConcept")]
    pub product_codeable_concept: Option<CodeableConcept>,
    /// What is to be administered/supplied (Reference)
    #[serde(rename = "productReference")]
    pub product_reference: Option<Reference>,
    /// How to consume/day?
    #[serde(rename = "dailyAmount")]
    pub daily_amount: Option<Quantity>,
    /// How much to administer/supply/consume
    pub quantity: Option<Quantity>,
    /// Extra info describing activity to perform
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// CarePlan nested structure for the 'activity' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarePlanActivity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// In-line definition of activity
    pub detail: Option<CarePlanActivityDetail>,
    /// Results of the activity
    ///
    /// Binding: example (Identifies the results of the activity.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/care-plan-activity-outcome
    #[serde(rename = "outcomeCodeableConcept")]
    pub outcome_codeable_concept: Option<Vec<CodeableConcept>>,
    /// Appointment, Encounter, Procedure, etc.
    #[serde(rename = "outcomeReference")]
    pub outcome_reference: Option<Vec<Reference>>,
    /// Comments about the activity status/progress
    pub progress: Option<Vec<Annotation>>,
    /// Activity details defined in specific resource
    pub reference: Option<Reference>,
}

impl Default for CarePlan {
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
            part_of: Default::default(),
            status: RequestStatus::default(),
            _status: Default::default(),
            intent: CarePlanIntent::default(),
            _intent: Default::default(),
            category: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            subject: Reference::default(),
            encounter: Default::default(),
            period: Default::default(),
            created: Default::default(),
            _created: Default::default(),
            author: Default::default(),
            contributor: Default::default(),
            care_team: Default::default(),
            addresses: Default::default(),
            supporting_info: Default::default(),
            goal: Default::default(),
            activity: Default::default(),
            note: Default::default(),
        }
    }
}

impl Default for CarePlanActivityDetail {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            kind: Default::default(),
            _kind: Default::default(),
            instantiates_canonical: Default::default(),
            _instantiates_canonical: Default::default(),
            instantiates_uri: Default::default(),
            _instantiates_uri: Default::default(),
            code: Default::default(),
            reason_code: Default::default(),
            reason_reference: Default::default(),
            goal: Default::default(),
            status: Default::default(),
            _status: Default::default(),
            status_reason: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            scheduled_timing: Default::default(),
            scheduled_period: Default::default(),
            scheduled_string: Default::default(),
            location: Default::default(),
            performer: Default::default(),
            product_codeable_concept: Default::default(),
            product_reference: Default::default(),
            daily_amount: Default::default(),
            quantity: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for CarePlanActivity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            detail: Default::default(),
            outcome_codeable_concept: Default::default(),
            outcome_reference: Default::default(),
            progress: Default::default(),
            reference: Default::default(),
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
    rh_foundation::Invariant::new("cpl-3", rh_foundation::Severity::Error, "Provide a reference or detail, not both", "detail.empty() or reference.empty()").with_xpath("not(exists(f:detail)) or not(exists(f:reference))"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
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
    rh_foundation::ElementBinding::new("CarePlan.activity.detail.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/care-plan-activity-kind|4.0.1").with_description("Resource types defined as part of FHIR that can be represented as in-line definitions of a care plan activity."),
    rh_foundation::ElementBinding::new("CarePlan.activity.detail.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/care-plan-activity-status|4.0.1").with_description("Codes that reflect the current state of a care plan activity within its overall life cycle."),
    rh_foundation::ElementBinding::new("CarePlan.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/care-plan-intent|4.0.1").with_description("Codes indicating the degree of authority/intentionality associated with a care plan."),
    rh_foundation::ElementBinding::new("CarePlan.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-status|4.0.1").with_description("Indicates whether the plan is currently being acted upon, represents future intentions or is now a historical record."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("CarePlan.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.contained", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.extension", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.identifier", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.instantiatesCanonical", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.instantiatesUri", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.replaces", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.partOf", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.intent", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.category", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.subject", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.encounter", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.created", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.author", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.contributor", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.careTeam", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.addresses", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.supportingInfo", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.goal", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.extension", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.outcomeCodeableConcept",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.outcomeReference", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity.progress", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity.reference", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.kind", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.instantiatesCanonical",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.instantiatesUri",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.reasonCode", 0, None),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.reasonReference",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.goal", 0, None),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.statusReason",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.doNotPerform",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.scheduled[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.performer", 0, None),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.product[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.dailyAmount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CarePlan.activity.detail.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CarePlan.activity.detail.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CarePlan.note", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CarePlan {
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

impl crate::traits::resource::ResourceMutators for CarePlan {
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

impl crate::traits::resource::ResourceExistence for CarePlan {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CarePlan {
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

impl crate::traits::domain_resource::DomainResourceMutators for CarePlan {
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

impl crate::traits::domain_resource::DomainResourceExistence for CarePlan {
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

impl crate::traits::care_plan::CarePlanAccessors for CarePlan {
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
    fn part_of(&self) -> &[Reference] {
        self.part_of.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> RequestStatus {
        self.status.clone()
    }
    fn intent(&self) -> CarePlanIntent {
        self.intent.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn subject(&self) -> Reference {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn created(&self) -> Option<DateTimeType> {
        self.created.clone()
    }
    fn author(&self) -> Option<Reference> {
        self.author.clone()
    }
    fn contributor(&self) -> &[Reference] {
        self.contributor.as_deref().unwrap_or(&[])
    }
    fn care_team(&self) -> &[Reference] {
        self.care_team.as_deref().unwrap_or(&[])
    }
    fn addresses(&self) -> &[Reference] {
        self.addresses.as_deref().unwrap_or(&[])
    }
    fn supporting_info(&self) -> &[Reference] {
        self.supporting_info.as_deref().unwrap_or(&[])
    }
    fn goal(&self) -> &[Reference] {
        self.goal.as_deref().unwrap_or(&[])
    }
    fn activity(&self) -> &[CarePlanActivity] {
        self.activity.as_deref().unwrap_or(&[])
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::care_plan::CarePlanMutators for CarePlan {
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
    fn set_status(self, value: RequestStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_intent(self, value: CarePlanIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = value;
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
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = Some(value);
        resource
    }
    fn set_author(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn set_contributor(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.contributor = Some(value);
        resource
    }
    fn add_contributor(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.contributor.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_care_team(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.care_team = Some(value);
        resource
    }
    fn add_care_team(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.care_team.get_or_insert_with(Vec::new).push(item);
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
    fn set_supporting_info(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_info = Some(value);
        resource
    }
    fn add_supporting_info(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_info
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_activity(self, value: Vec<CarePlanActivity>) -> Self {
        let mut resource = self.clone();
        resource.activity = Some(value);
        resource
    }
    fn add_activity(self, item: CarePlanActivity) -> Self {
        let mut resource = self.clone();
        resource.activity.get_or_insert_with(Vec::new).push(item);
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
}

impl crate::traits::care_plan::CarePlanExistence for CarePlan {
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
    fn has_part_of(&self) -> bool {
        self.part_of.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_intent(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_subject(&self) -> bool {
        true
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_created(&self) -> bool {
        self.created.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.is_some()
    }
    fn has_contributor(&self) -> bool {
        self.contributor.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_care_team(&self) -> bool {
        self.care_team.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_addresses(&self) -> bool {
        self.addresses.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_info(&self) -> bool {
        self.supporting_info.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_goal(&self) -> bool {
        self.goal.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_activity(&self) -> bool {
        self.activity.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CarePlan {
    fn resource_type(&self) -> &'static str {
        "CarePlan"
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
        Some("http://hl7.org/fhir/StructureDefinition/CarePlan")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::care_plan::{CarePlanAccessors, CarePlanExistence, CarePlanMutators};
