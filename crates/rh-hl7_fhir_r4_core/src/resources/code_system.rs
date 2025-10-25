use crate::bindings::codesystem_content_mode::CodesystemContentMode;
use crate::bindings::codesystem_hierarchy_meaning::CodesystemHierarchyMeaning;
use crate::bindings::concept_property_type::ConceptPropertyType;
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
use crate::primitives::unsigned_int::UnsignedIntType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// CodeSystem
///
/// The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/CodeSystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: CodeSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this code system, represented as a URI (globally unique) (Coding.system)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the code system (business identifier)
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the code system (Coding.version)
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this code system (computer friendly)
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this code system (human friendly)
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
    /// Natural language description of the code system
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for code system (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this code system is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// If code comparison is case sensitive
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<BooleanType>,
    /// Extension element for the 'caseSensitive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_caseSensitive")]
    pub _case_sensitive: Option<Element>,
    /// Canonical reference to the value set with entire code system
    #[serde(rename = "valueSet")]
    pub value_set: Option<StringType>,
    /// Extension element for the 'valueSet' primitive field. Contains metadata and extensions.
    #[serde(rename = "_valueSet")]
    pub _value_set: Option<Element>,
    /// grouped-by | is-a | part-of | classified-with
    #[serde(rename = "hierarchyMeaning")]
    pub hierarchy_meaning: Option<CodesystemHierarchyMeaning>,
    /// Extension element for the 'hierarchyMeaning' primitive field. Contains metadata and extensions.
    #[serde(rename = "_hierarchyMeaning")]
    pub _hierarchy_meaning: Option<Element>,
    /// If code system defines a compositional grammar
    pub compositional: Option<BooleanType>,
    /// Extension element for the 'compositional' primitive field. Contains metadata and extensions.
    pub _compositional: Option<Element>,
    /// If definitions are not stable
    #[serde(rename = "versionNeeded")]
    pub version_needed: Option<BooleanType>,
    /// Extension element for the 'versionNeeded' primitive field. Contains metadata and extensions.
    #[serde(rename = "_versionNeeded")]
    pub _version_needed: Option<Element>,
    /// not-present | example | fragment | complete | supplement
    pub content: CodesystemContentMode,
    /// Extension element for the 'content' primitive field. Contains metadata and extensions.
    pub _content: Option<Element>,
    /// Canonical URL of Code System this adds designations and properties to
    pub supplements: Option<StringType>,
    /// Extension element for the 'supplements' primitive field. Contains metadata and extensions.
    pub _supplements: Option<Element>,
    /// Total concepts in the code system
    pub count: Option<UnsignedIntType>,
    /// Extension element for the 'count' primitive field. Contains metadata and extensions.
    pub _count: Option<Element>,
    /// Filter that can be used in a value set
    pub filter: Option<Vec<CodeSystemFilter>>,
    /// Additional information supplied about each concept
    pub property: Option<Vec<CodeSystemProperty>>,
    /// Concepts in the code system
    pub concept: Option<Vec<CodeSystemConcept>>,
}
/// CodeSystemConcept nested structure for the 'designation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystemConceptDesignation {
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
    /// Details how this designation would be used
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
/// CodeSystem nested structure for the 'concept' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystemConcept {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Additional representations for the concept
    pub designation: Option<Vec<CodeSystemConceptDesignation>>,
    /// Property value for the concept
    pub property: Option<Vec<CodeSystemConceptProperty>>,
    /// Code that identifies concept
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Text to display to the user
    pub display: Option<StringType>,
    /// Extension element for the 'display' primitive field. Contains metadata and extensions.
    pub _display: Option<Element>,
    /// Formal definition
    pub definition: Option<StringType>,
    /// Extension element for the 'definition' primitive field. Contains metadata and extensions.
    pub _definition: Option<Element>,
    /// Child Concepts (is-a/contains/categorizes)
    pub concept: Option<Vec<StringType>>,
}
/// CodeSystem nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystemProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifies the property on the concepts, and when referred to in operations
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Formal identifier for the property
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// Why the property is defined, and/or what it conveys
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// code | Coding | string | integer | boolean | dateTime | decimal
    #[serde(rename = "type")]
    pub type_: ConceptPropertyType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
}
/// CodeSystemConcept nested structure for the 'property' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystemConceptProperty {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Reference to CodeSystem.property.code
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Value of the property for this concept (code)
    #[serde(rename = "valueCode")]
    pub value_code: StringType,
    /// Value of the property for this concept (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Value of the property for this concept (string)
    #[serde(rename = "valueString")]
    pub value_string: StringType,
    /// Value of the property for this concept (integer)
    #[serde(rename = "valueInteger")]
    pub value_integer: IntegerType,
    /// Value of the property for this concept (boolean)
    #[serde(rename = "valueBoolean")]
    pub value_boolean: BooleanType,
    /// Value of the property for this concept (dateTime)
    #[serde(rename = "valueDateTime")]
    pub value_date_time: DateTimeType,
    /// Value of the property for this concept (decimal)
    #[serde(rename = "valueDecimal")]
    pub value_decimal: DecimalType,
}
/// CodeSystem nested structure for the 'filter' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSystemFilter {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code that identifies the filter
    pub code: StringType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// How or why the filter is used
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | exists
    pub operator: Vec<FilterOperator>,
    /// Extension element for the 'operator' primitive field. Contains metadata and extensions.
    pub _operator: Option<Element>,
    /// What to use for the value
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
}

impl Default for CodeSystem {
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
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            case_sensitive: Default::default(),
            _case_sensitive: Default::default(),
            value_set: Default::default(),
            _value_set: Default::default(),
            hierarchy_meaning: Default::default(),
            _hierarchy_meaning: Default::default(),
            compositional: Default::default(),
            _compositional: Default::default(),
            version_needed: Default::default(),
            _version_needed: Default::default(),
            content: CodesystemContentMode::default(),
            _content: Default::default(),
            supplements: Default::default(),
            _supplements: Default::default(),
            count: Default::default(),
            _count: Default::default(),
            filter: Default::default(),
            property: Default::default(),
            concept: Default::default(),
        }
    }
}

impl Default for CodeSystemConceptDesignation {
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

impl Default for CodeSystemConcept {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            designation: Default::default(),
            property: Default::default(),
            code: StringType::default(),
            _code: Default::default(),
            display: Default::default(),
            _display: Default::default(),
            definition: Default::default(),
            _definition: Default::default(),
            concept: Default::default(),
        }
    }
}

impl Default for CodeSystemProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: StringType::default(),
            _code: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
        }
    }
}

impl Default for CodeSystemConceptProperty {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            value_code: Default::default(),
            value_coding: Default::default(),
            value_string: Default::default(),
            value_integer: Default::default(),
            value_boolean: Default::default(),
            value_date_time: Default::default(),
            value_decimal: Default::default(),
        }
    }
}

impl Default for CodeSystemFilter {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: StringType::default(),
            _code: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            operator: Vec::new(),
            _operator: Default::default(),
            value: StringType::default(),
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
    rh_foundation::Invariant::new("csd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("csd-1", rh_foundation::Severity::Error, "Within a code system definition, all the codes SHALL be unique", "concept.code.combine($this.descendants().concept.code).isDistinct()").with_xpath("count(distinct-values(descendant::f:concept/f:code/@value))=count(descendant::f:concept)"),
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
    rh_foundation::ElementBinding::new("CodeSystem.content", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/codesystem-content-mode|4.0.1").with_description("The extent of the content of the code system (the concepts and codes it defines) are represented in a code system resource."),
    rh_foundation::ElementBinding::new("CodeSystem.filter.operator", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/filter-operator|4.0.1").with_description("The kind of operation to perform as a part of a property based filter."),
    rh_foundation::ElementBinding::new("CodeSystem.hierarchyMeaning", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/codesystem-hierarchy-meaning|4.0.1").with_description("The meaning of the hierarchy of concepts in a code system."),
    rh_foundation::ElementBinding::new("CodeSystem.property.type", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/concept-property-type|4.0.1").with_description("The type of a property value."),
    rh_foundation::ElementBinding::new("CodeSystem.status", rh_foundation::BindingStrength::Required, "http://hl7.org/fhir/ValueSet/publication-status|4.0.1").with_description("The lifecycle status of an artifact."),
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("CodeSystem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.contained", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.identifier", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.contact", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.useContext", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.caseSensitive", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.valueSet", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.hierarchyMeaning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.compositional", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.versionNeeded", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.content", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.supplements", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.count", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.operator", 1, None),
            rh_foundation::ElementCardinality::new("CodeSystem.filter.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.uri", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.property.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.extension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.display", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.definition", 0, Some(1)),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.designation", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.designation.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.language",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.use",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.designation.value",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property", 0, None),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.property.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "CodeSystem.concept.property.value[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("CodeSystem.concept.concept", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CodeSystem {
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

impl crate::traits::resource::ResourceMutators for CodeSystem {
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

impl crate::traits::resource::ResourceExistence for CodeSystem {
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

impl crate::traits::domain_resource::DomainResourceAccessors for CodeSystem {
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

impl crate::traits::domain_resource::DomainResourceMutators for CodeSystem {
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

impl crate::traits::domain_resource::DomainResourceExistence for CodeSystem {
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

impl crate::traits::code_system::CodeSystemAccessors for CodeSystem {
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
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn case_sensitive(&self) -> Option<BooleanType> {
        self.case_sensitive
    }
    fn value_set(&self) -> Option<StringType> {
        self.value_set.clone()
    }
    fn hierarchy_meaning(&self) -> Option<CodesystemHierarchyMeaning> {
        self.hierarchy_meaning.clone()
    }
    fn compositional(&self) -> Option<BooleanType> {
        self.compositional
    }
    fn version_needed(&self) -> Option<BooleanType> {
        self.version_needed
    }
    fn content(&self) -> CodesystemContentMode {
        self.content.clone()
    }
    fn supplements(&self) -> Option<StringType> {
        self.supplements.clone()
    }
    fn count(&self) -> Option<UnsignedIntType> {
        self.count
    }
    fn filter(&self) -> &[CodeSystemFilter] {
        self.filter.as_deref().unwrap_or(&[])
    }
    fn property(&self) -> &[CodeSystemProperty] {
        self.property.as_deref().unwrap_or(&[])
    }
    fn concept(&self) -> &[CodeSystemConcept] {
        self.concept.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::code_system::CodeSystemMutators for CodeSystem {
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
    fn set_case_sensitive(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.case_sensitive = Some(value);
        resource
    }
    fn set_value_set(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.value_set = Some(value);
        resource
    }
    fn set_hierarchy_meaning(self, value: CodesystemHierarchyMeaning) -> Self {
        let mut resource = self.clone();
        resource.hierarchy_meaning = Some(value);
        resource
    }
    fn set_compositional(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.compositional = Some(value);
        resource
    }
    fn set_version_needed(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.version_needed = Some(value);
        resource
    }
    fn set_content(self, value: CodesystemContentMode) -> Self {
        let mut resource = self.clone();
        resource.content = value;
        resource
    }
    fn set_supplements(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.supplements = Some(value);
        resource
    }
    fn set_count(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.count = Some(value);
        resource
    }
    fn set_filter(self, value: Vec<CodeSystemFilter>) -> Self {
        let mut resource = self.clone();
        resource.filter = Some(value);
        resource
    }
    fn add_filter(self, item: CodeSystemFilter) -> Self {
        let mut resource = self.clone();
        resource.filter.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_property(self, value: Vec<CodeSystemProperty>) -> Self {
        let mut resource = self.clone();
        resource.property = Some(value);
        resource
    }
    fn add_property(self, item: CodeSystemProperty) -> Self {
        let mut resource = self.clone();
        resource.property.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_concept(self, value: Vec<CodeSystemConcept>) -> Self {
        let mut resource = self.clone();
        resource.concept = Some(value);
        resource
    }
    fn add_concept(self, item: CodeSystemConcept) -> Self {
        let mut resource = self.clone();
        resource.concept.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::code_system::CodeSystemExistence for CodeSystem {
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
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_case_sensitive(&self) -> bool {
        self.case_sensitive.is_some()
    }
    fn has_value_set(&self) -> bool {
        self.value_set.is_some()
    }
    fn has_hierarchy_meaning(&self) -> bool {
        self.hierarchy_meaning.is_some()
    }
    fn has_compositional(&self) -> bool {
        self.compositional.is_some()
    }
    fn has_version_needed(&self) -> bool {
        self.version_needed.is_some()
    }
    fn has_content(&self) -> bool {
        true
    }
    fn has_supplements(&self) -> bool {
        self.supplements.is_some()
    }
    fn has_count(&self) -> bool {
        self.count.is_some()
    }
    fn has_filter(&self) -> bool {
        self.filter.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_property(&self) -> bool {
        self.property.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_concept(&self) -> bool {
        self.concept.as_ref().is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for CodeSystem {
    fn resource_type(&self) -> &'static str {
        "CodeSystem"
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
        Some("http://hl7.org/fhir/StructureDefinition/CodeSystem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::code_system::{
    CodeSystemAccessors, CodeSystemExistence, CodeSystemMutators,
};
