use crate::bindings::capability_statement_kind::CapabilityStatementKind;
use crate::bindings::conditional_delete_status::ConditionalDeleteStatus;
use crate::bindings::conditional_read_status::ConditionalReadStatus;
use crate::bindings::document_mode::DocumentMode;
use crate::bindings::event_capability_mode::EventCapabilityMode;
use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::mimetypes::Mimetypes;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::reference_handling_policy::ReferenceHandlingPolicy;
use crate::bindings::resource_types::ResourceTypes;
use crate::bindings::restful_capability_mode::RestfulCapabilityMode;
use crate::bindings::search_param_type::SearchParamType;
use crate::bindings::system_restful_interaction::SystemRestfulInteraction;
use crate::bindings::type_restful_interaction::TypeRestfulInteraction;
use crate::bindings::versioning_policy::VersioningPolicy;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CapabilityStatement
///
/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CapabilityStatement
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatement {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Business version of the capability statement
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this capability statement (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this capability statement (human friendly)
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
    /// Natural language description of the capability statement
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for capability statement (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this capability statement is defined
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
    /// Canonical URL of another capability statement this implements
    pub instantiates: Option<Vec<StringType>>,
    /// Extension element for the 'instantiates' primitive field. Contains metadata and extensions.
    pub _instantiates: Option<Element>,
    /// Canonical URL of another capability statement this adds to
    pub imports: Option<Vec<StringType>>,
    /// Extension element for the 'imports' primitive field. Contains metadata and extensions.
    pub _imports: Option<Element>,
    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,
    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,
    /// FHIR Version the system supports
    #[serde(rename = "fhirVersion")]
    pub fhir_version: FHIRVersion,
    /// Extension element for the 'fhirVersion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fhirVersion")]
    pub _fhir_version: Option<Element>,
    /// formats supported (xml | json | ttl | mime type)
    pub format: Vec<Mimetypes>,
    /// Extension element for the 'format' primitive field. Contains metadata and extensions.
    pub _format: Option<Element>,
    /// Patch formats supported
    #[serde(rename = "patchFormat")]
    pub patch_format: Option<Vec<Mimetypes>>,
    /// Extension element for the 'patchFormat' primitive field. Contains metadata and extensions.
    #[serde(rename = "_patchFormat")]
    pub _patch_format: Option<Element>,
    /// Implementation guides supported
    #[serde(rename = "implementationGuide")]
    pub implementation_guide: Option<Vec<StringType>>,
    /// Extension element for the 'implementationGuide' primitive field. Contains metadata and extensions.
    #[serde(rename = "_implementationGuide")]
    pub _implementation_guide: Option<Element>,
    /// If the endpoint is a RESTful one
    pub rest: Option<Vec<CapabilityStatementRest>>,
    /// If messaging is supported
    pub messaging: Option<Vec<CapabilityStatementMessaging>>,
    /// Document definition
    pub document: Option<Vec<CapabilityStatementDocument>>,
}
/// CapabilityStatementMessaging nested structure for the 'endpoint' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementMessagingEndpoint {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// http | ftp | mllp +
    ///
    /// Binding: extensible (The protocol used for message transport.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/message-transport
    pub protocol: Coding,
    /// Network address or identifier of the end-point
    pub address: StringType,
    /// Extension element for the 'address' primitive field. Contains metadata and extensions.
    pub _address: Option<Element>,
}
/// CapabilityStatement nested structure for the 'implementation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementImplementation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Describes this specific instance
    pub description: StringType,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Base URL for the installation
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Organization that manages the data
    pub custodian: Option<Reference>,
}
/// CapabilityStatement nested structure for the 'messaging' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementMessaging {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Where messages should be sent
    pub endpoint: Option<Vec<CapabilityStatementMessagingEndpoint>>,
    /// Messages supported by this system
    #[serde(rename = "supportedMessage")]
    pub supported_message: Option<Vec<CapabilityStatementMessagingSupportedmessage>>,
    /// Reliable Message Cache Length (min)
    #[serde(rename = "reliableCache")]
    pub reliable_cache: Option<UnsignedIntType>,
    /// Extension element for the 'reliableCache' primitive field. Contains metadata and extensions.
    #[serde(rename = "_reliableCache")]
    pub _reliable_cache: Option<Element>,
    /// Messaging interface behavior details
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// CapabilityStatementRest nested structure for the 'security' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestSecurity {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<BooleanType>,
    /// Extension element for the 'cors' primitive field. Contains metadata and extensions.
    pub _cors: Option<Element>,
    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    ///
    /// Binding: extensible (Types of security services used with FHIR.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/restful-security-service
    pub service: Option<Vec<CodeableConcept>>,
    /// General description of how security works
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// CapabilityStatementRest nested structure for the 'resource' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A resource type that is supported
    #[serde(rename = "type")]
    pub type_: ResourceTypes,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Base System profile for all uses of resource
    pub profile: Option<StringType>,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
    /// Profiles for use cases supported
    #[serde(rename = "supportedProfile")]
    pub supported_profile: Option<Vec<StringType>>,
    /// Extension element for the 'supportedProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_supportedProfile")]
    pub _supported_profile: Option<Element>,
    /// Additional information about the use of the resource type
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
    /// no-version | versioned | versioned-update
    pub versioning: Option<VersioningPolicy>,
    /// Extension element for the 'versioning' primitive field. Contains metadata and extensions.
    pub _versioning: Option<Element>,
    /// Whether vRead can return past versions
    #[serde(rename = "readHistory")]
    pub read_history: Option<BooleanType>,
    /// Extension element for the 'readHistory' primitive field. Contains metadata and extensions.
    #[serde(rename = "_readHistory")]
    pub _read_history: Option<Element>,
    /// If update can commit to a new identity
    #[serde(rename = "updateCreate")]
    pub update_create: Option<BooleanType>,
    /// Extension element for the 'updateCreate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_updateCreate")]
    pub _update_create: Option<Element>,
    /// If allows/uses conditional create
    #[serde(rename = "conditionalCreate")]
    pub conditional_create: Option<BooleanType>,
    /// Extension element for the 'conditionalCreate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_conditionalCreate")]
    pub _conditional_create: Option<Element>,
    /// not-supported | modified-since | not-match | full-support
    #[serde(rename = "conditionalRead")]
    pub conditional_read: Option<ConditionalReadStatus>,
    /// Extension element for the 'conditionalRead' primitive field. Contains metadata and extensions.
    #[serde(rename = "_conditionalRead")]
    pub _conditional_read: Option<Element>,
    /// If allows/uses conditional update
    #[serde(rename = "conditionalUpdate")]
    pub conditional_update: Option<BooleanType>,
    /// Extension element for the 'conditionalUpdate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_conditionalUpdate")]
    pub _conditional_update: Option<Element>,
    /// not-supported | single | multiple - how conditional delete is supported
    #[serde(rename = "conditionalDelete")]
    pub conditional_delete: Option<ConditionalDeleteStatus>,
    /// Extension element for the 'conditionalDelete' primitive field. Contains metadata and extensions.
    #[serde(rename = "_conditionalDelete")]
    pub _conditional_delete: Option<Element>,
    /// literal | logical | resolves | enforced | local
    #[serde(rename = "referencePolicy")]
    pub reference_policy: Option<Vec<ReferenceHandlingPolicy>>,
    /// Extension element for the 'referencePolicy' primitive field. Contains metadata and extensions.
    #[serde(rename = "_referencePolicy")]
    pub _reference_policy: Option<Element>,
    /// _include values supported by the server
    #[serde(rename = "searchInclude")]
    pub search_include: Option<Vec<StringType>>,
    /// Extension element for the 'searchInclude' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchInclude")]
    pub _search_include: Option<Element>,
    /// _revinclude values supported by the server
    #[serde(rename = "searchRevInclude")]
    pub search_rev_include: Option<Vec<StringType>>,
    /// Extension element for the 'searchRevInclude' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchRevInclude")]
    pub _search_rev_include: Option<Element>,
}
/// CapabilityStatement nested structure for the 'document' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementDocument {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// producer | consumer
    pub mode: DocumentMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Description of document support
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
    /// Constraint on the resources used in the document
    pub profile: StringType,
    /// Extension element for the 'profile' primitive field. Contains metadata and extensions.
    pub _profile: Option<Element>,
}
/// CapabilityStatementRest nested structure for the 'interaction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestInteraction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// transaction | batch | search-system | history-system
    pub code: SystemRestfulInteraction,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Anything special about operation behavior
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// CapabilityStatement nested structure for the 'software' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementSoftware {
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
    /// Date this version was released
    #[serde(rename = "releaseDate")]
    pub release_date: Option<DateTimeType>,
    /// Extension element for the 'releaseDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_releaseDate")]
    pub _release_date: Option<Element>,
}
/// CapabilityStatementRestResource nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestResourceOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name by which the operation/query is invoked
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// The defined operation/query
    pub definition: StringType,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Specific details about operation behavior
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// CapabilityStatementRestResource nested structure for the 'interaction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestResourceInteraction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    pub code: TypeRestfulInteraction,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Anything special about operation behavior
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// CapabilityStatementRestResource nested structure for the 'searchParam' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRestResourceSearchparam {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of search parameter
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Source of definition for parameter
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "type")]
    pub type_: SearchParamType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Server-specific usage
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
}
/// CapabilityStatement nested structure for the 'rest' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementRest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// What operations are supported?
    pub interaction: Option<Vec<CapabilityStatementRestInteraction>>,
    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,
    /// Resource served on the REST interface
    pub resource: Option<Vec<CapabilityStatementRestResource>>,
    /// client | server
    pub mode: RestfulCapabilityMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// General description of implementation
    pub documentation: Option<StringType>,
    /// Extension element for the 'documentation' primitive field. Contains metadata and extensions.
    pub _documentation: Option<Element>,
    /// Search parameters for searching all resources
    #[serde(rename = "searchParam")]
    pub search_param: Option<Vec<StringType>>,
    /// Definition of a system level operation
    pub operation: Option<Vec<StringType>>,
    /// Compartments served/used by system
    pub compartment: Option<Vec<StringType>>,
    /// Extension element for the 'compartment' primitive field. Contains metadata and extensions.
    pub _compartment: Option<Element>,
}
/// CapabilityStatementMessaging nested structure for the 'supportedMessage' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatementMessagingSupportedmessage {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// sender | receiver
    pub mode: EventCapabilityMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Message supported by this system
    pub definition: StringType,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
}

impl Default for CapabilityStatement {
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
            instantiates: Default::default(),
            _instantiates: Default::default(),
            imports: Default::default(),
            _imports: Default::default(),
            software: Default::default(),
            implementation: Default::default(),
            fhir_version: FHIRVersion::default(),
            _fhir_version: Default::default(),
            format: Vec::new(),
            _format: Default::default(),
            patch_format: Default::default(),
            _patch_format: Default::default(),
            implementation_guide: Default::default(),
            _implementation_guide: Default::default(),
            rest: Default::default(),
            messaging: Default::default(),
            document: Default::default(),
        }
    }
}

impl Default for CapabilityStatementMessagingEndpoint {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            protocol: Default::default(),
            address: Default::default(),
            _address: Default::default(),
        }
    }
}

impl Default for CapabilityStatementImplementation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: StringType::default(),
            _description: Default::default(),
            url: Default::default(),
            _url: Default::default(),
            custodian: Default::default(),
        }
    }
}

impl Default for CapabilityStatementMessaging {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            endpoint: Default::default(),
            supported_message: Default::default(),
            reliable_cache: Default::default(),
            _reliable_cache: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestSecurity {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            cors: Default::default(),
            _cors: Default::default(),
            service: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestResource {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            profile: Default::default(),
            _profile: Default::default(),
            supported_profile: Default::default(),
            _supported_profile: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            versioning: Default::default(),
            _versioning: Default::default(),
            read_history: Default::default(),
            _read_history: Default::default(),
            update_create: Default::default(),
            _update_create: Default::default(),
            conditional_create: Default::default(),
            _conditional_create: Default::default(),
            conditional_read: Default::default(),
            _conditional_read: Default::default(),
            conditional_update: Default::default(),
            _conditional_update: Default::default(),
            conditional_delete: Default::default(),
            _conditional_delete: Default::default(),
            reference_policy: Default::default(),
            _reference_policy: Default::default(),
            search_include: Default::default(),
            _search_include: Default::default(),
            search_rev_include: Default::default(),
            _search_rev_include: Default::default(),
        }
    }
}

impl Default for CapabilityStatementDocument {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: DocumentMode::default(),
            _mode: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            profile: StringType::default(),
            _profile: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestInteraction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for CapabilityStatementSoftware {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: StringType::default(),
            _name: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            release_date: Default::default(),
            _release_date: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestResourceOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestResourceInteraction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRestResourceSearchparam {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            name: Default::default(),
            _name: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
        }
    }
}

impl Default for CapabilityStatementRest {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            interaction: Default::default(),
            security: Default::default(),
            resource: Default::default(),
            mode: RestfulCapabilityMode::default(),
            _mode: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            search_param: Default::default(),
            operation: Default::default(),
            compartment: Default::default(),
            _compartment: Default::default(),
        }
    }
}

impl Default for CapabilityStatementMessagingSupportedmessage {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: Default::default(),
            _mode: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
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
    rh_foundation::Invariant::new("cpb-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("cpb-1", rh_foundation::Severity::Error, "A Capability Statement SHALL have at least one of REST, messaging or document element.", "rest.exists() or messaging.exists() or document.exists()").with_xpath("exists(f:rest) or exists(f:messaging) or exists(f:document)"),
    rh_foundation::Invariant::new("cpb-12", rh_foundation::Severity::Error, "Search parameter names must be unique in the context of a resource.", "searchParam.select(name).isDistinct()").with_xpath("count(f:searchParam)=count(distinct-values(f:searchParam/f:name/@value))"),
    rh_foundation::Invariant::new("cpb-14", rh_foundation::Severity::Error, "If kind = instance, implementation must be present and software may be present", "(kind != 'instance') or implementation.exists()").with_xpath("not(f:kind/@value='instance') or exists(f:implementation)"),
    rh_foundation::Invariant::new("cpb-15", rh_foundation::Severity::Error, "If kind = capability, implementation must be absent, software must be present", "(kind != 'capability') or (implementation.exists().not() and software.exists())").with_xpath(" not(f:kind/@value='instance') or (not(exists(f:implementation)) and exists(f:software))"),
    rh_foundation::Invariant::new("cpb-16", rh_foundation::Severity::Error, "If kind = requirements, implementation and software must be absent", "(kind!='requirements') or (implementation.exists().not() and software.exists().not())").with_xpath("not(f:kind/@value='instance') or (not(exists(f:implementation)) and not(exists(f:software)))"),
    rh_foundation::Invariant::new("cpb-2", rh_foundation::Severity::Error, "A Capability Statement SHALL have at least one of description, software, or implementation element.", "(description.count() + software.count() + implementation.count()) > 0").with_xpath("count(f:software | f:implementation | f:description) > 0"),
    rh_foundation::Invariant::new("cpb-3", rh_foundation::Severity::Error, "Messaging end-point is required (and is only permitted) when a statement is for an implementation.", "messaging.endpoint.empty() or kind = 'instance'").with_xpath("not(exists(f:messaging/f:endpoint)) or f:kind/@value = 'instance'"),
    rh_foundation::Invariant::new("cpb-7", rh_foundation::Severity::Error, "The set of documents must be unique by the combination of profile and mode.", "document.select(profile&mode).isDistinct()").with_xpath("count(f:document[f:mode/@value='producer'])=count(distinct-values(f:document[f:mode/@value='producer']/f:profile/f:reference/@value)) and count(f:document[f:mode/@value='consumer'])=count(distinct-values(f:document[f:mode/@value='consumer']/f:profile/f:reference/@value))"),
    rh_foundation::Invariant::new("cpb-9", rh_foundation::Severity::Error, "A given resource can only be described once per RESTful mode.", "resource.select(type).isDistinct()").with_xpath("count(f:resource)=count(distinct-values(f:resource/f:type/@value))"),
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
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.document.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/document-mode|4.0.1",
            )
            .with_description("Whether the application produces or consumes documents."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.fhirVersion",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/FHIR-version|4.0.1",
            )
            .with_description("All published FHIR Versions."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.format",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1",
            )
            .with_description("The mime type of an attachment. Any valid mime type is allowed."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.kind",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/capability-statement-kind|4.0.1",
            )
            .with_description("How a capability statement is intended to be used."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.messaging.supportedMessage.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/event-capability-mode|4.0.1",
            )
            .with_description("The mode of a message capability statement."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.patchFormat",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1",
            )
            .with_description("The mime type of an attachment. Any valid mime type is allowed."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.interaction.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/system-restful-interaction|4.0.1",
            )
            .with_description("Operations supported by REST at the system level."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.mode",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/restful-capability-mode|4.0.1",
            )
            .with_description("The mode of a RESTful capability statement."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.conditionalDelete",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/conditional-delete-status|4.0.1",
            )
            .with_description("A code that indicates how the server supports conditional delete."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.conditionalRead",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/conditional-read-status|4.0.1",
            )
            .with_description("A code that indicates how the server supports conditional read."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.interaction.code",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/type-restful-interaction|4.0.1",
            )
            .with_description("Operations supported by REST at the type or instance level."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.referencePolicy",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/reference-handling-policy|4.0.1",
            )
            .with_description("A set of flags that defines how references are supported."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.searchParam.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/search-param-type|4.0.1",
            )
            .with_description("Data types allowed to be used for search parameters."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/resource-types|4.0.1",
            )
            .with_description("One of the resource types defined as part of this version of FHIR."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.rest.resource.versioning",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/versioning-policy|4.0.1",
            )
            .with_description("How the system supports versioning for a resource."),
            rh_foundation::ElementBinding::new(
                "CapabilityStatement.status",
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
            rh_foundation::ElementCardinality::new("CapabilityStatement.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.contained", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.contact", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.useContext", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.instantiates", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.imports", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.software", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.software.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.software.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.software.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.software.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.software.version",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.software.releaseDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.description",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.url",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementation.custodian",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.fhirVersion", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.format", 1, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.patchFormat", 0, None),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.implementationGuide",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.security", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.cors",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.service",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.security.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.resource", 0, None),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.profile",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.supportedProfile",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.interaction.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.versioning",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.readHistory",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.updateCreate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.conditionalCreate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.conditionalRead",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.conditionalUpdate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.conditionalDelete",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.referencePolicy",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchInclude",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchRevInclude",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.name",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.definition",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.type",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.searchParam.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.name",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.definition",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.resource.operation.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.interaction", 0, None),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.interaction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.interaction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.interaction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.interaction.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.rest.interaction.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.searchParam", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.operation", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.rest.compartment", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.messaging", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.messaging.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint.protocol",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.endpoint.address",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.reliableCache",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage.mode",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.messaging.supportedMessage.definition",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.document", 0, None),
            rh_foundation::ElementCardinality::new("CapabilityStatement.document.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.document.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.document.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CapabilityStatement.document.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.document.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CapabilityStatement.document.profile",
                1,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CapabilityStatement {
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

impl crate::traits::resource::ResourceMutators for CapabilityStatement {
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

impl crate::traits::resource::ResourceExistence for CapabilityStatement {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CapabilityStatement {
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

impl crate::traits::domain_resource::DomainResourceMutators for CapabilityStatement {
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

impl crate::traits::domain_resource::DomainResourceExistence for CapabilityStatement {
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

impl crate::traits::capability_statement::CapabilityStatementAccessors for CapabilityStatement {
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
    fn instantiates(&self) -> &[StringType] {
        self.instantiates.as_deref().unwrap_or(&[])
    }
    fn imports(&self) -> &[StringType] {
        self.imports.as_deref().unwrap_or(&[])
    }
    fn software(&self) -> Option<CapabilityStatementSoftware> {
        self.software.clone()
    }
    fn implementation(&self) -> Option<CapabilityStatementImplementation> {
        self.implementation.clone()
    }
    fn fhir_version(&self) -> FHIRVersion {
        self.fhir_version.clone()
    }
    fn format(&self) -> &[Mimetypes] {
        &self.format
    }
    fn patch_format(&self) -> &[Mimetypes] {
        self.patch_format.as_deref().unwrap_or(&[])
    }
    fn implementation_guide(&self) -> &[StringType] {
        self.implementation_guide.as_deref().unwrap_or(&[])
    }
    fn rest(&self) -> &[CapabilityStatementRest] {
        self.rest.as_deref().unwrap_or(&[])
    }
    fn messaging(&self) -> &[CapabilityStatementMessaging] {
        self.messaging.as_deref().unwrap_or(&[])
    }
    fn document(&self) -> &[CapabilityStatementDocument] {
        self.document.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::capability_statement::CapabilityStatementMutators for CapabilityStatement {
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
    fn set_instantiates(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.instantiates = Some(value);
        resource
    }
    fn add_instantiates(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .instantiates
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_imports(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.imports = Some(value);
        resource
    }
    fn add_imports(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.imports.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_software(self, value: CapabilityStatementSoftware) -> Self {
        let mut resource = self.clone();
        resource.software = Some(value);
        resource
    }
    fn set_implementation(self, value: CapabilityStatementImplementation) -> Self {
        let mut resource = self.clone();
        resource.implementation = Some(value);
        resource
    }
    fn set_fhir_version(self, value: FHIRVersion) -> Self {
        let mut resource = self.clone();
        resource.fhir_version = value;
        resource
    }
    fn set_format(self, value: Vec<Mimetypes>) -> Self {
        let mut resource = self.clone();
        resource.format = value;
        resource
    }
    fn add_format(self, item: Mimetypes) -> Self {
        let mut resource = self.clone();
        resource.format.push(item);
        resource
    }
    fn set_patch_format(self, value: Vec<Mimetypes>) -> Self {
        let mut resource = self.clone();
        resource.patch_format = Some(value);
        resource
    }
    fn add_patch_format(self, item: Mimetypes) -> Self {
        let mut resource = self.clone();
        resource
            .patch_format
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_implementation_guide(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.implementation_guide = Some(value);
        resource
    }
    fn add_implementation_guide(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .implementation_guide
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_rest(self, value: Vec<CapabilityStatementRest>) -> Self {
        let mut resource = self.clone();
        resource.rest = Some(value);
        resource
    }
    fn add_rest(self, item: CapabilityStatementRest) -> Self {
        let mut resource = self.clone();
        resource.rest.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_messaging(self, value: Vec<CapabilityStatementMessaging>) -> Self {
        let mut resource = self.clone();
        resource.messaging = Some(value);
        resource
    }
    fn add_messaging(self, item: CapabilityStatementMessaging) -> Self {
        let mut resource = self.clone();
        resource.messaging.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_document(self, value: Vec<CapabilityStatementDocument>) -> Self {
        let mut resource = self.clone();
        resource.document = Some(value);
        resource
    }
    fn add_document(self, item: CapabilityStatementDocument) -> Self {
        let mut resource = self.clone();
        resource.document.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::capability_statement::CapabilityStatementExistence for CapabilityStatement {
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
    fn has_instantiates(&self) -> bool {
        self.instantiates.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_imports(&self) -> bool {
        self.imports.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_software(&self) -> bool {
        self.software.is_some()
    }
    fn has_implementation(&self) -> bool {
        self.implementation.is_some()
    }
    fn has_fhir_version(&self) -> bool {
        true
    }
    fn has_format(&self) -> bool {
        !self.format.is_empty()
    }
    fn has_patch_format(&self) -> bool {
        self.patch_format.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_implementation_guide(&self) -> bool {
        self.implementation_guide
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_rest(&self) -> bool {
        self.rest.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_messaging(&self) -> bool {
        self.messaging.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_document(&self) -> bool {
        self.document.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CapabilityStatement {
    fn resource_type(&self) -> &'static str {
        "CapabilityStatement"
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
        Some("http://hl7.org/fhir/StructureDefinition/CapabilityStatement")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::capability_statement::{
    CapabilityStatementAccessors, CapabilityStatementExistence, CapabilityStatementMutators,
};
