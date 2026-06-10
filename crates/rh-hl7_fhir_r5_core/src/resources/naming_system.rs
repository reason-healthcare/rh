use crate::bindings::namingsystem_identifier_type::NamingsystemIdentifierType;
use crate::bindings::namingsystem_type::NamingsystemType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::related_artifact::RelatedArtifact;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// NamingSystem
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingSystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this naming system, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the naming system (business identifier)
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the naming system
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this naming system (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Title for this naming system (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// codesystem | identifier | root
    pub kind: NamingsystemType,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Who maintains system namespace?
    pub responsible: Option<StringType>,
    /// Extension element for the 'responsible' primitive field. Contains metadata and extensions.
    pub _responsible: Option<Element>,
    /// e.g. driver,  provider,  patient, bank etc
    ///
    /// Binding: preferred (A coded type for an identifier that can be used to determine which identifier to use for a specific purpose.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/namingsystem-identifier-system-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Natural language description of the naming system
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for naming system (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this naming system is defined
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
    /// When the NamingSystem was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<DateType>,
    /// Extension element for the 'approvalDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_approvalDate")]
    pub _approval_date: Option<Element>,
    /// When the NamingSystem was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<DateType>,
    /// Extension element for the 'lastReviewDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastReviewDate")]
    pub _last_review_date: Option<Element>,
    /// When the NamingSystem is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-topic
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the CodeSystem
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the NamingSystem
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the NamingSystem
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the NamingSystem
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// How/where is it used
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
    /// Unique identifiers used for system
    #[serde(rename = "uniqueId")]
    pub unique_id: Vec<NamingSystemUniqueid>,
}
/// NamingSystem nested structure for the 'uniqueId' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingSystemUniqueid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// oid | uuid | uri | iri-stem | v2csmnemonic | other
    #[serde(rename = "type")]
    pub type_: NamingsystemIdentifierType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The unique identifier
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Is this the id that should be used for this type
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
    /// Notes about identifier usage
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// When is identifier valid?
    pub period: Option<Period>,
    /// Whether the identifier is authoritative
    pub authoritative: Option<BooleanType>,
    /// Extension element for the 'authoritative' primitive field. Contains metadata and extensions.
    pub _authoritative: Option<Element>,
}

impl Default for NamingSystem {
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
            name: StringType::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            kind: NamingsystemType::default(),
            _kind: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            responsible: Default::default(),
            _responsible: Default::default(),
            type_: Default::default(),
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
            topic: Default::default(),
            author: Default::default(),
            editor: Default::default(),
            reviewer: Default::default(),
            endorser: Default::default(),
            related_artifact: Default::default(),
            usage: Default::default(),
            _usage: Default::default(),
            unique_id: Vec::new(),
        }
    }
}

impl Default for NamingSystemUniqueid {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            period: Default::default(),
            authoritative: Default::default(),
            _authoritative: Default::default(),
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
    rh_foundation::Invariant::new("nsd-1", rh_foundation::Severity::Error, "Root systems cannot have uuid identifiers", "kind != 'root' or uniqueId.all(type != 'uuid')"),
    rh_foundation::Invariant::new("nsd-2", rh_foundation::Severity::Error, "Can't have more than one preferred identifier for a type", "uniqueId.where(preferred = true).select(type).isDistinct()"),
    rh_foundation::Invariant::new("nsd-3", rh_foundation::Severity::Error, "Can't have more than one authoritative identifier for a type/period combination (only one authoritative identifier allowed at any given point of time)", "uniqueId.where(authoritative = 'true').select(type.toString() & period.start.toString() & period.end.toString()).isDistinct()"),
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
                "NamingSystem.kind",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-type|5.0.0",
            )
            .with_description("Identifies the purpose of the naming system."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "NamingSystem.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.uniqueId.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-identifier-type|5.0.0",
            )
            .with_description(
                "Identifies the style of unique identifier used to identify a namespace.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NamingSystem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contained", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contact", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.responsible", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.useContext", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.approvalDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.lastReviewDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.effectivePeriod", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.topic", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.author", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.editor", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.reviewer", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.endorser", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.relatedArtifact", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId", 1, None),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NamingSystem.uniqueId.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.preferred", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "NamingSystem.uniqueId.authoritative",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for NamingSystem {
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

impl crate::traits::resource::ResourceMutators for NamingSystem {
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

impl crate::traits::resource::ResourceExistence for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceAccessors for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceMutators for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceExistence for NamingSystem {
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

impl crate::traits::naming_system::NamingSystemAccessors for NamingSystem {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> StringType {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn kind(&self) -> NamingsystemType {
        self.kind.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_deref().unwrap_or(&[])
    }
    fn responsible(&self) -> Option<StringType> {
        self.responsible.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
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
    fn usage(&self) -> Option<StringType> {
        self.usage.clone()
    }
    fn unique_id(&self) -> &[NamingSystemUniqueid] {
        &self.unique_id
    }
}

impl crate::traits::naming_system::NamingSystemMutators for NamingSystem {
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
        resource.name = value;
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_kind(self, value: NamingsystemType) -> Self {
        let mut resource = self.clone();
        resource.kind = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
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
    fn set_responsible(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.responsible = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
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
    fn set_usage(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage = Some(value);
        resource
    }
    fn set_unique_id(self, value: Vec<NamingSystemUniqueid>) -> Self {
        let mut resource = self.clone();
        resource.unique_id = value;
        resource
    }
    fn add_unique_id(self, item: NamingSystemUniqueid) -> Self {
        let mut resource = self.clone();
        resource.unique_id.push(item);
        resource
    }
}

impl crate::traits::naming_system::NamingSystemExistence for NamingSystem {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
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
        true
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_kind(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        self.contact.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_responsible(&self) -> bool {
        self.responsible.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
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
    fn has_usage(&self) -> bool {
        self.usage.is_some()
    }
    fn has_unique_id(&self) -> bool {
        !self.unique_id.is_empty()
    }
}

impl crate::validation::ValidatableResource for NamingSystem {
    fn resource_type(&self) -> &'static str {
        "NamingSystem"
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
        Some("http://hl7.org/fhir/StructureDefinition/NamingSystem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::naming_system::{
    NamingSystemAccessors, NamingSystemExistence, NamingSystemMutators,
};
