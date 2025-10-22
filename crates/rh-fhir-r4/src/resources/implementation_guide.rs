use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::guide_page_generation::GuidePageGeneration;
use crate::bindings::guide_parameter_code::GuideParameterCode;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::resource_types::ResourceTypes;
use crate::bindings::spdx_license::SpdxLicense;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ImplementationGuide
///
/// A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ImplementationGuide
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ImplementationGuide
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuide {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this implementation guide, represented as a URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business version of the implementation guide
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this implementation guide (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this implementation guide (human friendly)
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
    /// Name of the publisher (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the implementation guide
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for implementation guide (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// NPM Package name for IG
    #[serde(rename = "packageId")]
    pub package_id: StringType,
    /// Extension element for the 'packageId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_packageId")]
    pub _package_id: Option<Element>,
    /// SPDX license code for this IG (or not-open-source)
    pub license: Option<SpdxLicense>,
    /// Extension element for the 'license' primitive field. Contains metadata and extensions.
    pub _license: Option<Element>,
    /// FHIR Version(s) this Implementation Guide targets
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Vec<FHIRVersion>,
    /// Extension element for the 'fhirVersion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fhirVersion")]
    pub _fhir_version: Option<Element>,
    /// Another Implementation guide this depends on
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<ImplementationGuideDependson>>,
    /// Profiles that apply globally
    pub global: Option<Vec<ImplementationGuideGlobal>>,
    /// Information needed to build the IG
    pub definition: Option<ImplementationGuideDefinition>,
    /// Information about an assembled IG
    pub manifest: Option<ImplementationGuideManifest>,
}
/// ImplementationGuide nested structure for the 'manifest' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideManifest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Resource in the implementation guide
    pub resource: Vec<ImplementationGuideManifestResource>,
    /// HTML page within the parent IG
    pub page: Option<Vec<ImplementationGuideManifestPage>>,
    /// Location of rendered implementation guide
    pub rendering: Option<StringType>,
    /// Extension element for the 'rendering' primitive field. Contains metadata and extensions.
    pub _rendering: Option<Element>,
    /// Image within the IG
    pub image: Option<Vec<StringType>>,
    /// Extension element for the 'image' primitive field. Contains metadata and extensions.
    pub _image: Option<Element>,
    /// Additional linkable file in IG
    pub other: Option<Vec<StringType>>,
    /// Extension element for the 'other' primitive field. Contains metadata and extensions.
    pub _other: Option<Element>,
}
/// ImplementationGuideDefinition nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinitionParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// apply | path-resource | path-pages | path-tx-cache | expansion-parameter | rule-broken-links | generate-xml | generate-json | generate-turtle | html-template
    pub code: GuideParameterCode,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Value for named type
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// ImplementationGuide nested structure for the 'definition' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Resource in the implementation guide
    pub resource: Vec<ImplementationGuideDefinitionResource>,
    /// Grouping used to present related resources in the IG
    pub grouping: Option<Vec<ImplementationGuideDefinitionGrouping>>,
    /// Defines how IG is built by tools
    pub parameter: Option<Vec<ImplementationGuideDefinitionParameter>>,
    /// A template for building resources
    pub template: Option<Vec<ImplementationGuideDefinitionTemplate>>,
    /// Page/Section in the Guide
    pub page: Option<ImplementationGuideDefinitionPage>,
}
/// ImplementationGuideDefinition nested structure for the 'resource' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinitionResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Location of the resource
    pub reference: Reference,
    /// Versions this applies to (if different to IG)
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Option<Vec<FHIRVersion>>,
    /// Extension element for the 'fhirVersion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fhirVersion")]
    pub _fhir_version: Option<Element>,
    /// Human Name for the resource
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Reason why included in guide
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Is an example/What is this an example of? (boolean)
    #[serde(rename = "exampleBoolean")]
    pub example_boolean: Option<BooleanType>,
    /// Is an example/What is this an example of? (canonical)
    #[serde(rename = "exampleCanonical")]
    pub example_canonical: Option<StringType>,
    /// Grouping this is part of
    #[serde(rename = "groupingId")]
    pub grouping_id: Option<StringType>,
    /// Extension element for the 'groupingId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_groupingId")]
    pub _grouping_id: Option<Element>,
}
/// ImplementationGuideDefinition nested structure for the 'grouping' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinitionGrouping {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Descriptive name for the package
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Human readable text describing the package
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// ImplementationGuideManifest nested structure for the 'resource' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideManifestResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Location of the resource
    pub reference: Reference,
    /// Is an example/What is this an example of? (boolean)
    #[serde(rename = "exampleBoolean")]
    pub example_boolean: Option<BooleanType>,
    /// Is an example/What is this an example of? (canonical)
    #[serde(rename = "exampleCanonical")]
    pub example_canonical: Option<StringType>,
    /// Relative path for page in IG
    #[serde(rename = "relativePath")]
    pub relative_path: Option<StringType>,
    /// Extension element for the 'relativePath' primitive field. Contains metadata and extensions.
    #[serde(rename = "_relativePath")]
    pub _relative_path: Option<Element>,
}
/// ImplementationGuide nested structure for the 'global' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideGlobal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type this profile applies to
    #[serde(rename = "type")]
    pub type_: ResourceTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Profile that all resources must conform to
    pub profile: StringType,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
}
/// ImplementationGuideDefinition nested structure for the 'template' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinitionTemplate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Type of template specified
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// The source location for the template
    pub source: StringType,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// The scope in which the template applies
    pub scope: Option<StringType>,
    /// Extension element for the 'scope' primitive field. Contains metadata and extensions.
    pub _scope: Option<Element>,
}
/// ImplementationGuideManifest nested structure for the 'page' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideManifestPage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// HTML page name
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Title of the page, for references
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Anchor available on the page
    pub anchor: Option<Vec<StringType>>,
    /// Extension element for the 'anchor' primitive field. Contains metadata and extensions.
    pub _anchor: Option<Element>,
}
/// ImplementationGuideDefinition nested structure for the 'page' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDefinitionPage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Where to find that page (url)
    #[serde(rename = "nameUrl")]
    pub name_url: StringType,
    /// Where to find that page (Reference)
    #[serde(rename = "nameReference")]
    pub name_reference: Reference,
    /// Short title shown for navigational assistance
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// html | markdown | xml | generated
    pub generation: GuidePageGeneration,
    /// Extension element for the 'generation' primitive field. Contains metadata and extensions.
    pub _generation: Option<Element>,
    /// Nested Pages / Sections
    pub page: Option<Vec<StringType>>,
}
/// ImplementationGuide nested structure for the 'dependsOn' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuideDependson {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identity of the IG that this depends on
    pub uri: StringType,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// NPM Package name for IG this depends on
    #[serde(rename = "packageId")]
    pub package_id: Option<StringType>,
    /// Extension element for the 'packageId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_packageId")]
    pub _package_id: Option<Element>,
    /// Version of the IG
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
}

impl Default for ImplementationGuide {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: StringType::default(),
            _url: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            name: StringType::default(),
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
            copyright: Default::default(),
            _copyright: Default::default(),
            package_id: StringType::default(),
            _package_id: Default::default(),
            license: Default::default(),
            _license: Default::default(),
            fhir_version: Vec::new(),
            _fhir_version: Default::default(),
            depends_on: Default::default(),
            global: Default::default(),
            definition: Default::default(),
            manifest: Default::default(),
        }
    }
}

impl Default for ImplementationGuideManifest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            resource: Vec::new(),
            page: Default::default(),
            rendering: Default::default(),
            _rendering: Default::default(),
            image: Default::default(),
            _image: Default::default(),
            other: Default::default(),
            _other: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinitionParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinition {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            resource: Vec::new(),
            grouping: Default::default(),
            parameter: Default::default(),
            template: Default::default(),
            page: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinitionResource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            fhir_version: Default::default(),
            _fhir_version: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            example_boolean: Default::default(),
            example_canonical: Default::default(),
            grouping_id: Default::default(),
            _grouping_id: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinitionGrouping {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for ImplementationGuideManifestResource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            reference: Default::default(),
            example_boolean: Default::default(),
            example_canonical: Default::default(),
            relative_path: Default::default(),
            _relative_path: Default::default(),
        }
    }
}

impl Default for ImplementationGuideGlobal {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            profile: StringType::default(),
            _profile: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinitionTemplate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            scope: Default::default(),
            _scope: Default::default(),
        }
    }
}

impl Default for ImplementationGuideManifestPage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            anchor: Default::default(),
            _anchor: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDefinitionPage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name_url: Default::default(),
            name_reference: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            generation: Default::default(),
            _generation: Default::default(),
            page: Default::default(),
        }
    }
}

impl Default for ImplementationGuideDependson {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            uri: Default::default(),
            _uri: Default::default(),
            package_id: Default::default(),
            _package_id: Default::default(),
            version: Default::default(),
            _version: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ImplementationGuide {
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

impl crate::traits::resource::ResourceMutators for ImplementationGuide {
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

impl crate::traits::resource::ResourceExistence for ImplementationGuide {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ImplementationGuide {
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

impl crate::traits::domain_resource::DomainResourceMutators for ImplementationGuide {
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

impl crate::traits::domain_resource::DomainResourceExistence for ImplementationGuide {
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

impl crate::traits::implementation_guide::ImplementationGuideAccessors for ImplementationGuide {
    fn url(&self) -> StringType {
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
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn package_id(&self) -> StringType {
        self.package_id.clone()
    }
    fn license(&self) -> Option<SpdxLicense> {
        self.license.clone()
    }
    fn fhir_version(&self) -> &[FHIRVersion] {
        &self.fhir_version
    }
    fn depends_on(&self) -> &[ImplementationGuideDependson] {
        self.depends_on.as_deref().unwrap_or(&[])
    }
    fn global(&self) -> &[ImplementationGuideGlobal] {
        self.global.as_deref().unwrap_or(&[])
    }
    fn definition(&self) -> Option<ImplementationGuideDefinition> {
        self.definition.clone()
    }
    fn manifest(&self) -> Option<ImplementationGuideManifest> {
        self.manifest.clone()
    }
}

impl crate::traits::implementation_guide::ImplementationGuideMutators for ImplementationGuide {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = value;
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
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_package_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.package_id = value;
        resource
    }
    fn set_license(self, value: SpdxLicense) -> Self {
        let mut resource = self.clone();
        resource.license = Some(value);
        resource
    }
    fn set_fhir_version(self, value: Vec<FHIRVersion>) -> Self {
        let mut resource = self.clone();
        resource.fhir_version = value;
        resource
    }
    fn add_fhir_version(self, item: FHIRVersion) -> Self {
        let mut resource = self.clone();
        resource.fhir_version.push(item);
        resource
    }
    fn set_depends_on(self, value: Vec<ImplementationGuideDependson>) -> Self {
        let mut resource = self.clone();
        resource.depends_on = Some(value);
        resource
    }
    fn add_depends_on(self, item: ImplementationGuideDependson) -> Self {
        let mut resource = self.clone();
        resource.depends_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_global(self, value: Vec<ImplementationGuideGlobal>) -> Self {
        let mut resource = self.clone();
        resource.global = Some(value);
        resource
    }
    fn add_global(self, item: ImplementationGuideGlobal) -> Self {
        let mut resource = self.clone();
        resource.global.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_definition(self, value: ImplementationGuideDefinition) -> Self {
        let mut resource = self.clone();
        resource.definition = Some(value);
        resource
    }
    fn set_manifest(self, value: ImplementationGuideManifest) -> Self {
        let mut resource = self.clone();
        resource.manifest = Some(value);
        resource
    }
}

impl crate::traits::implementation_guide::ImplementationGuideExistence for ImplementationGuide {
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
        true
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
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_package_id(&self) -> bool {
        true
    }
    fn has_license(&self) -> bool {
        self.license.is_some()
    }
    fn has_fhir_version(&self) -> bool {
        !self.fhir_version.is_empty()
    }
    fn has_depends_on(&self) -> bool {
        self.depends_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_global(&self) -> bool {
        self.global.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_definition(&self) -> bool {
        self.definition.is_some()
    }
    fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }
}
