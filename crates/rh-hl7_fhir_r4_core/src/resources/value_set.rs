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
use crate::primitives::date::DateType;
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
    pub locked_date: Option<DateType>,
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
    rh_foundation::Invariant::new("vsd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("vsd-1", rh_foundation::Severity::Error, "A value set include/exclude SHALL have a value set or a system", "valueSet.exists() or system.exists()").with_xpath("exists(f:valueSet) or exists(f:system)"),
    rh_foundation::Invariant::new("vsd-10", rh_foundation::Severity::Error, "Must have a system if a code is present", "code.empty() or system.exists()").with_xpath("exists(f:system) or not(exists(f:code))"),
    rh_foundation::Invariant::new("vsd-2", rh_foundation::Severity::Error, "A value set with concepts or filters SHALL include a system", "(concept.exists() or filter.exists()) implies system.exists()").with_xpath("not(exists(f:concept) or exists(f:filter)) or exists(f:system)"),
    rh_foundation::Invariant::new("vsd-3", rh_foundation::Severity::Error, "Cannot have both concept and filter", "concept.empty() or filter.empty()").with_xpath("not(exists(f:concept)) or not(exists(f:filter))"),
    rh_foundation::Invariant::new("vsd-6", rh_foundation::Severity::Error, "SHALL have a code or a display", "code.exists() or display.exists()").with_xpath("exists(f:code) or exists(f:display)"),
    rh_foundation::Invariant::new("vsd-9", rh_foundation::Severity::Error, "Must have a code if not abstract", "code.exists() or abstract = true").with_xpath("exists(f:code) or (f:abstract/@value = true())"),
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
                "ValueSet.compose.include.filter.op",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/filter-operator|4.0.1",
            )
            .with_description(
                "The kind of operation to perform as a part of a property based filter.",
            ),
            rh_foundation::ElementBinding::new(
                "ValueSet.status",
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
            rh_foundation::ElementCardinality::new("ValueSet.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.contained", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.contact", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.immutable", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.lockedDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.inactive", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include", 1, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.system", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.concept", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.code",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.use",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.concept.designation.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.filter", 0, None),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.property",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.op",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.compose.include.filter.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.compose.include.valueSet", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.compose.exclude", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.extension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.identifier", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.timestamp", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.total", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.offset", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.parameter.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.parameter.value[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains", 0, None),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.system",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.abstract",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.inactive",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.version",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.code", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.display",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ValueSet.expansion.contains.designation",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ValueSet.expansion.contains.contains", 0, None),
        ]
    });

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

impl crate::validation::ValidatableResource for ValueSet {
    fn resource_type(&self) -> &'static str {
        "ValueSet"
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
        Some("http://hl7.org/fhir/StructureDefinition/ValueSet")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::value_set::{ValueSetAccessors, ValueSetExistence, ValueSetMutators};
