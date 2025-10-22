use crate::bindings::capability_statement_kind::CapabilityStatementKind;
use crate::bindings::code_search_support::CodeSearchSupport;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// TerminologyCapabilities
///
/// A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: TerminologyCapabilities
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilities {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this terminology capabilities, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business version of the terminology capabilities
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this terminology capabilities (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this terminology capabilities (human friendly)
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
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the terminology capabilities
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for terminology capabilities (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this terminology capabilities is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// instance | capability | requirements
    pub kind: CapabilityStatementKind,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// Software that is covered by this terminology capability statement
    pub software: Option<TerminologyCapabilitiesSoftware>,
    /// If this describes a specific instance
    pub implementation: Option<TerminologyCapabilitiesImplementation>,
    /// Whether lockedDate is supported
    #[serde(rename = "lockedDate")]
    pub locked_date: Option<BooleanType>,
    /// Extension element for the 'lockedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lockedDate")]
    pub _locked_date: Option<Element>,
    /// A code system supported by the server
    #[serde(rename = "codeSystem")]
    pub code_system: Option<Vec<TerminologyCapabilitiesCodesystem>>,
    /// Information about the [ValueSet/$expand](valueset-operation-expand.html) operation
    pub expansion: Option<TerminologyCapabilitiesExpansion>,
    /// explicit | all
    #[serde(rename = "codeSearch")]
    pub code_search: Option<CodeSearchSupport>,
    /// Extension element for the 'codeSearch' primitive field. Contains metadata and extensions.
    #[serde(rename = "_codeSearch")]
    pub _code_search: Option<Element>,
    /// Information about the [ValueSet/$validate-code](valueset-operation-validate-code.html) operation
    #[serde(rename = "validateCode")]
    pub validate_code: Option<TerminologyCapabilitiesValidatecode>,
    /// Information about the [ConceptMap/$translate](conceptmap-operation-translate.html) operation
    pub translation: Option<TerminologyCapabilitiesTranslation>,
    /// Information about the [ConceptMap/$closure](conceptmap-operation-closure.html) operation
    pub closure: Option<TerminologyCapabilitiesClosure>,
}
/// TerminologyCapabilities nested structure for the 'translation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesTranslation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether the client must identify the map
    #[serde(rename = "needsMap")]
    pub needs_map: BooleanType,
    /// Extension element for the 'needsMap' primitive field. Contains metadata and extensions.
    #[serde(rename = "_needsMap")]
    pub _needs_map: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'software' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesSoftware {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A name the software is known by
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Version covered by this statement
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
}
/// TerminologyCapabilitiesCodesystemVersion nested structure for the 'filter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesCodesystemVersionFilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code of the property supported
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Operations supported for the property
    pub op: Vec<StringType>,
    /// Extension element for the 'op' primitive field. Contains metadata and extensions.
    pub _op: Option<Element>,
}
/// TerminologyCapabilitiesExpansion nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesExpansionParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Expansion Parameter name
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Description of support for parameter
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'validateCode' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesValidatecode {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Whether translations are validated
    pub translations: BooleanType,
    /// Extension element for the 'translations' primitive field. Contains metadata and extensions.
    pub _translations: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'implementation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesImplementation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Describes this specific instance
    pub description: StringType,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Base URL for the implementation
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'expansion' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesExpansion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Supported expansion parameter
    pub parameter: Option<Vec<TerminologyCapabilitiesExpansionParameter>>,
    /// Whether the server can return nested value sets
    pub hierarchical: Option<BooleanType>,
    /// Extension element for the 'hierarchical' primitive field. Contains metadata and extensions.
    pub _hierarchical: Option<Element>,
    /// Whether the server supports paging on expansion
    pub paging: Option<BooleanType>,
    /// Extension element for the 'paging' primitive field. Contains metadata and extensions.
    pub _paging: Option<Element>,
    /// Allow request for incomplete expansions?
    pub incomplete: Option<BooleanType>,
    /// Extension element for the 'incomplete' primitive field. Contains metadata and extensions.
    pub _incomplete: Option<Element>,
    /// Documentation about text searching works
    #[serde(rename = "textFilter")]
    pub text_filter: Option<StringType>,
    /// Extension element for the 'textFilter' primitive field. Contains metadata and extensions.
    #[serde(rename = "_textFilter")]
    pub _text_filter: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'codeSystem' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesCodesystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Version of Code System supported
    pub version: Option<Vec<TerminologyCapabilitiesCodesystemVersion>>,
    /// URI for the Code System
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// Whether subsumption is supported
    pub subsumption: Option<BooleanType>,
    /// Extension element for the 'subsumption' primitive field. Contains metadata and extensions.
    pub _subsumption: Option<Element>,
}
/// TerminologyCapabilitiesCodesystem nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesCodesystemVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Version identifier for this version
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// If this is the default version for this code system
    #[serde(rename = "isDefault")]
    pub is_default: Option<BooleanType>,
    /// Extension element for the 'isDefault' primitive field. Contains metadata and extensions.
    #[serde(rename = "_isDefault")]
    pub _is_default: Option<Element>,
    /// If compositional grammar is supported
    pub compositional: Option<BooleanType>,
    /// Extension element for the 'compositional' primitive field. Contains metadata and extensions.
    pub _compositional: Option<Element>,
    /// Language Displays supported
    pub language: Option<Vec<StringType>>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Properties supported for $lookup
    pub property: Option<Vec<StringType>>,
    /// Extension element for the 'property' primitive field. Contains metadata and extensions.
    pub _property: Option<Element>,
}
/// TerminologyCapabilities nested structure for the 'closure' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyCapabilitiesClosure {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// If cross-system closure is supported
    pub translation: Option<BooleanType>,
    /// Extension element for the 'translation' primitive field. Contains metadata and extensions.
    pub _translation: Option<Element>,
}

impl Default for TerminologyCapabilities {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: DateTimeType::default(),
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
            kind: CapabilityStatementKind::default(),
            _kind: Default::default(),
            software: Default::default(),
            implementation: Default::default(),
            locked_date: Default::default(),
            _locked_date: Default::default(),
            code_system: Default::default(),
            expansion: Default::default(),
            code_search: Default::default(),
            _code_search: Default::default(),
            validate_code: Default::default(),
            translation: Default::default(),
            closure: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesTranslation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            needs_map: BooleanType::default(),
            _needs_map: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesSoftware {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
            version: Default::default(),
            _version: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesCodesystemVersionFilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            op: Default::default(),
            _op: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesExpansionParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesValidatecode {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            translations: Default::default(),
            _translations: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesImplementation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: StringType::default(),
            _description: Default::default(),
            url: Default::default(),
            _url: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesExpansion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            parameter: Default::default(),
            hierarchical: Default::default(),
            _hierarchical: Default::default(),
            paging: Default::default(),
            _paging: Default::default(),
            incomplete: Default::default(),
            _incomplete: Default::default(),
            text_filter: Default::default(),
            _text_filter: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesCodesystem {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            version: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
            subsumption: Default::default(),
            _subsumption: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesCodesystemVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            is_default: Default::default(),
            _is_default: Default::default(),
            compositional: Default::default(),
            _compositional: Default::default(),
            language: Default::default(),
            _language: Default::default(),
            property: Default::default(),
            _property: Default::default(),
        }
    }
}

impl Default for TerminologyCapabilitiesClosure {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            translation: Default::default(),
            _translation: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for TerminologyCapabilities {
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

impl crate::traits::resource::ResourceMutators for TerminologyCapabilities {
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

impl crate::traits::resource::ResourceExistence for TerminologyCapabilities {
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

impl crate::traits::domain_resource::DomainResourceAccessors for TerminologyCapabilities {
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

impl crate::traits::domain_resource::DomainResourceMutators for TerminologyCapabilities {
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

impl crate::traits::domain_resource::DomainResourceExistence for TerminologyCapabilities {
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

impl crate::traits::terminology_capabilities::TerminologyCapabilitiesAccessors
    for TerminologyCapabilities
{
    fn url(&self) -> Option<StringType> {
        self.url.clone()
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
    fn date(&self) -> DateTimeType {
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
    fn kind(&self) -> CapabilityStatementKind {
        self.kind.clone()
    }
    fn software(&self) -> Option<TerminologyCapabilitiesSoftware> {
        self.software.clone()
    }
    fn implementation(&self) -> Option<TerminologyCapabilitiesImplementation> {
        self.implementation.clone()
    }
    fn locked_date(&self) -> Option<BooleanType> {
        self.locked_date
    }
    fn code_system(&self) -> &[TerminologyCapabilitiesCodesystem] {
        self.code_system.as_deref().unwrap_or(&[])
    }
    fn expansion(&self) -> Option<TerminologyCapabilitiesExpansion> {
        self.expansion.clone()
    }
    fn code_search(&self) -> Option<CodeSearchSupport> {
        self.code_search.clone()
    }
    fn validate_code(&self) -> Option<TerminologyCapabilitiesValidatecode> {
        self.validate_code.clone()
    }
    fn translation(&self) -> Option<TerminologyCapabilitiesTranslation> {
        self.translation.clone()
    }
    fn closure(&self) -> Option<TerminologyCapabilitiesClosure> {
        self.closure.clone()
    }
}

impl crate::traits::terminology_capabilities::TerminologyCapabilitiesMutators
    for TerminologyCapabilities
{
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
    fn set_kind(self, value: CapabilityStatementKind) -> Self {
        let mut resource = self.clone();
        resource.kind = value;
        resource
    }
    fn set_software(self, value: TerminologyCapabilitiesSoftware) -> Self {
        let mut resource = self.clone();
        resource.software = Some(value);
        resource
    }
    fn set_implementation(self, value: TerminologyCapabilitiesImplementation) -> Self {
        let mut resource = self.clone();
        resource.implementation = Some(value);
        resource
    }
    fn set_locked_date(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.locked_date = Some(value);
        resource
    }
    fn set_code_system(self, value: Vec<TerminologyCapabilitiesCodesystem>) -> Self {
        let mut resource = self.clone();
        resource.code_system = Some(value);
        resource
    }
    fn add_code_system(self, item: TerminologyCapabilitiesCodesystem) -> Self {
        let mut resource = self.clone();
        resource.code_system.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_expansion(self, value: TerminologyCapabilitiesExpansion) -> Self {
        let mut resource = self.clone();
        resource.expansion = Some(value);
        resource
    }
    fn set_code_search(self, value: CodeSearchSupport) -> Self {
        let mut resource = self.clone();
        resource.code_search = Some(value);
        resource
    }
    fn set_validate_code(self, value: TerminologyCapabilitiesValidatecode) -> Self {
        let mut resource = self.clone();
        resource.validate_code = Some(value);
        resource
    }
    fn set_translation(self, value: TerminologyCapabilitiesTranslation) -> Self {
        let mut resource = self.clone();
        resource.translation = Some(value);
        resource
    }
    fn set_closure(self, value: TerminologyCapabilitiesClosure) -> Self {
        let mut resource = self.clone();
        resource.closure = Some(value);
        resource
    }
}

impl crate::traits::terminology_capabilities::TerminologyCapabilitiesExistence
    for TerminologyCapabilities
{
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
        true
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
    fn has_kind(&self) -> bool {
        true
    }
    fn has_software(&self) -> bool {
        self.software.is_some()
    }
    fn has_implementation(&self) -> bool {
        self.implementation.is_some()
    }
    fn has_locked_date(&self) -> bool {
        self.locked_date.is_some()
    }
    fn has_code_system(&self) -> bool {
        self.code_system.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_expansion(&self) -> bool {
        self.expansion.is_some()
    }
    fn has_code_search(&self) -> bool {
        self.code_search.is_some()
    }
    fn has_validate_code(&self) -> bool {
        self.validate_code.is_some()
    }
    fn has_translation(&self) -> bool {
        self.translation.is_some()
    }
    fn has_closure(&self) -> bool {
        self.closure.is_some()
    }
}
