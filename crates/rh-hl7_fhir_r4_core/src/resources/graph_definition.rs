use crate::bindings::compartment_type::CompartmentType;
use crate::bindings::graph_compartment_rule::GraphCompartmentRule;
use crate::bindings::graph_compartment_use::GraphCompartmentUse;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// GraphDefinition
///
/// A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules about the set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: GraphDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this graph definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business version of the graph definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this graph definition (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
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
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the graph definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for graph definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this graph definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Type of resource at which the graph starts
    pub start: ResourceTypes,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// Profile on base resource
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Links this graph makes rules about
    pub link: Option<Vec<GraphDefinitionLink>>,
}
/// GraphDefinitionLink nested structure for the 'target' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDefinitionLinkTarget {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of resource this link refers to
    #[serde(rename = "type")]
    pub type_: ResourceTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Criteria for reverse lookup
    pub params: Option<StringType>,
    /// Extension element for the 'params' primitive field. Contains metadata and extensions.
    pub _params: Option<Element>,
    /// Profile for the target resource
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Additional links from target resource
    pub link: Option<Vec<StringType>>,
}
/// GraphDefinitionLinkTarget nested structure for the 'compartment' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDefinitionLinkTargetCompartment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// condition | requirement
    #[serde(rename = "use")]
    pub use_: GraphCompartmentUse,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Patient | Encounter | RelatedPerson | Practitioner | Device
    pub code: CompartmentType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// identical | matching | different | custom
    pub rule: GraphCompartmentRule,
    /// Extension element for the 'rule' primitive field. Contains metadata and extensions.
    pub _rule: Option<Element>,
    /// Custom rule, as a FHIRPath expression
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// Documentation for FHIRPath expression
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// GraphDefinition nested structure for the 'link' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDefinitionLink {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Potential target for the link
    pub target: Option<Vec<GraphDefinitionLinkTarget>>,
    /// Path in the resource that contains the link
    pub path: Option<StringType>,
    /// Extension element for the 'path' primitive field. Contains metadata and extensions.
    pub _path: Option<Element>,
    /// Which slice (if profiled)
    #[serde(rename = "sliceName")]
    pub slice_name: Option<StringType>,
    /// Extension element for the 'sliceName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sliceName")]
    pub _slice_name: Option<Element>,
    /// Minimum occurrences for this link
    pub min: Option<IntegerType>,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Maximum occurrences for this link
    pub max: Option<StringType>,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
    /// Why this link is specified
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}

impl Default for GraphDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
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
            start: ResourceTypes::default(),
            _start: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            link: Default::default(),
        }
    }
}

impl Default for GraphDefinitionLinkTarget {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            params: Default::default(),
            _params: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            link: Default::default(),
        }
    }
}

impl Default for GraphDefinitionLinkTargetCompartment {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            use_: Default::default(),
            _use: Default::default(),
            code: Default::default(),
            _code: Default::default(),
            rule: Default::default(),
            _rule: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for GraphDefinitionLink {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            target: Default::default(),
            path: Default::default(),
            _path: Default::default(),
            slice_name: Default::default(),
            _slice_name: Default::default(),
            min: Default::default(),
            _min: Default::default(),
            max: Default::default(),
            _max: Default::default(),
            description: Default::default(),
            _description: Default::default(),
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
    rh_foundation::Invariant::new("gdf-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
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
                "GraphDefinition.link.target.compartment.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/compartment-type|4.0.1",
            )
            .with_description("Identifies a compartment."),
            rh_foundation::ElementBinding::new(
                "GraphDefinition.link.target.compartment.rule",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/graph-compartment-rule|4.0.1",
            )
            .with_description("How a compartment must be linked."),
            rh_foundation::ElementBinding::new(
                "GraphDefinition.link.target.compartment.use",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/graph-compartment-use|4.0.1",
            )
            .with_description("Defines how a compartment rule is used."),
            rh_foundation::ElementBinding::new(
                "GraphDefinition.link.target.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/resource-types|4.0.1",
            )
            .with_description("One of the resource types defined as part of this version of FHIR."),
            rh_foundation::ElementBinding::new(
                "GraphDefinition.start",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/resource-types|4.0.1",
            )
            .with_description("One of the resource types defined as part of this version of FHIR."),
            rh_foundation::ElementBinding::new(
                "GraphDefinition.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|4.0.1",
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
            rh_foundation::ElementCardinality::new("GraphDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.start", 1, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.profile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.path", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.sliceName", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.min", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.max", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.target", 0, None),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.target.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.target.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.params",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.profile",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.use",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.rule",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.expression",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "GraphDefinition.link.target.compartment.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("GraphDefinition.link.target.link", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for GraphDefinition {
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

impl crate::traits::resource::ResourceMutators for GraphDefinition {
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

impl crate::traits::resource::ResourceExistence for GraphDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for GraphDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for GraphDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for GraphDefinition {
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

impl crate::traits::graph_definition::GraphDefinitionAccessors for GraphDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> StringType {
        self.name.clone()
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
    fn start(&self) -> ResourceTypes {
        self.start.clone()
    }
    fn profile(&self) -> Option<StringType> {
        self.profile.clone()
    }
    fn link(&self) -> &[GraphDefinitionLink] {
        self.link.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::graph_definition::GraphDefinitionMutators for GraphDefinition {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
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
    fn set_start(self, value: ResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.start = value;
        resource
    }
    fn set_profile(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.profile = Some(value);
        resource
    }
    fn set_link(self, value: Vec<GraphDefinitionLink>) -> Self {
        let mut resource = self.clone();
        resource.link = Some(value);
        resource
    }
    fn add_link(self, item: GraphDefinitionLink) -> Self {
        let mut resource = self.clone();
        resource.link.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::graph_definition::GraphDefinitionExistence for GraphDefinition {
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
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        true
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
    fn has_start(&self) -> bool {
        true
    }
    fn has_profile(&self) -> bool {
        self.profile.is_some()
    }
    fn has_link(&self) -> bool {
        self.link.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for GraphDefinition {
    fn resource_type(&self) -> &'static str {
        "GraphDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/GraphDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::graph_definition::{
    GraphDefinitionAccessors, GraphDefinitionExistence, GraphDefinitionMutators,
};
