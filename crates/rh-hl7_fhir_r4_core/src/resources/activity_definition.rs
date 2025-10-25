use crate::bindings::action_participant_type::ActionParticipantType;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::request_intent::RequestIntent;
use crate::bindings::request_priority::RequestPriority;
use crate::bindings::request_resource_types::RequestResourceTypes;
use crate::datatypes::age::Age;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::dosage::Dosage;
use crate::datatypes::duration::Duration;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::quantity::Quantity;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::timing::Timing;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ActivityDefinition
///
/// This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ActivityDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this activity definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the activity definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the activity definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this activity definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this activity definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate title of the activity definition
    pub subtitle: Option<StringType>,
    /// Extension element for the 'subtitle' primitive field. Contains metadata and extensions.
    pub _subtitle: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Type of individual the activity definition is intended for (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// Type of individual the activity definition is intended for (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
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
    /// Natural language description of the activity definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for activity definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this activity definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Describes the clinical usage of the activity definition
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// When the activity definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the activity definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the activity definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc.
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
    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Logic used by the activity definition
    pub library: Option<Vec<StringType>>,
    /// Extension element for the 'library' primitive field. Contains metadata and extensions.
    pub _library: Option<Element>,
    /// Kind of resource
    pub kind: Option<RequestResourceTypes>,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// What profile the resource needs to conform to
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Detail type of activity
    ///
    /// Binding: example (Detailed type of the activity; e.g. CBC.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/procedure-code
    pub code: Option<CodeableConcept>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: Option<RequestIntent>,
    /// Extension element for the 'intent' primitive field. Contains metadata and extensions.
    pub _intent: Option<Element>,
    /// routine | urgent | asap | stat
    pub priority: Option<RequestPriority>,
    /// Extension element for the 'priority' primitive field. Contains metadata and extensions.
    pub _priority: Option<Element>,
    /// True if the activity should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<BooleanType>,
    /// Extension element for the 'doNotPerform' primitive field. Contains metadata and extensions.
    #[serde(rename = "_doNotPerform")]
    pub _do_not_perform: Option<Element>,
    /// When activity is to occur (Timing)
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,
    /// When activity is to occur (dateTime)
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<DateTimeType>,
    /// When activity is to occur (Age)
    #[serde(rename = "timingAge")]
    pub timing_age: Option<Age>,
    /// When activity is to occur (Period)
    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,
    /// When activity is to occur (Range)
    #[serde(rename = "timingRange")]
    pub timing_range: Option<Range>,
    /// When activity is to occur (Duration)
    #[serde(rename = "timingDuration")]
    pub timing_duration: Option<Duration>,
    /// Where it should happen
    pub location: Option<Reference>,
    /// Who should participate in the action
    pub participant: Option<Vec<ActivityDefinitionParticipant>>,
    /// What's administered/supplied (Reference)
    #[serde(rename = "productReference")]
    pub product_reference: Option<Reference>,
    /// What's administered/supplied (CodeableConcept)
    #[serde(rename = "productCodeableConcept")]
    pub product_codeable_concept: Option<CodeableConcept>,
    /// How much is administered/consumed/supplied
    pub quantity: Option<Quantity>,
    /// Detailed dosage instructions
    pub dosage: Option<Vec<Dosage>>,
    /// What part of body to perform on
    ///
    /// Binding: example (A code that identifies the anatomical location.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/body-site
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// What specimens are required to perform this action
    #[serde(rename = "specimenRequirement")]
    pub specimen_requirement: Option<Vec<Reference>>,
    /// What observations are required to perform this action
    #[serde(rename = "observationRequirement")]
    pub observation_requirement: Option<Vec<Reference>>,
    /// What observations must be produced by this action
    #[serde(rename = "observationResultRequirement")]
    pub observation_result_requirement: Option<Vec<Reference>>,
    /// Transform to apply the template
    pub transform: Option<StringType>,
    /// Extension element for the 'transform' primitive field. Contains metadata and extensions.
    pub _transform: Option<Element>,
    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<ActivityDefinitionDynamicvalue>>,
}
/// ActivityDefinition nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDefinitionParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// patient | practitioner | related-person | device
    #[serde(rename = "type")]
    pub type_: ActionParticipantType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// E.g. Nurse, Surgeon, Parent, etc.
    ///
    /// Binding: example (Defines roles played by participants for the action.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/action-participant-role
    pub role: Option<CodeableConcept>,
}
/// ActivityDefinition nested structure for the 'dynamicValue' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDefinitionDynamicvalue {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The path to the element to be set dynamically
    pub path: StringType,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// An expression that provides the dynamic value for the customization
    pub expression: Expression,
}

impl Default for ActivityDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            subtitle: Default::default(),
            _subtitle: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            subject_codeable_concept: Default::default(),
            subject_reference: Default::default(),
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
            kind: Default::default(),
            _kind: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            code: Default::default(),
            intent: Default::default(),
            _intent: Default::default(),
            priority: Default::default(),
            _priority: Default::default(),
            do_not_perform: Default::default(),
            _do_not_perform: Default::default(),
            timing_timing: Default::default(),
            timing_date_time: Default::default(),
            timing_age: Default::default(),
            timing_period: Default::default(),
            timing_range: Default::default(),
            timing_duration: Default::default(),
            location: Default::default(),
            participant: Default::default(),
            product_reference: Default::default(),
            product_codeable_concept: Default::default(),
            quantity: Default::default(),
            dosage: Default::default(),
            body_site: Default::default(),
            specimen_requirement: Default::default(),
            observation_requirement: Default::default(),
            observation_result_requirement: Default::default(),
            transform: Default::default(),
            _transform: Default::default(),
            dynamic_value: Default::default(),
        }
    }
}

impl Default for ActivityDefinitionParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            role: Default::default(),
        }
    }
}

impl Default for ActivityDefinitionDynamicvalue {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            path: Default::default(),
            _path: Default::default(),
            expression: Default::default(),
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
    rh_foundation::Invariant::new("adf-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
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
    rh_foundation::ElementBinding::new("ActivityDefinition.intent", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-intent|4.0.1").with_description("Codes indicating the degree of authority/intentionality associated with a request."),
    rh_foundation::ElementBinding::new("ActivityDefinition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-resource-types|4.0.1").with_description("The kind of activity the definition is describing."),
    rh_foundation::ElementBinding::new("ActivityDefinition.participant.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/action-participant-type|4.0.1").with_description("The type of participant in the activity."),
    rh_foundation::ElementBinding::new("ActivityDefinition.priority", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/request-priority|4.0.1").with_description("Identifies the level of importance to be assigned to actioning the request."),
    rh_foundation::ElementBinding::new("ActivityDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ActivityDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.effectivePeriod",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.kind", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.profile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.intent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.doNotPerform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.timing[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.location", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.participant", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.participant.role",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.product[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.quantity", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.dosage", 0, None),
            rh_foundation::ElementCardinality::new("ActivityDefinition.bodySite", 0, None),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.specimenRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.observationRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.observationResultRequirement",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ActivityDefinition.transform", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ActivityDefinition.dynamicValue", 0, None),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.path",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ActivityDefinition.dynamicValue.expression",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ActivityDefinition {
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

impl crate::traits::resource::ResourceMutators for ActivityDefinition {
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

impl crate::traits::resource::ResourceExistence for ActivityDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ActivityDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ActivityDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ActivityDefinition {
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

impl crate::traits::activity_definition::ActivityDefinitionAccessors for ActivityDefinition {
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
    fn kind(&self) -> Option<RequestResourceTypes> {
        self.kind.clone()
    }
    fn profile(&self) -> Option<StringType> {
        self.profile.clone()
    }
    fn code(&self) -> Option<CodeableConcept> {
        self.code.clone()
    }
    fn intent(&self) -> Option<RequestIntent> {
        self.intent.clone()
    }
    fn priority(&self) -> Option<RequestPriority> {
        self.priority.clone()
    }
    fn do_not_perform(&self) -> Option<BooleanType> {
        self.do_not_perform
    }
    fn location(&self) -> Option<Reference> {
        self.location.clone()
    }
    fn participant(&self) -> &[ActivityDefinitionParticipant] {
        self.participant.as_deref().unwrap_or(&[])
    }
    fn quantity(&self) -> Option<Quantity> {
        self.quantity.clone()
    }
    fn dosage(&self) -> &[Dosage] {
        self.dosage.as_deref().unwrap_or(&[])
    }
    fn body_site(&self) -> &[CodeableConcept] {
        self.body_site.as_deref().unwrap_or(&[])
    }
    fn specimen_requirement(&self) -> &[Reference] {
        self.specimen_requirement.as_deref().unwrap_or(&[])
    }
    fn observation_requirement(&self) -> &[Reference] {
        self.observation_requirement.as_deref().unwrap_or(&[])
    }
    fn observation_result_requirement(&self) -> &[Reference] {
        self.observation_result_requirement
            .as_deref()
            .unwrap_or(&[])
    }
    fn transform(&self) -> Option<StringType> {
        self.transform.clone()
    }
    fn dynamic_value(&self) -> &[ActivityDefinitionDynamicvalue] {
        self.dynamic_value.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::activity_definition::ActivityDefinitionMutators for ActivityDefinition {
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
    fn set_kind(self, value: RequestResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.kind = Some(value);
        resource
    }
    fn set_profile(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.profile = Some(value);
        resource
    }
    fn set_code(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.code = Some(value);
        resource
    }
    fn set_intent(self, value: RequestIntent) -> Self {
        let mut resource = self.clone();
        resource.intent = Some(value);
        resource
    }
    fn set_priority(self, value: RequestPriority) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_do_not_perform(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.do_not_perform = Some(value);
        resource
    }
    fn set_location(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.location = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<ActivityDefinitionParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = Some(value);
        resource
    }
    fn add_participant(self, item: ActivityDefinitionParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_quantity(self, value: Quantity) -> Self {
        let mut resource = self.clone();
        resource.quantity = Some(value);
        resource
    }
    fn set_dosage(self, value: Vec<Dosage>) -> Self {
        let mut resource = self.clone();
        resource.dosage = Some(value);
        resource
    }
    fn add_dosage(self, item: Dosage) -> Self {
        let mut resource = self.clone();
        resource.dosage.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.body_site = Some(value);
        resource
    }
    fn add_body_site(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.body_site.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_specimen_requirement(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.specimen_requirement = Some(value);
        resource
    }
    fn add_specimen_requirement(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .specimen_requirement
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_observation_requirement(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.observation_requirement = Some(value);
        resource
    }
    fn add_observation_requirement(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .observation_requirement
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_observation_result_requirement(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.observation_result_requirement = Some(value);
        resource
    }
    fn add_observation_result_requirement(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .observation_result_requirement
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_transform(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.transform = Some(value);
        resource
    }
    fn set_dynamic_value(self, value: Vec<ActivityDefinitionDynamicvalue>) -> Self {
        let mut resource = self.clone();
        resource.dynamic_value = Some(value);
        resource
    }
    fn add_dynamic_value(self, item: ActivityDefinitionDynamicvalue) -> Self {
        let mut resource = self.clone();
        resource
            .dynamic_value
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::activity_definition::ActivityDefinitionExistence for ActivityDefinition {
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
    fn has_subject(&self) -> bool {
        self.subject_codeable_concept.is_some() || self.subject_reference.is_some()
    }
    fn has_timing(&self) -> bool {
        self.timing_timing.is_some()
            || self.timing_date_time.is_some()
            || self.timing_age.is_some()
            || self.timing_period.is_some()
            || self.timing_range.is_some()
            || self.timing_duration.is_some()
    }
    fn has_product(&self) -> bool {
        self.product_reference.is_some() || self.product_codeable_concept.is_some()
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
    fn has_kind(&self) -> bool {
        self.kind.is_some()
    }
    fn has_profile(&self) -> bool {
        self.profile.is_some()
    }
    fn has_code(&self) -> bool {
        self.code.is_some()
    }
    fn has_intent(&self) -> bool {
        self.intent.is_some()
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_do_not_perform(&self) -> bool {
        self.do_not_perform.is_some()
    }
    fn has_location(&self) -> bool {
        self.location.is_some()
    }
    fn has_participant(&self) -> bool {
        self.participant.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }
    fn has_dosage(&self) -> bool {
        self.dosage.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_body_site(&self) -> bool {
        self.body_site.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specimen_requirement(&self) -> bool {
        self.specimen_requirement
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_observation_requirement(&self) -> bool {
        self.observation_requirement
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_observation_result_requirement(&self) -> bool {
        self.observation_result_requirement
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_transform(&self) -> bool {
        self.transform.is_some()
    }
    fn has_dynamic_value(&self) -> bool {
        self.dynamic_value.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for ActivityDefinition {
    fn resource_type(&self) -> &'static str {
        "ActivityDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ActivityDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::activity_definition::{
    ActivityDefinitionAccessors, ActivityDefinitionExistence, ActivityDefinitionMutators,
};
