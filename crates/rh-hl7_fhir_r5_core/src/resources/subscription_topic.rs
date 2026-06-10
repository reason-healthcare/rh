use crate::bindings::interaction_trigger::InteractionTrigger;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::search_comparator::SearchComparator;
use crate::bindings::search_modifier_code::SearchModifierCode;
use crate::bindings::subscriptiontopic_cr_behavior::SubscriptiontopicCrBehavior;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SubscriptionTopic
///
/// Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SubscriptionTopic
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopic {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this subscription topic, represented as an absolute URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business identifier for subscription topic
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the subscription topic
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this subscription topic (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this subscription topic (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Based on FHIR protocol or definition
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<StringType>>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    pub _derived_from: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// If for testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date status first applied
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// The name of the individual or organization that published the SubscriptionTopic
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the SubscriptionTopic
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Content intends to support these contexts
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction of the SubscriptionTopic (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this SubscriptionTopic is defined
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
    /// When SubscriptionTopic is/was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// Date the Subscription Topic was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// The effective date range for the SubscriptionTopic
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// Definition of a resource-based trigger for the subscription topic
    #[serde(rename = "resourceTrigger")]
    pub resource_trigger: Option<Vec<SubscriptionTopicResourcetrigger>>,
    /// Event definitions the SubscriptionTopic
    #[serde(rename = "eventTrigger")]
    pub event_trigger: Option<Vec<SubscriptionTopicEventtrigger>>,
    /// Properties by which a Subscription can filter notifications from the SubscriptionTopic
    #[serde(rename = "canFilterBy")]
    pub can_filter_by: Option<Vec<SubscriptionTopicCanfilterby>>,
    /// Properties for describing the shape of notifications generated by this topic
    #[serde(rename = "notificationShape")]
    pub notification_shape: Option<Vec<SubscriptionTopicNotificationshape>>,
}
/// SubscriptionTopic nested structure for the 'canFilterBy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopicCanfilterby {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Description of this filter parameter
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// URL of the triggering Resource that this filter applies to
    ///
    /// Binding: extensible (A type of resource, or a Reference (from all versions))
    ///
    /// Available values:
    /// - `Reference`
    pub resource: Option<StringType>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// Human-readable and computation-friendly name for a filter parameter usable by subscriptions on this topic, via Subscription.filterBy.filterParameter
    #[serde(rename = "filterParameter")]
    pub filter_parameter: StringType,
    /// Extension element for the 'filterParameter' primitive field. Contains metadata and extensions.
    #[serde(rename = "_filterParameter")]
    pub _filter_parameter: Option<Element>,
    /// Canonical URL for a filterParameter definition
    #[serde(rename = "filterDefinition")]
    pub filter_definition: Option<StringType>,
    /// Extension element for the 'filterDefinition' primitive field. Contains metadata and extensions.
    #[serde(rename = "_filterDefinition")]
    pub _filter_definition: Option<Element>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<Vec<SearchComparator>>,
    /// Extension element for the 'comparator' primitive field. Contains metadata and extensions.
    pub _comparator: Option<Element>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<Vec<SearchModifierCode>>,
    /// Extension element for the 'modifier' primitive field. Contains metadata and extensions.
    pub _modifier: Option<Element>,
}
/// SubscriptionTopicResourcetrigger nested structure for the 'queryCriteria' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopicResourcetriggerQuerycriteria {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Rule applied to previous resource state
    pub previous: Option<StringType>,
    /// Extension element for the 'previous' primitive field. Contains metadata and extensions.
    pub _previous: Option<Element>,
    /// test-passes | test-fails
    #[serde(rename = "resultForCreate")]
    pub result_for_create: Option<SubscriptiontopicCrBehavior>,
    /// Extension element for the 'resultForCreate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_resultForCreate")]
    pub _result_for_create: Option<Element>,
    /// Rule applied to current resource state
    pub current: Option<StringType>,
    /// Extension element for the 'current' primitive field. Contains metadata and extensions.
    pub _current: Option<Element>,
    /// test-passes | test-fails
    #[serde(rename = "resultForDelete")]
    pub result_for_delete: Option<SubscriptiontopicCrBehavior>,
    /// Extension element for the 'resultForDelete' primitive field. Contains metadata and extensions.
    #[serde(rename = "_resultForDelete")]
    pub _result_for_delete: Option<Element>,
    /// Both must be true flag
    #[serde(rename = "requireBoth")]
    pub require_both: Option<BooleanType>,
    /// Extension element for the 'requireBoth' primitive field. Contains metadata and extensions.
    #[serde(rename = "_requireBoth")]
    pub _require_both: Option<Element>,
}
/// SubscriptionTopic nested structure for the 'notificationShape' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopicNotificationshape {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// URL of the Resource that is the focus (main) resource in a notification shape
    ///
    /// Binding: extensible (A type of resource, or a Reference (from all versions))
    ///
    /// Available values:
    /// - `Reference`
    pub resource: StringType,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// Include directives, rooted in the resource for this shape
    pub include: Option<Vec<StringType>>,
    /// Extension element for the 'include' primitive field. Contains metadata and extensions.
    pub _include: Option<Element>,
    /// Reverse include directives, rooted in the resource for this shape
    #[serde(rename = "revInclude")]
    pub rev_include: Option<Vec<StringType>>,
    /// Extension element for the 'revInclude' primitive field. Contains metadata and extensions.
    #[serde(rename = "_revInclude")]
    pub _rev_include: Option<Element>,
}
/// SubscriptionTopic nested structure for the 'resourceTrigger' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopicResourcetrigger {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Query based trigger rule
    #[serde(rename = "queryCriteria")]
    pub query_criteria: Option<SubscriptionTopicResourcetriggerQuerycriteria>,
    /// Text representation of the resource trigger
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Data Type or Resource (reference to definition) for this trigger definition
    ///
    /// Binding: extensible (A type of resource, or a Reference (from all versions))
    ///
    /// Available values:
    /// - `Reference`
    pub resource: StringType,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// create | update | delete
    #[serde(rename = "supportedInteraction")]
    pub supported_interaction: Option<Vec<InteractionTrigger>>,
    /// Extension element for the 'supportedInteraction' primitive field. Contains metadata and extensions.
    #[serde(rename = "_supportedInteraction")]
    pub _supported_interaction: Option<Element>,
    /// FHIRPath based trigger rule
    #[serde(rename = "fhirPathCriteria")]
    pub fhir_path_criteria: Option<StringType>,
    /// Extension element for the 'fhirPathCriteria' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fhirPathCriteria")]
    pub _fhir_path_criteria: Option<Element>,
}
/// SubscriptionTopic nested structure for the 'eventTrigger' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTopicEventtrigger {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Text representation of the event trigger
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Event which can trigger a notification from the SubscriptionTopic
    ///
    /// Binding: example (FHIR Value set/code system definition for HL7 V2 table 0003 (EVENT TYPE CODE).)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0003
    pub event: CodeableConcept,
    /// Data Type or Resource (reference to definition) for this trigger definition
    ///
    /// Binding: extensible (A type of resource, or a Reference (from all versions))
    ///
    /// Available values:
    /// - `Reference`
    pub resource: StringType,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
}

impl Default for SubscriptionTopic {
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
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            derived_from: Default::default(),
            _derived_from: Default::default(),
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
            resource_trigger: Default::default(),
            event_trigger: Default::default(),
            can_filter_by: Default::default(),
            notification_shape: Default::default(),
        }
    }
}

impl Default for SubscriptionTopicCanfilterby {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            filter_parameter: Default::default(),
            _filter_parameter: Default::default(),
            filter_definition: Default::default(),
            _filter_definition: Default::default(),
            comparator: Default::default(),
            _comparator: Default::default(),
            modifier: Default::default(),
            _modifier: Default::default(),
        }
    }
}

impl Default for SubscriptionTopicResourcetriggerQuerycriteria {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            previous: Default::default(),
            _previous: Default::default(),
            result_for_create: Default::default(),
            _result_for_create: Default::default(),
            current: Default::default(),
            _current: Default::default(),
            result_for_delete: Default::default(),
            _result_for_delete: Default::default(),
            require_both: Default::default(),
            _require_both: Default::default(),
        }
    }
}

impl Default for SubscriptionTopicNotificationshape {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            resource: Default::default(),
            _resource: Default::default(),
            include: Default::default(),
            _include: Default::default(),
            rev_include: Default::default(),
            _rev_include: Default::default(),
        }
    }
}

impl Default for SubscriptionTopicResourcetrigger {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            query_criteria: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            supported_interaction: Default::default(),
            _supported_interaction: Default::default(),
            fhir_path_criteria: Default::default(),
            _fhir_path_criteria: Default::default(),
        }
    }
}

impl Default for SubscriptionTopicEventtrigger {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            event: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
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
            rh_foundation::Invariant::new(
                "ele-1",
                rh_foundation::Severity::Error,
                "All FHIR elements must have a @value or children",
                "hasValue() or (children().count() > id.count())",
            ),
            rh_foundation::Invariant::new(
                "ext-1",
                rh_foundation::Severity::Error,
                "Must have either extensions or value[x], not both",
                "extension.exists() != value.exists()",
            ),
        ]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("SubscriptionTopic.canFilterBy.comparator", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-comparator|5.0.0").with_description("Search Comparator Codes supported in this filter."),
    rh_foundation::ElementBinding::new("SubscriptionTopic.canFilterBy.modifier", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-modifier-code|5.0.0").with_description("Search Modifier Codes supported in this filter."),
    rh_foundation::ElementBinding::new("SubscriptionTopic.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("SubscriptionTopic.resourceTrigger.queryCriteria.resultForCreate", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/subscriptiontopic-cr-behavior|5.0.0").with_description("Behavior a server can exhibit when a criteria state does not exist (e.g., state prior to a create or after a delete)."),
    rh_foundation::ElementBinding::new("SubscriptionTopic.resourceTrigger.queryCriteria.resultForDelete", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/subscriptiontopic-cr-behavior|5.0.0").with_description("Behavior a server can exhibit when a criteria state does not exist (e.g., state prior to a create or after a delete)."),
    rh_foundation::ElementBinding::new("SubscriptionTopic.resourceTrigger.supportedInteraction", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/interaction-trigger|5.0.0").with_description("FHIR RESTful interaction used to filter a resource-based SubscriptionTopic trigger."),
    rh_foundation::ElementBinding::new("SubscriptionTopic.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SubscriptionTopic.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.contained", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.extension", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.identifier", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.contact", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.useContext", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.resourceTrigger", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.resource",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.supportedInteraction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.previous",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.resultForCreate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.current",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.resultForDelete",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.queryCriteria.requireBoth",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.resourceTrigger.fhirPathCriteria",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.eventTrigger", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.eventTrigger.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.eventTrigger.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.eventTrigger.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.eventTrigger.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.eventTrigger.event",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.eventTrigger.resource",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.canFilterBy", 0, None),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.canFilterBy.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.resource",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.filterParameter",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.filterDefinition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.comparator",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.canFilterBy.modifier",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("SubscriptionTopic.notificationShape", 0, None),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.resource",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.include",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SubscriptionTopic.notificationShape.revInclude",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SubscriptionTopic {
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

impl crate::traits::resource::ResourceMutators for SubscriptionTopic {
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

impl crate::traits::resource::ResourceExistence for SubscriptionTopic {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SubscriptionTopic {
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

impl crate::traits::domain_resource::DomainResourceMutators for SubscriptionTopic {
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

impl crate::traits::domain_resource::DomainResourceExistence for SubscriptionTopic {
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

impl crate::traits::subscription_topic::SubscriptionTopicAccessors for SubscriptionTopic {
    fn url(&self) -> StringType {
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
    fn derived_from(&self) -> &[StringType] {
        self.derived_from.as_deref().unwrap_or(&[])
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
    fn resource_trigger(&self) -> &[SubscriptionTopicResourcetrigger] {
        self.resource_trigger.as_deref().unwrap_or(&[])
    }
    fn event_trigger(&self) -> &[SubscriptionTopicEventtrigger] {
        self.event_trigger.as_deref().unwrap_or(&[])
    }
    fn can_filter_by(&self) -> &[SubscriptionTopicCanfilterby] {
        self.can_filter_by.as_deref().unwrap_or(&[])
    }
    fn notification_shape(&self) -> &[SubscriptionTopicNotificationshape] {
        self.notification_shape.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::subscription_topic::SubscriptionTopicMutators for SubscriptionTopic {
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
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_derived_from(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
        resource
    }
    fn add_derived_from(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .derived_from
            .get_or_insert_with(Vec::new)
            .push(item);
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
    fn set_resource_trigger(self, value: Vec<SubscriptionTopicResourcetrigger>) -> Self {
        let mut resource = self.clone();
        resource.resource_trigger = Some(value);
        resource
    }
    fn add_resource_trigger(self, item: SubscriptionTopicResourcetrigger) -> Self {
        let mut resource = self.clone();
        resource
            .resource_trigger
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_event_trigger(self, value: Vec<SubscriptionTopicEventtrigger>) -> Self {
        let mut resource = self.clone();
        resource.event_trigger = Some(value);
        resource
    }
    fn add_event_trigger(self, item: SubscriptionTopicEventtrigger) -> Self {
        let mut resource = self.clone();
        resource
            .event_trigger
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_can_filter_by(self, value: Vec<SubscriptionTopicCanfilterby>) -> Self {
        let mut resource = self.clone();
        resource.can_filter_by = Some(value);
        resource
    }
    fn add_can_filter_by(self, item: SubscriptionTopicCanfilterby) -> Self {
        let mut resource = self.clone();
        resource
            .can_filter_by
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_notification_shape(self, value: Vec<SubscriptionTopicNotificationshape>) -> Self {
        let mut resource = self.clone();
        resource.notification_shape = Some(value);
        resource
    }
    fn add_notification_shape(self, item: SubscriptionTopicNotificationshape) -> Self {
        let mut resource = self.clone();
        resource
            .notification_shape
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::subscription_topic::SubscriptionTopicExistence for SubscriptionTopic {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
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
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.as_ref().is_some_and(|v| !v.is_empty())
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
    fn has_resource_trigger(&self) -> bool {
        self.resource_trigger
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_event_trigger(&self) -> bool {
        self.event_trigger.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_can_filter_by(&self) -> bool {
        self.can_filter_by.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_notification_shape(&self) -> bool {
        self.notification_shape
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SubscriptionTopic {
    fn resource_type(&self) -> &'static str {
        "SubscriptionTopic"
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
        Some("http://hl7.org/fhir/StructureDefinition/SubscriptionTopic")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::subscription_topic::{
    SubscriptionTopicAccessors, SubscriptionTopicExistence, SubscriptionTopicMutators,
};
