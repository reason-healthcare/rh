use crate::bindings::all_types::AllTypes;
use crate::bindings::binding_strength::BindingStrength;
use crate::bindings::operation_kind::OperationKind;
use crate::bindings::operation_parameter_use::OperationParameterUse;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::bindings::search_param_type::SearchParamType;
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
/// OperationDefinition
///
/// A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: OperationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this operation definition, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business version of the operation definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this operation definition (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this operation definition (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// operation | query
    pub kind: OperationKind,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
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
    /// Natural language description of the operation definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for operation definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this operation definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Whether content is changed by the operation
    #[serde(rename = "affectsState")]
    pub affects_state: Option<BooleanType>,
    /// Extension element for the 'affectsState' primitive field. Contains metadata and extensions.
    #[serde(rename = "_affectsState")]
    pub _affects_state: Option<Element>,
    /// Name used to invoke the operation
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Additional information about use
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// Marks this as a profile of the base
    #[serde(rename = "base")]
    pub base_definition: Option<StringType>,
    /// Extension element for the 'base' primitive field. Contains metadata and extensions.
    pub _base: Option<Element>,
    /// Types this operation applies to
    pub resource: Option<Vec<ResourceTypes>>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    pub _resource: Option<Element>,
    /// Invoke at the system level?
    pub system: BooleanType,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// Invoke at the type level?
    #[serde(rename = "type")]
    pub type_: BooleanType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Invoke on an instance?
    pub instance: BooleanType,
    /// Extension element for the 'instance' primitive field. Contains metadata and extensions.
    pub _instance: Option<Element>,
    /// Validation information for in parameters
    #[serde(rename = "inputProfile")]
    pub input_profile: Option<StringType>,
    /// Extension element for the 'inputProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_inputProfile")]
    pub _input_profile: Option<Element>,
    /// Validation information for out parameters
    #[serde(rename = "outputProfile")]
    pub output_profile: Option<StringType>,
    /// Extension element for the 'outputProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_outputProfile")]
    pub _output_profile: Option<Element>,
    /// Parameters for the operation/query
    pub parameter: Option<Vec<OperationDefinitionParameter>>,
    /// Define overloaded variants for when  generating code
    pub overload: Option<Vec<OperationDefinitionOverload>>,
}
/// OperationDefinitionParameter nested structure for the 'binding' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinitionParameterBinding {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// required | extensible | preferred | example
    pub strength: BindingStrength,
    /// Extension element for the 'strength' primitive field. Contains metadata and extensions.
    pub _strength: Option<Element>,
    /// Source of value set
    #[serde(rename = "valueSet")]
    pub value_set: StringType,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
}
/// OperationDefinition nested structure for the 'overload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinitionOverload {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of parameter to include in overload
    #[serde(rename = "parameterName")]
    pub parameter_name: Option<Vec<StringType>>,
    /// Extension element for the 'parameterName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_parameterName")]
    pub _parameter_name: Option<Element>,
    /// Comments to go on overload
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}
/// OperationDefinitionParameter nested structure for the 'referencedFrom' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinitionParameterReferencedfrom {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Referencing parameter
    pub source: StringType,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// Element id of reference
    #[serde(rename = "sourceId")]
    pub source_id: Option<StringType>,
    /// Extension element for the 'sourceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_sourceId")]
    pub _source_id: Option<Element>,
}
/// OperationDefinition nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinitionParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// ValueSet details if this is coded
    pub binding: Option<OperationDefinitionParameterBinding>,
    /// References to this parameter
    #[serde(rename = "referencedFrom")]
    pub referenced_from: Option<Vec<OperationDefinitionParameterReferencedfrom>>,
    /// Name in Parameters.parameter.name or in URL
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// in | out
    #[serde(rename = "use")]
    pub use_: OperationParameterUse,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// Minimum Cardinality
    pub min: IntegerType,
    /// Extension element for the 'min' primitive field. Contains metadata and extensions.
    pub _min: Option<Element>,
    /// Maximum Cardinality (a number or *)
    pub max: StringType,
    /// Extension element for the 'max' primitive field. Contains metadata and extensions.
    pub _max: Option<Element>,
    /// Description of meaning/use
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
    /// What type this parameter has
    #[serde(rename = "type")]
    pub type_: Option<AllTypes>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// If type is Reference | canonical, allowed targets
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<StringType>>,
    /// Extension element for the 'targetProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetProfile")]
    pub _target_profile: Option<Element>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "searchType")]
    pub search_type: Option<SearchParamType>,
    /// Extension element for the 'searchType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchType")]
    pub _search_type: Option<Element>,
    /// Parts of a nested Parameter
    pub part: Option<Vec<StringType>>,
}

impl Default for OperationDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            kind: OperationKind::default(),
            _kind: Default::default(),
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
            affects_state: Default::default(),
            _affects_state: Default::default(),
            code: StringType::default(),
            _code: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            base_definition: Default::default(),
            _base: Default::default(),
            resource: Default::default(),
            _resource: Default::default(),
            system: BooleanType::default(),
            _system: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            instance: BooleanType::default(),
            _instance: Default::default(),
            input_profile: Default::default(),
            _input_profile: Default::default(),
            output_profile: Default::default(),
            _output_profile: Default::default(),
            parameter: Default::default(),
            overload: Default::default(),
        }
    }
}

impl Default for OperationDefinitionParameterBinding {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            strength: Default::default(),
            _strength: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
        }
    }
}

impl Default for OperationDefinitionOverload {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            parameter_name: Default::default(),
            _parameter_name: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
        }
    }
}

impl Default for OperationDefinitionParameterReferencedfrom {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            source: Default::default(),
            _source: Default::default(),
            source_id: Default::default(),
            _source_id: Default::default(),
        }
    }
}

impl Default for OperationDefinitionParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            binding: Default::default(),
            referenced_from: Default::default(),
            name: StringType::default(),
            _name: Default::default(),
            use_: Default::default(),
            _use: Default::default(),
            min: IntegerType::default(),
            _min: Default::default(),
            max: StringType::default(),
            _max: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            target_profile: Default::default(),
            _target_profile: Default::default(),
            search_type: Default::default(),
            _search_type: Default::default(),
            part: Default::default(),
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
    rh_foundation::Invariant::new("opd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("opd-1", rh_foundation::Severity::Error, "Either a type must be provided, or parts", "type.exists() or part.exists()").with_xpath("exists(f:type) or exists(f:part)"),
    rh_foundation::Invariant::new("opd-2", rh_foundation::Severity::Error, "A search type can only be specified for parameters of type string", "searchType.exists() implies type = 'string'").with_xpath("not(exists(f:searchType)) or (f:type/@value = 'string')"),
    rh_foundation::Invariant::new("opd-3", rh_foundation::Severity::Error, "A targetProfile can only be specified for parameters of type Reference or Canonical", "targetProfile.exists() implies (type = 'Reference' or type = 'canonical')").with_xpath("not(exists(f:targetProfile)) or ((f:type/@value = 'Reference') or (f:type/@value = 'canonical'))"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for OperationDefinition {
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

impl crate::traits::resource::ResourceMutators for OperationDefinition {
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

impl crate::traits::resource::ResourceExistence for OperationDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for OperationDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for OperationDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for OperationDefinition {
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

impl crate::traits::operation_definition::OperationDefinitionAccessors for OperationDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
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
    fn kind(&self) -> OperationKind {
        self.kind.clone()
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
    fn affects_state(&self) -> Option<BooleanType> {
        self.affects_state
    }
    fn code(&self) -> StringType {
        self.code.clone()
    }
    fn comment(&self) -> Option<StringType> {
        self.comment.clone()
    }
    fn base_definition(&self) -> Option<StringType> {
        self.base_definition.clone()
    }
    fn resource(&self) -> &[ResourceTypes] {
        self.resource.as_deref().unwrap_or(&[])
    }
    fn system(&self) -> BooleanType {
        self.system
    }
    fn type_(&self) -> BooleanType {
        self.type_
    }
    fn instance(&self) -> BooleanType {
        self.instance
    }
    fn input_profile(&self) -> Option<StringType> {
        self.input_profile.clone()
    }
    fn output_profile(&self) -> Option<StringType> {
        self.output_profile.clone()
    }
    fn parameter(&self) -> &[OperationDefinitionParameter] {
        self.parameter.as_deref().unwrap_or(&[])
    }
    fn overload(&self) -> &[OperationDefinitionOverload] {
        self.overload.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::operation_definition::OperationDefinitionMutators for OperationDefinition {
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
    fn set_kind(self, value: OperationKind) -> Self {
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
    fn set_affects_state(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.affects_state = Some(value);
        resource
    }
    fn set_code(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_comment(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.comment = Some(value);
        resource
    }
    fn set_base_definition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base_definition = Some(value);
        resource
    }
    fn set_resource(self, value: Vec<ResourceTypes>) -> Self {
        let mut resource = self.clone();
        resource.resource = Some(value);
        resource
    }
    fn add_resource(self, item: ResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.resource.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_system(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.system = value;
        resource
    }
    fn set_type_(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_instance(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.instance = value;
        resource
    }
    fn set_input_profile(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.input_profile = Some(value);
        resource
    }
    fn set_output_profile(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.output_profile = Some(value);
        resource
    }
    fn set_parameter(self, value: Vec<OperationDefinitionParameter>) -> Self {
        let mut resource = self.clone();
        resource.parameter = Some(value);
        resource
    }
    fn add_parameter(self, item: OperationDefinitionParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_overload(self, value: Vec<OperationDefinitionOverload>) -> Self {
        let mut resource = self.clone();
        resource.overload = Some(value);
        resource
    }
    fn add_overload(self, item: OperationDefinitionOverload) -> Self {
        let mut resource = self.clone();
        resource.overload.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::operation_definition::OperationDefinitionExistence for OperationDefinition {
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
    fn has_affects_state(&self) -> bool {
        self.affects_state.is_some()
    }
    fn has_code(&self) -> bool {
        true
    }
    fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    fn has_base_definition(&self) -> bool {
        self.base_definition.is_some()
    }
    fn has_resource(&self) -> bool {
        self.resource.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_system(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_instance(&self) -> bool {
        true
    }
    fn has_input_profile(&self) -> bool {
        self.input_profile.is_some()
    }
    fn has_output_profile(&self) -> bool {
        self.output_profile.is_some()
    }
    fn has_parameter(&self) -> bool {
        self.parameter.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_overload(&self) -> bool {
        self.overload.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for OperationDefinition {
    fn resource_type(&self) -> &'static str {
        "OperationDefinition"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/OperationDefinition")
    }
}
