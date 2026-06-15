use crate::bindings::conformance_expectation::ConformanceExpectation;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Requirements
///
/// The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Requirements
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Requirements
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this Requirements, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the Requirements (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the Requirements
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this Requirements (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this Requirements (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
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
    /// Natural language description of the requirements
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for Requirements (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this Requirements is defined
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
    /// Other set of Requirements this builds on
    #[serde(rename = "derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<StringType>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _derived_from: Vec<Element>,
    /// External artifact (rule/document etc. that) created this set of requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _reference: Vec<Element>,
    /// Actor for these requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<StringType>,
    /// Extension element for the 'actor' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _actor: Vec<Element>,
    /// Actual statement as markdown
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statement: Vec<RequirementsStatement>,
}
/// Requirements nested structure for the 'statement' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsStatement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Key that identifies this statement
    pub key: StringType,
    /// Extension element for the 'key' primitive field. Contains metadata and extensions.
    pub _key: Option<Element>,
    /// Short Human label for this statement
    pub label: Option<StringType>,
    /// Extension element for the 'label' primitive field. Contains metadata and extensions.
    pub _label: Option<Element>,
    /// SHALL | SHOULD | MAY | SHOULD-NOT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conformance: Vec<ConformanceExpectation>,
    /// Extension element for the 'conformance' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _conformance: Vec<Element>,
    /// Set to true if requirements statement is conditional
    pub conditionality: Option<BooleanType>,
    /// Extension element for the 'conditionality' primitive field. Contains metadata and extensions.
    pub _conditionality: Option<Element>,
    /// The actual requirement
    pub requirement: StringType,
    /// Extension element for the 'requirement' primitive field. Contains metadata and extensions.
    pub _requirement: Option<Element>,
    /// Another statement this clarifies/restricts ([url#]key)
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<StringType>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    pub _derived_from: Option<Element>,
    /// A larger requirement that this requirement helps to refine and enable
    pub parent: Option<StringType>,
    /// Extension element for the 'parent' primitive field. Contains metadata and extensions.
    pub _parent: Option<Element>,
    /// Design artifact that satisfies this requirement
    #[serde(rename = "satisfiedBy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub satisfied_by: Vec<StringType>,
    /// Extension element for the 'satisfiedBy' primitive field. Contains metadata and extensions.
    #[serde(rename = "_satisfiedBy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _satisfied_by: Vec<Element>,
    /// External artifact (rule/document etc. that) created this requirement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<StringType>,
    /// Extension element for the 'reference' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _reference: Vec<Element>,
    /// Who asked for this statement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<Reference>,
}

impl Default for Requirements {
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
            derived_from: Default::default(),
            _derived_from: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
            actor: Default::default(),
            _actor: Default::default(),
            statement: Default::default(),
        }
    }
}

impl Default for RequirementsStatement {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            key: StringType::default(),
            _key: Default::default(),
            label: Default::default(),
            _label: Default::default(),
            conformance: Default::default(),
            _conformance: Default::default(),
            conditionality: Default::default(),
            _conditionality: Default::default(),
            requirement: StringType::default(),
            _requirement: Default::default(),
            derived_from: Default::default(),
            _derived_from: Default::default(),
            parent: Default::default(),
            _parent: Default::default(),
            satisfied_by: Default::default(),
            _satisfied_by: Default::default(),
            reference: Default::default(),
            _reference: Default::default(),
            source: Default::default(),
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
                "Requirements.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Requirements.statement.conformance",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/conformance-expectation|5.0.0",
            ),
            rh_foundation::ElementBinding::new(
                "Requirements.status",
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
            rh_foundation::ElementCardinality::new("Requirements.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.contained", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.extension", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.versionAlgorithm[x]", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.contact", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.useContext", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.derivedFrom", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.reference", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.actor", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.statement", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.statement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.statement.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Requirements.statement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Requirements.statement.key", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.statement.label", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.statement.conformance", 0, None),
            rh_foundation::ElementCardinality::new(
                "Requirements.statement.conditionality",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Requirements.statement.requirement",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Requirements.statement.derivedFrom",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Requirements.statement.parent", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Requirements.statement.satisfiedBy", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.statement.reference", 0, None),
            rh_foundation::ElementCardinality::new("Requirements.statement.source", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Requirements {
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

impl crate::traits::resource::ResourceMutators for Requirements {
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

impl crate::traits::resource::ResourceExistence for Requirements {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Requirements {
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

impl crate::traits::domain_resource::DomainResourceMutators for Requirements {
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

impl crate::traits::domain_resource::DomainResourceExistence for Requirements {
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

impl crate::traits::requirements::RequirementsAccessors for Requirements {
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
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn derived_from(&self) -> &[StringType] {
        self.derived_from.as_slice()
    }
    fn reference(&self) -> &[StringType] {
        self.reference.as_slice()
    }
    fn actor(&self) -> &[StringType] {
        self.actor.as_slice()
    }
    fn statement(&self) -> &[RequirementsStatement] {
        self.statement.as_slice()
    }
}

impl crate::traits::requirements::RequirementsMutators for Requirements {
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
    fn set_derived_from(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.derived_from = value;
        resource
    }
    fn add_derived_from(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.derived_from.push(item);
        resource
    }
    fn set_reference(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.reference = value;
        resource
    }
    fn add_reference(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.reference.push(item);
        resource
    }
    fn set_actor(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.actor = value;
        resource
    }
    fn add_actor(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.actor.push(item);
        resource
    }
    fn set_statement(self, value: Vec<RequirementsStatement>) -> Self {
        let mut resource = self.clone();
        resource.statement = value;
        resource
    }
    fn add_statement(self, item: RequirementsStatement) -> Self {
        let mut resource = self.clone();
        resource.statement.push(item);
        resource
    }
}

impl crate::traits::requirements::RequirementsExistence for Requirements {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
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
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_derived_from(&self) -> bool {
        !self.derived_from.is_empty()
    }
    fn has_reference(&self) -> bool {
        !self.reference.is_empty()
    }
    fn has_actor(&self) -> bool {
        !self.actor.is_empty()
    }
    fn has_statement(&self) -> bool {
        !self.statement.is_empty()
    }
}

impl crate::validation::ValidatableResource for Requirements {
    fn resource_type(&self) -> &'static str {
        "Requirements"
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
        Some("http://hl7.org/fhir/StructureDefinition/Requirements")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::requirements::{
    RequirementsAccessors, RequirementsExistence, RequirementsMutators,
};
