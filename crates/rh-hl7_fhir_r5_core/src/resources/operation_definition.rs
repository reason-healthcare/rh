use crate::bindings::binding_strength::BindingStrength;
use crate::bindings::fhir_types::FhirTypes;
use crate::bindings::operation_kind::OperationKind;
use crate::bindings::operation_parameter_scope::OperationParameterScope;
use crate::bindings::operation_parameter_use::OperationParameterUse;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::search_param_type::SearchParamType;
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
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: OperationDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this operation definition, represented as an absolute URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the implementation guide (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the operation definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
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
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the operation definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for operation definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// Why this operation definition is defined
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
    /// Whether content is changed by the operation
    #[serde(rename = "affectsState")]
    pub affects_state: Option<BooleanType>,
    /// Extension element for the 'affectsState' primitive field. Contains metadata and extensions.
    #[serde(rename = "_affectsState")]
    pub _affects_state: Option<Element>,
    /// Recommended name for operation in search url
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<VersionIndependentAllResourceTypes>,
    /// Extension element for the 'resource' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _resource: Vec<Element>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<OperationDefinitionParameter>,
    /// Define overloaded variants for when  generating code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overload: Vec<OperationDefinitionOverload>,
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
/// OperationDefinition nested structure for the 'overload' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDefinitionOverload {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Name of parameter to include in overload
    #[serde(rename = "parameterName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_name: Vec<StringType>,
    /// Extension element for the 'parameterName' primitive field. Contains metadata and extensions.
    #[serde(rename = "_parameterName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _parameter_name: Vec<Element>,
    /// Comments to go on overload
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_from: Vec<OperationDefinitionParameterReferencedfrom>,
    /// Name in Parameters.parameter.name or in URL
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// in | out
    #[serde(rename = "use")]
    pub use_: OperationParameterUse,
    /// Extension element for the 'use' primitive field. Contains metadata and extensions.
    pub _use: Option<Element>,
    /// instance | type | system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<OperationParameterScope>,
    /// Extension element for the 'scope' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _scope: Vec<Element>,
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
    pub type_: Option<FhirTypes>,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Allowed sub-type this parameter can have (if type is abstract)
    #[serde(rename = "allowedType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_type: Vec<FhirTypes>,
    /// Extension element for the 'allowedType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_allowedType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _allowed_type: Vec<Element>,
    /// If type is Reference | canonical, allowed targets. If type is 'Resource', then this constrains the allowed resource types
    #[serde(rename = "targetProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile: Vec<StringType>,
    /// Extension element for the 'targetProfile' primitive field. Contains metadata and extensions.
    #[serde(rename = "_targetProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _target_profile: Vec<Element>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "searchType")]
    pub search_type: Option<SearchParamType>,
    /// Extension element for the 'searchType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_searchType")]
    pub _search_type: Option<Element>,
    /// Parts of a nested Parameter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<StringType>,
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

impl Default for OperationDefinition {
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
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
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
            scope: Default::default(),
            _scope: Default::default(),
            min: IntegerType::default(),
            _min: Default::default(),
            max: StringType::default(),
            _max: Default::default(),
            documentation: Default::default(),
            _documentation: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            allowed_type: Default::default(),
            _allowed_type: Default::default(),
            target_profile: Default::default(),
            _target_profile: Default::default(),
            search_type: Default::default(),
            _search_type: Default::default(),
            part: Default::default(),
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
    rh_foundation::Invariant::new("opd-1", rh_foundation::Severity::Error, "Either a type must be provided, or parts", "type.exists() or part.exists()"),
    rh_foundation::Invariant::new("opd-2", rh_foundation::Severity::Error, "A search type can only be specified for parameters of type string", "searchType.exists() implies type = 'string'"),
    rh_foundation::Invariant::new("opd-3", rh_foundation::Severity::Error, "A targetProfile can only be specified for parameters of type Reference, Canonical, or a Resource", "targetProfile.exists() implies (type = 'Reference' or type = 'canonical' or type.memberOf('http://hl7.org/fhir/ValueSet/resource-types'))"),
    rh_foundation::Invariant::new("opd-4", rh_foundation::Severity::Error, "SearchParamType can only be specified on in parameters", "(use = 'out') implies searchType.empty()"),
    rh_foundation::Invariant::new("opd-5", rh_foundation::Severity::Error, "A query operation cannot be defined at the instance level", "(kind = 'query') implies (instance = false)"),
    rh_foundation::Invariant::new("opd-6", rh_foundation::Severity::Error, "A query operation requires input parameters to have a search type", "(kind = 'query') implies (parameter.all((use = 'in' and searchType.exists()) or (use != 'in')))"),
    rh_foundation::Invariant::new("opd-7", rh_foundation::Severity::Error, "Named queries always have a single output parameter named 'result' of type Bundle", "(kind = 'query') implies ((parameter.where(use = 'out').count() = 1) and (parameter.where(use = 'out').all(name = 'result' and type = 'Bundle')))"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::ElementBinding::new("OperationDefinition.kind", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/operation-kind|5.0.0").with_description("Whether an operation is a normal operation or a query."),
    rh_foundation::ElementBinding::new("OperationDefinition.language", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/all-languages|5.0.0").with_description("IETF language tag for a human language"),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.allowedType", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/fhir-types|5.0.0").with_description("A list of all the types defined in this version of the FHIR specification - Abstract Types, Data Types and Resource Types."),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.binding.strength", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/binding-strength|5.0.0").with_description("Indication of the degree of conformance expectations associated with a binding."),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.scope", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/operation-parameter-scope|5.0.0"),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.searchType", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/search-param-type|5.0.0").with_description("Data types allowed to be used for search parameters."),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/fhir-types|5.0.0").with_description("A list of all the types defined in this version of the FHIR specification - Abstract Types, Data Types and Resource Types."),
    rh_foundation::ElementBinding::new("OperationDefinition.parameter.use", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/operation-parameter-use|5.0.0").with_description("Whether an operation parameter is an input or an output parameter."),
    rh_foundation::ElementBinding::new("OperationDefinition.resource", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/version-independent-all-resource-types|5.0.0").with_description("A type of resource, or a Reference (from all versions)"),
    rh_foundation::ElementBinding::new("OperationDefinition.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|5.0.0").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("OperationDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("OperationDefinition.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OperationDefinition.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.contact", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.useContext", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.copyrightLabel",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OperationDefinition.affectsState", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.base", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.resource", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.system", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.instance", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.inputProfile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.outputProfile", 0, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.name",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.use", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.scope", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.min", 1, Some(1)),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.max", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.documentation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.allowedType",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.targetProfile",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.searchType",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding.strength",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.binding.valueSet",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom.source",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.parameter.referencedFrom.sourceId",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("OperationDefinition.parameter.part", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.overload", 0, None),
            rh_foundation::ElementCardinality::new("OperationDefinition.overload.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.overload.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.overload.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.overload.parameterName",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "OperationDefinition.overload.comment",
                0,
                Some(1),
            ),
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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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

impl crate::traits::domain_resource::DomainResourceExistence for OperationDefinition {
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

impl crate::traits::operation_definition::OperationDefinitionAccessors for OperationDefinition {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
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
    fn resource(&self) -> &[VersionIndependentAllResourceTypes] {
        self.resource.as_slice()
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
        self.parameter.as_slice()
    }
    fn overload(&self) -> &[OperationDefinitionOverload] {
        self.overload.as_slice()
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
    fn set_resource(self, value: Vec<VersionIndependentAllResourceTypes>) -> Self {
        let mut resource = self.clone();
        resource.resource = value;
        resource
    }
    fn add_resource(self, item: VersionIndependentAllResourceTypes) -> Self {
        let mut resource = self.clone();
        resource.resource.push(item);
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
        resource.parameter = value;
        resource
    }
    fn add_parameter(self, item: OperationDefinitionParameter) -> Self {
        let mut resource = self.clone();
        resource.parameter.push(item);
        resource
    }
    fn set_overload(self, value: Vec<OperationDefinitionOverload>) -> Self {
        let mut resource = self.clone();
        resource.overload = value;
        resource
    }
    fn add_overload(self, item: OperationDefinitionOverload) -> Self {
        let mut resource = self.clone();
        resource.overload.push(item);
        resource
    }
}

impl crate::traits::operation_definition::OperationDefinitionExistence for OperationDefinition {
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
        !self.resource.is_empty()
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
        !self.parameter.is_empty()
    }
    fn has_overload(&self) -> bool {
        !self.overload.is_empty()
    }
}

impl crate::validation::ValidatableResource for OperationDefinition {
    fn resource_type(&self) -> &'static str {
        "OperationDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/OperationDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::operation_definition::{
    OperationDefinitionAccessors, OperationDefinitionExistence, OperationDefinitionMutators,
};
