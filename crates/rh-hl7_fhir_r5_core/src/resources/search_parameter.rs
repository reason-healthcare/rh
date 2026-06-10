use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::search_comparator::SearchComparator;
use crate::bindings::search_modifier_code::SearchModifierCode;
use crate::bindings::search_param_type::SearchParamType;
use crate::bindings::search_processingmode::SearchProcessingmode;
use crate::bindings::version_independent_all_resource_types::VersionIndependentAllResourceTypes;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// SearchParameter
///
/// A search parameter that defines a named search item that can be used to search/filter on a resource.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: SearchParameter
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this search parameter, represented as a URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the search parameter (business identifier)
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the search parameter
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// Name for this search parameter (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this search parameter (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Original definition for the search parameter
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<StringType>,
    /// Extension element for the 'derivedFrom' primitive field. Contains metadata and extensions.
    #[serde(rename = "_derivedFrom")]
    pub _derived_from: Option<Element>,
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
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the search parameter
    pub description: StringType,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for search parameter (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this search parameter is defined
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
    /// Recommended name for parameter in search url
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// The resource type(s) this search parameter applies to
    #[serde(rename = "base")]
    pub base_definition: Vec<VersionIndependentAllResourceTypes>,
    /// Extension element for the 'base' primitive field. Contains metadata and extensions.
    pub _base: Option<Element>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "type")]
    pub type_: SearchParamType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// FHIRPath expression that extracts the values
    pub expression: Option<StringType>,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
    /// normal | phonetic | other
    #[serde(rename = "processingMode")]
    pub processing_mode: Option<SearchProcessingmode>,
    /// Extension element for the 'processingMode' primitive field. Contains metadata and extensions.
    #[serde(rename = "_processingMode")]
    pub _processing_mode: Option<Element>,
    /// FHIRPath expression that constraints the usage of this SearchParamete
    pub constraint: Option<StringType>,
    /// Extension element for the 'constraint' primitive field. Contains metadata and extensions.
    pub _constraint: Option<Element>,
    /// Types of resource (if a resource reference)
    pub target: Option<Vec<VersionIndependentAllResourceTypes>>,
    /// Extension element for the 'target' primitive field. Contains metadata and extensions.
    pub _target: Option<Element>,
    /// Allow multiple values per parameter (or)
    #[serde(rename = "multipleOr")]
    pub multiple_or: Option<BooleanType>,
    /// Extension element for the 'multipleOr' primitive field. Contains metadata and extensions.
    #[serde(rename = "_multipleOr")]
    pub _multiple_or: Option<Element>,
    /// Allow multiple parameters (and)
    #[serde(rename = "multipleAnd")]
    pub multiple_and: Option<BooleanType>,
    /// Extension element for the 'multipleAnd' primitive field. Contains metadata and extensions.
    #[serde(rename = "_multipleAnd")]
    pub _multiple_and: Option<Element>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<Vec<SearchComparator>>,
    /// Extension element for the 'comparator' primitive field. Contains metadata and extensions.
    pub _comparator: Option<Element>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<Vec<SearchModifierCode>>,
    /// Extension element for the 'modifier' primitive field. Contains metadata and extensions.
    pub _modifier: Option<Element>,
    /// Chained names supported
    pub chain: Option<Vec<StringType>>,
    /// Extension element for the 'chain' primitive field. Contains metadata and extensions.
    pub _chain: Option<Element>,
    /// For Composite resources to define the parts
    pub component: Option<Vec<SearchParameterComponent>>,
}
/// SearchParameter nested structure for the 'component' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParameterComponent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Defines how the part works
    pub definition: StringType,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Subexpression relative to main expression
    pub expression: StringType,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
}

impl Default for SearchParameter {
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
            name: StringType::default(),
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
            description: StringType::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            code: StringType::default(),
            _code: Default::default(),
            base_definition: Default::default(),
            _base: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            expression: Default::default(),
            _expression: Default::default(),
            processing_mode: Default::default(),
            _processing_mode: Default::default(),
            constraint: Default::default(),
            _constraint: Default::default(),
            target: Default::default(),
            _target: Default::default(),
            multiple_or: Default::default(),
            _multiple_or: Default::default(),
            multiple_and: Default::default(),
            _multiple_and: Default::default(),
            comparator: Default::default(),
            _comparator: Default::default(),
            modifier: Default::default(),
            _modifier: Default::default(),
            chain: Default::default(),
            _chain: Default::default(),
            component: Default::default(),
        }
    }
}

impl Default for SearchParameterComponent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            definition: StringType::default(),
            _definition: Default::default(),
            expression: StringType::default(),
            _expression: Default::default(),
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
    rh_foundation::Invariant::new("spd-1", rh_foundation::Severity::Error, "If an expression is present, there SHALL be a processingMode", "expression.empty() or processingMode.exists()"),
    rh_foundation::Invariant::new("spd-2", rh_foundation::Severity::Error, "Search parameters can only have chain names when the search parameter type is 'reference'", "chain.empty() or type = 'reference'"),
    rh_foundation::Invariant::new("spd-3", rh_foundation::Severity::Error, "Search parameters comparator can only be used on type 'number', 'date', 'quantity' or 'special'.", "comparator.empty() or (type in ('number' | 'date' | 'quantity' | 'special'))"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("SearchParameter.base", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/version-independent-all-resource-types|5.0.0").with_description("A type of resource, or a Reference (from all versions)"),
    rh_foundation::ElementBinding::new("SearchParameter.comparator", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-comparator|5.0.0").with_description("What Search Comparator Codes are supported in search."),
    rh_foundation::ElementBinding::new("SearchParameter.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("SearchParameter.modifier", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-modifier-code|5.0.0").with_description("A supported modifier for a search parameter."),
    rh_foundation::ElementBinding::new("SearchParameter.processingMode", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-processingmode|5.0.0").with_description("How a search parameter relates to the set of elements returned by evaluating its expression query."),
    rh_foundation::ElementBinding::new("SearchParameter.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
    rh_foundation::ElementBinding::new("SearchParameter.target", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/version-independent-all-resource-types|5.0.0").with_description("A type of resource, or a Reference (from all versions)"),
    rh_foundation::ElementBinding::new("SearchParameter.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-param-type|5.0.0").with_description("Data types allowed to be used for search parameters."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("SearchParameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.contained", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.extension", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.url", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.identifier", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "SearchParameter.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("SearchParameter.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.derivedFrom", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.contact", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.description", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.useContext", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.base", 1, None),
            rh_foundation::ElementCardinality::new("SearchParameter.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.expression", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.processingMode", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.constraint", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.target", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.multipleOr", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.multipleAnd", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.comparator", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.modifier", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.chain", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.component", 0, None),
            rh_foundation::ElementCardinality::new("SearchParameter.component.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("SearchParameter.component.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "SearchParameter.component.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "SearchParameter.component.definition",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "SearchParameter.component.expression",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for SearchParameter {
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

impl crate::traits::resource::ResourceMutators for SearchParameter {
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

impl crate::traits::resource::ResourceExistence for SearchParameter {
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

impl crate::traits::domain_resource::DomainResourceAccessors for SearchParameter {
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

impl crate::traits::domain_resource::DomainResourceMutators for SearchParameter {
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

impl crate::traits::domain_resource::DomainResourceExistence for SearchParameter {
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

impl crate::traits::search_parameter::SearchParameterAccessors for SearchParameter {
    fn url(&self) -> StringType {
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
    fn derived_from(&self) -> Option<StringType> {
        self.derived_from.clone()
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
    fn description(&self) -> StringType {
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
    fn code(&self) -> StringType {
        self.code.clone()
    }
    fn base_definition(&self) -> &[VersionIndependentAllResourceTypes] {
        &self.base_definition
    }
    fn type_(&self) -> SearchParamType {
        self.type_.clone()
    }
    fn expression(&self) -> Option<StringType> {
        self.expression.clone()
    }
    fn processing_mode(&self) -> Option<SearchProcessingmode> {
        self.processing_mode.clone()
    }
    fn constraint(&self) -> Option<StringType> {
        self.constraint.clone()
    }
    fn target(&self) -> &[VersionIndependentAllResourceTypes] {
        self.target.as_deref().unwrap_or(&[])
    }
    fn multiple_or(&self) -> Option<BooleanType> {
        self.multiple_or
    }
    fn multiple_and(&self) -> Option<BooleanType> {
        self.multiple_and
    }
    fn comparator(&self) -> &[SearchComparator] {
        self.comparator.as_deref().unwrap_or(&[])
    }
    fn modifier(&self) -> &[SearchModifierCode] {
        self.modifier.as_deref().unwrap_or(&[])
    }
    fn chain(&self) -> &[StringType] {
        self.chain.as_deref().unwrap_or(&[])
    }
    fn component(&self) -> &[SearchParameterComponent] {
        self.component.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::search_parameter::SearchParameterMutators for SearchParameter {
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
        resource.name = value;
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_derived_from(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.derived_from = Some(value);
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
        resource.description = value;
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
    fn set_code(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.code = value;
        resource
    }
    fn set_base_definition(self, value: Vec<VersionIndependentAllResourceTypes>) -> Self {
        let mut resource = self.clone();
        resource.base_definition = value;
        resource
    }
    fn add_base_definition(self, item: VersionIndependentAllResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.base_definition.push(item);
        resource
    }
    fn set_type_(self, value: SearchParamType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_expression(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.expression = Some(value);
        resource
    }
    fn set_processing_mode(self, value: SearchProcessingmode) -> Self {
        let mut resource = self.clone();
        resource.processing_mode = Some(value);
        resource
    }
    fn set_constraint(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.constraint = Some(value);
        resource
    }
    fn set_target(self, value: Vec<VersionIndependentAllResourceTypes>) -> Self {
        let mut resource = self.clone();
        resource.target = Some(value);
        resource
    }
    fn add_target(self, item: VersionIndependentAllResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.target.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_multiple_or(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.multiple_or = Some(value);
        resource
    }
    fn set_multiple_and(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.multiple_and = Some(value);
        resource
    }
    fn set_comparator(self, value: Vec<SearchComparator>) -> Self {
        let mut resource = self.clone();
        resource.comparator = Some(value);
        resource
    }
    fn add_comparator(self, item: SearchComparator) -> Self {
        let mut resource = self.clone();
        resource.comparator.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_modifier(self, value: Vec<SearchModifierCode>) -> Self {
        let mut resource = self.clone();
        resource.modifier = Some(value);
        resource
    }
    fn add_modifier(self, item: SearchModifierCode) -> Self {
        let mut resource = self.clone();
        resource.modifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_chain(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.chain = Some(value);
        resource
    }
    fn add_chain(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.chain.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_component(self, value: Vec<SearchParameterComponent>) -> Self {
        let mut resource = self.clone();
        resource.component = Some(value);
        resource
    }
    fn add_component(self, item: SearchParameterComponent) -> Self {
        let mut resource = self.clone();
        resource.component.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::search_parameter::SearchParameterExistence for SearchParameter {
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
        true
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_derived_from(&self) -> bool {
        self.derived_from.is_some()
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
        true
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
    fn has_code(&self) -> bool {
        true
    }
    fn has_base_definition(&self) -> bool {
        !self.base_definition.is_empty()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_expression(&self) -> bool {
        self.expression.is_some()
    }
    fn has_processing_mode(&self) -> bool {
        self.processing_mode.is_some()
    }
    fn has_constraint(&self) -> bool {
        self.constraint.is_some()
    }
    fn has_target(&self) -> bool {
        self.target.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_multiple_or(&self) -> bool {
        self.multiple_or.is_some()
    }
    fn has_multiple_and(&self) -> bool {
        self.multiple_and.is_some()
    }
    fn has_comparator(&self) -> bool {
        self.comparator.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_modifier(&self) -> bool {
        self.modifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_chain(&self) -> bool {
        self.chain.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_component(&self) -> bool {
        self.component.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for SearchParameter {
    fn resource_type(&self) -> &'static str {
        "SearchParameter"
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
        Some("http://hl7.org/fhir/StructureDefinition/SearchParameter")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::search_parameter::{
    SearchParameterAccessors, SearchParameterExistence, SearchParameterMutators,
};
