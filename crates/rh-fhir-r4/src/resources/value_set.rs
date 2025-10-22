use crate::bindings::filter_operator::FilterOperator;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::decimal::DecimalType;
use crate::primitives::integer::IntegerType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ValueSet
///
/// A ValueSet resource instance specifies a set of codes drawn from one or more code systems, intended for use in a particular context. Value sets link between [[[CodeSystem]]] definitions and their use in [coded elements](terminologies.html).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ValueSet
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ValueSet
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this value set, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the value set (business identifier)
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the value set
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this value set (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this value set (human friendly)
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
    /// Natural language description of the value set
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for value set (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Indicates whether or not any change to the content logical definition may occur
    pub immutable: Option<BooleanType>,
    /// Extension element for the 'immutable' primitive field. Contains metadata and extensions.
    pub _immutable: Option<Element>,
    /// Why this value set is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Content logical definition of the value set (CLD)
    pub compose: Option<ValueSetCompose>,
    /// Used when the value set is "expanded"
    pub expansion: Option<ValueSetExpansion>,
}
/// ValueSet nested structure for the 'expansion' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetExpansion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Parameter that controlled the expansion process
    pub parameter: Option<Vec<ValueSetExpansionParameter>>,
    /// Codes in the value set
    pub contains: Option<Vec<ValueSetExpansionContains>>,
    /// Identifies the value set expansion (business identifier)
    pub identifier: Option<StringType>,
    /// Extension element for the 'identifier' primitive field. Contains metadata and extensions.
    pub _identifier: Option<Element>,
    /// Time ValueSet expansion happened
    pub timestamp: DateTimeType,
    /// Extension element for the 'timestamp' primitive field. Contains metadata and extensions.
    pub _timestamp: Option<Element>,
    /// Total number of codes in the expansion
    pub total: Option<IntegerType>,
    /// Extension element for the 'total' primitive field. Contains metadata and extensions.
    pub _total: Option<Element>,
    /// Offset at which this resource starts
    pub offset: Option<IntegerType>,
    /// Extension element for the 'offset' primitive field. Contains metadata and extensions.
    pub _offset: Option<Element>,
}
/// ValueSetExpansion nested structure for the 'parameter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetExpansionParameter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name as assigned by the client or server
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Value of the named parameter (string)
    #[serde(rename = "valueString")]
    pub value_string: Option<StringType>,
    /// Value of the named parameter (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<BooleanType>,
    /// Value of the named parameter (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<IntegerType>,
    /// Value of the named parameter (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<DecimalType>,
    /// Value of the named parameter (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: Option<StringType>,
    /// Value of the named parameter (code)
    #[serde(rename = "valueCode")]
    pub value_code: Option<StringType>,
    /// Value of the named parameter (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<DateTimeType>,
}
/// ValueSetExpansion nested structure for the 'contains' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetExpansionContains {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// System value for the code
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// If user cannot select this entry
    #[serde(rename = "abstract")]
    pub abstract_: Option<BooleanType>,
    /// Extension element for the 'abstract' primitive field. Contains metadata and extensions.
    pub _abstract: Option<Element>,
    /// If concept is inactive in the code system
    pub inactive: Option<BooleanType>,
    /// Extension element for the 'inactive' primitive field. Contains metadata and extensions.
    pub _inactive: Option<Element>,
    /// Version in which this code/display is defined
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Code - if blank, this is not a selectable code
    pub code: Option<StringType>,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// User display for the concept
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Additional representations for this item
    pub designation: Option<Vec<StringType>>,
    /// Codes contained under this entry
    pub contains: Option<Vec<StringType>>,
}
/// ValueSet nested structure for the 'compose' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetCompose {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Include one or more codes from a code system or other value set(s)
    pub include: Vec<ValueSetComposeInclude>,
    /// Fixed date for references with no specified version (transitive)
    #[serde(rename = "lockedDate")]
    pub locked_date: Option<StringType>,
    /// Extension element for the 'lockedDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lockedDate")]
    pub _locked_date: Option<Element>,
    /// Whether inactive codes are in the value set
    pub inactive: Option<BooleanType>,
    /// Extension element for the 'inactive' primitive field. Contains metadata and extensions.
    pub _inactive: Option<Element>,
    /// Explicitly exclude codes from a code system or other value sets
    pub exclude: Option<Vec<StringType>>,
}
/// ValueSetComposeIncludeConcept nested structure for the 'designation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetComposeIncludeConceptDesignation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Human language of the designation
    ///
    /// Binding: preferred (A human language.)
    ///
    /// Available values:
    /// - `ar`: Arabic
    /// - `bn`: Bengali
    /// - `cs`: Czech
    /// - `da`: Danish
    /// - `de`: German
    /// - `de-AT`: German (Austria)
    /// - `de-CH`: German (Switzerland)
    /// - `de-DE`: German (Germany)
    /// - `el`: Greek
    /// - `en`: English
    /// - ... and 46 more values
    pub language: Option<StringType>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
    /// Types of uses of designations
    ///
    /// Binding: extensible (Details of how a designation would be used.)
    ///
    /// Available values:
    /// - `900000000000003001`
    /// - `900000000000013009`
    #[serde(rename = "use")]
    pub use_: Option<Coding>,
    /// The text value for this designation
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// ValueSetComposeInclude nested structure for the 'filter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetComposeIncludeFilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A property/filter defined by the code system
    pub property: StringType,
    /// Extension element for the 'property' primitive field. Contains metadata and extensions.
    pub _property: Option<Element>,
    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | exists
    pub op: FilterOperator,
    /// Extension element for the 'op' primitive field. Contains metadata and extensions.
    pub _op: Option<Element>,
    /// Code from the system, or regex criteria, or boolean value for exists
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}
/// ValueSetComposeInclude nested structure for the 'concept' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetComposeIncludeConcept {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code or expression from system
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Text to display for this code for this value set in this valueset
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
}
/// ValueSetCompose nested structure for the 'include' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetComposeInclude {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The system the codes come from
    pub system: Option<StringType>,
    /// Extension element for the 'system' primitive field. Contains metadata and extensions.
    pub _system: Option<Element>,
    /// Specific version of the code system referred to
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Select the contents included in this value set
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<StringType>>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
}

impl Default for ValueSet {
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
            immutable: Default::default(),
            _immutable: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            compose: Default::default(),
            expansion: Default::default(),
        }
    }
}

impl Default for ValueSetExpansion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            parameter: Default::default(),
            contains: Default::default(),
            identifier: Default::default(),
            _identifier: Default::default(),
            timestamp: DateTimeType::default(),
            _timestamp: Default::default(),
            total: Default::default(),
            _total: Default::default(),
            offset: Default::default(),
            _offset: Default::default(),
        }
    }
}

impl Default for ValueSetExpansionParameter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            value_string: Default::default(),
            value_boolean: Default::default(),
            value_integer: Default::default(),
            value_decimal: Default::default(),
            value_uri: Default::default(),
            value_code: Default::default(),
            value_date_time: Default::default(),
        }
    }
}

impl Default for ValueSetExpansionContains {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            system: Default::default(),
            _system: Default::default(),
            abstract_: Default::default(),
            _abstract: Default::default(),
            inactive: Default::default(),
            _inactive: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            designation: Default::default(),
            contains: Default::default(),
        }
    }
}

impl Default for ValueSetCompose {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            include: Vec::new(),
            locked_date: Default::default(),
            _locked_date: Default::default(),
            inactive: Default::default(),
            _inactive: Default::default(),
            exclude: Default::default(),
        }
    }
}

impl Default for ValueSetComposeIncludeConceptDesignation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            language: Default::default(),
            _language: Default::default(),
            use_: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for ValueSetComposeIncludeFilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            property: Default::default(),
            _property: Default::default(),
            op: Default::default(),
            _op: Default::default(),
            value: Default::default(),
            _value: Default::default(),
        }
    }
}

impl Default for ValueSetComposeIncludeConcept {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
        }
    }
}

impl Default for ValueSetComposeInclude {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            system: Default::default(),
            _system: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ValueSet {
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

impl crate::traits::resource::ResourceMutators for ValueSet {
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

impl crate::traits::resource::ResourceExistence for ValueSet {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ValueSet {
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

impl crate::traits::domain_resource::DomainResourceMutators for ValueSet {
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

impl crate::traits::domain_resource::DomainResourceExistence for ValueSet {
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

impl crate::traits::value_set::ValueSetAccessors for ValueSet {
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
    fn immutable(&self) -> Option<BooleanType> {
        self.immutable
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn compose(&self) -> Option<ValueSetCompose> {
        self.compose.clone()
    }
    fn expansion(&self) -> Option<ValueSetExpansion> {
        self.expansion.clone()
    }
}

impl crate::traits::value_set::ValueSetMutators for ValueSet {
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
    fn set_immutable(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.immutable = Some(value);
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
    fn set_compose(self, value: ValueSetCompose) -> Self {
        let mut resource = self.clone();
        resource.compose = Some(value);
        resource
    }
    fn set_expansion(self, value: ValueSetExpansion) -> Self {
        let mut resource = self.clone();
        resource.expansion = Some(value);
        resource
    }
}

impl crate::traits::value_set::ValueSetExistence for ValueSet {
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
    fn has_immutable(&self) -> bool {
        self.immutable.is_some()
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_compose(&self) -> bool {
        self.compose.is_some()
    }
    fn has_expansion(&self) -> bool {
        self.expansion.is_some()
    }
}
