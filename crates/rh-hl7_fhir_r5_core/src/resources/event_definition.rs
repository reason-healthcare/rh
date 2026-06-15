use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::trigger_definition::TriggerDefinition;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// EventDefinition
///
/// The EventDefinition resource provides a reusable description of when a particular event can occur.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/EventDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: EventDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this event definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the event definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the event definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this event definition (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this event definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Subordinate title of the event definition
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
    /// Type of individual the event definition is focused on (CodeableConcept)
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,
    /// Type of individual the event definition is focused on (Reference)
    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Reference>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the event definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for event definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this event definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Describes the clinical usage of the event definition
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
    /// When the event definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the event definition was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the event definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    ///
    /// Binding: example (High-level categorization of the definition, used for searching, sorting, and filtering.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<CodeableConcept>,
    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<ContactDetail>,
    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<ContactDetail>,
    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<ContactDetail>,
    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<ContactDetail>,
    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<RelatedArtifact>,
    /// "when" the event occurs (multiple = 'or')
    pub trigger: Vec<TriggerDefinition>,
}

impl Default for EventDefinition {
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
            trigger: Vec::new(),
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
                "EventDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "EventDefinition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("EventDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "EventDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("EventDefinition.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.subtitle", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.subject[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("EventDefinition.topic", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.author", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.editor", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.endorser", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("EventDefinition.trigger", 1, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for EventDefinition {
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

impl crate::traits::resource::ResourceMutators for EventDefinition {
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

impl crate::traits::resource::ResourceExistence for EventDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for EventDefinition {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for EventDefinition {
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for EventDefinition {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::event_definition::EventDefinitionAccessors for EventDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
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
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
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
        self.topic.as_slice()
    }
    fn author(&self) -> &[ContactDetail] {
        self.author.as_slice()
    }
    fn editor(&self) -> &[ContactDetail] {
        self.editor.as_slice()
    }
    fn reviewer(&self) -> &[ContactDetail] {
        self.reviewer.as_slice()
    }
    fn endorser(&self) -> &[ContactDetail] {
        self.endorser.as_slice()
    }
    fn related_artifact(&self) -> &[RelatedArtifact] {
        self.related_artifact.as_slice()
    }
    fn trigger(&self) -> &[TriggerDefinition] {
        &self.trigger
    }
}

impl crate::traits::event_definition::EventDefinitionMutators for EventDefinition {
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
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
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
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
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
        resource.topic = value;
        resource
    }
    fn add_topic(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.topic.push(item);
        resource
    }
    fn set_author(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.author = value;
        resource
    }
    fn add_author(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.author.push(item);
        resource
    }
    fn set_editor(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.editor = value;
        resource
    }
    fn add_editor(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.editor.push(item);
        resource
    }
    fn set_reviewer(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.reviewer = value;
        resource
    }
    fn add_reviewer(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.reviewer.push(item);
        resource
    }
    fn set_endorser(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.endorser = value;
        resource
    }
    fn add_endorser(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.endorser.push(item);
        resource
    }
    fn set_related_artifact(self, value: Vec<RelatedArtifact>) -> Self {
        let mut resource = self.clone();
        resource.related_artifact = value;
        resource
    }
    fn add_related_artifact(self, item: RelatedArtifact) -> Self {
        let mut resource = self.clone();
        resource.related_artifact.push(item);
        resource
    }
    fn set_trigger(self, value: Vec<TriggerDefinition>) -> Self {
        let mut resource = self.clone();
        resource.trigger = value;
        resource
    }
    fn add_trigger(self, item: TriggerDefinition) -> Self {
        let mut resource = self.clone();
        resource.trigger.push(item);
        resource
    }
}

impl crate::traits::event_definition::EventDefinitionExistence for EventDefinition {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject_codeable_concept.is_some() || self.subject_reference.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
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
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
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
        !self.topic.is_empty()
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_editor(&self) -> bool {
        !self.editor.is_empty()
    }
    fn has_reviewer(&self) -> bool {
        !self.reviewer.is_empty()
    }
    fn has_endorser(&self) -> bool {
        !self.endorser.is_empty()
    }
    fn has_related_artifact(&self) -> bool {
        !self.related_artifact.is_empty()
    }
    fn has_trigger(&self) -> bool {
        !self.trigger.is_empty()
    }
}

impl crate::validation::ValidatableResource for EventDefinition {
    fn resource_type(&self) -> &'static str {
        "EventDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/EventDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::event_definition::{
    EventDefinitionAccessors, EventDefinitionExistence, EventDefinitionMutators,
};
