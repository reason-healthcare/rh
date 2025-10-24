use crate::bindings::extension_context_type::ExtensionContextType;
use crate::bindings::fhir_version::FHIRVersion;
use crate::bindings::publication_status::PublicationStatus;
use crate::bindings::structure_definition_kind::StructureDefinitionKind;
use crate::bindings::type_derivation_rule::TypeDerivationRule;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::element_definition::ElementDefinition;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// StructureDefinition
///
/// A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: StructureDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this structure definition, represented as a URI (globally unique)
    pub url: StringType,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the structure definition
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the structure definition
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Name for this structure definition (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this structure definition (human friendly)
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
    /// Natural language description of the structure definition
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for structure definition (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this structure definition is defined
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Assist with indexing and finding
    ///
    /// Binding: extensible (Codes for the meaning of the defined structure (SNOMED CT and LOINC codes, as an example).)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/definition-use
    pub keyword: Option<Vec<Coding>>,
    /// FHIR Version this StructureDefinition targets
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Option<FHIRVersion>,
    /// Extension element for the 'fhirVersion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_fhirVersion")]
    pub _fhir_version: Option<Element>,
    /// External specification that the content is mapped to
    pub mapping: Option<Vec<StructureDefinitionMapping>>,
    /// primitive-type | complex-type | resource | logical
    pub kind: StructureDefinitionKind,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
    /// Whether the structure is abstract
    #[serde(rename = "abstract")]
    pub abstract_: BooleanType,
    /// Extension element for the 'abstract' primitive field. Contains metadata and extensions.
    pub _abstract: Option<Element>,
    /// If an extension, where it can be used in instances
    pub context: Option<Vec<StructureDefinitionContext>>,
    /// FHIRPath invariants - when the extension can be used
    #[serde(rename = "contextInvariant")]
    pub context_invariant: Option<Vec<StringType>>,
    /// Extension element for the 'contextInvariant' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contextInvariant")]
    pub _context_invariant: Option<Element>,
    /// Type defined or constrained by this structure
    ///
    /// Binding: extensible (Either a resource or a data type, including logical model types.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/defined-types
    #[serde(rename = "type")]
    pub type_: StringType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Definition that this type is constrained/specialized from
    #[serde(rename = "baseDefinition")]
    pub base_definition: Option<StringType>,
    /// Extension element for the 'baseDefinition' primitive field. Contains metadata and extensions.
    #[serde(rename = "_baseDefinition")]
    pub _base_definition: Option<Element>,
    /// specialization | constraint - How relates to base definition
    pub derivation: Option<TypeDerivationRule>,
    /// Extension element for the 'derivation' primitive field. Contains metadata and extensions.
    pub _derivation: Option<Element>,
    /// Snapshot view of the structure
    pub snapshot: Option<StructureDefinitionSnapshot>,
    /// Differential view of the structure
    pub differential: Option<StructureDefinitionDifferential>,
}
/// StructureDefinition nested structure for the 'differential' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitionDifferential {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: Vec<ElementDefinition>,
}
/// StructureDefinition nested structure for the 'snapshot' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitionSnapshot {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: Vec<ElementDefinition>,
}
/// StructureDefinition nested structure for the 'context' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitionContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// fhirpath | element | extension
    #[serde(rename = "type")]
    pub type_: ExtensionContextType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Where the extension can be used in instances
    pub expression: StringType,
    /// Extension element for the 'expression' primitive field. Contains metadata and extensions.
    pub _expression: Option<Element>,
}
/// StructureDefinition nested structure for the 'mapping' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureDefinitionMapping {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Internal id when this mapping is used
    pub identity: StringType,
    /// Extension element for the 'identity' primitive field. Contains metadata and extensions.
    pub _identity: Option<Element>,
    /// Identifies what this mapping refers to
    pub uri: Option<StringType>,
    /// Extension element for the 'uri' primitive field. Contains metadata and extensions.
    pub _uri: Option<Element>,
    /// Names what this mapping refers to
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Versions, Issues, Scope limitations etc.
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
}

impl Default for StructureDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: StringType::default(),
            _url: Default::default(),
            identifier: Default::default(),
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
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            keyword: Default::default(),
            fhir_version: Default::default(),
            _fhir_version: Default::default(),
            mapping: Default::default(),
            kind: StructureDefinitionKind::default(),
            _kind: Default::default(),
            abstract_: Default::default(),
            _abstract: Default::default(),
            context: Default::default(),
            context_invariant: Default::default(),
            _context_invariant: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            base_definition: Default::default(),
            _base_definition: Default::default(),
            derivation: Default::default(),
            _derivation: Default::default(),
            snapshot: Default::default(),
            differential: Default::default(),
        }
    }
}

impl Default for StructureDefinitionDifferential {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            element: Vec::new(),
        }
    }
}

impl Default for StructureDefinitionSnapshot {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            element: Vec::new(),
        }
    }
}

impl Default for StructureDefinitionContext {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            expression: StringType::default(),
            _expression: Default::default(),
        }
    }
}

impl Default for StructureDefinitionMapping {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identity: StringType::default(),
            _identity: Default::default(),
            uri: Default::default(),
            _uri: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
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
    rh_foundation::Invariant::new("sdf-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("sdf-1", rh_foundation::Severity::Error, "Element paths must be unique unless the structure is a constraint", "derivation = 'constraint' or snapshot.element.select(path).isDistinct()").with_xpath("(f:derivation/@value = 'constraint') or (count(f:snapshot/f:element) = count(distinct-values(f:snapshot/f:element/f:path/@value)))"),
    rh_foundation::Invariant::new("sdf-10", rh_foundation::Severity::Error, "provide either a binding reference or a description (or both)", "binding.empty() or binding.valueSet.exists() or binding.description.exists()").with_xpath("not(exists(f:binding)) or exists(f:binding/f:valueSet) or exists(f:binding/f:description)"),
    rh_foundation::Invariant::new("sdf-11", rh_foundation::Severity::Error, "If there's a type, its content must match the path name in the first element of a snapshot", "kind != 'logical' implies snapshot.empty() or snapshot.element.first().path = type").with_xpath("(f:kind/@value = 'logical') or not(exists(f:snapshot)) or (f:type/@value = f:snapshot/f:element[1]/f:path/@value)"),
    rh_foundation::Invariant::new("sdf-14", rh_foundation::Severity::Error, "All element definitions must have an id", "snapshot.element.all(id.exists()) and differential.element.all(id.exists())").with_xpath("count(*/f:element)=count(*/f:element/@id)"),
    rh_foundation::Invariant::new("sdf-15", rh_foundation::Severity::Error, "The first element in a snapshot has no type unless model is a logical model.", "kind!='logical' implies snapshot.element.first().type.empty()").with_xpath("f:kind/@value='logical' or not(f:snapshot/f:element[1]/f:type)"),
    rh_foundation::Invariant::new("sdf-15a", rh_foundation::Severity::Error, "If the first element in a differential has no \".\" in the path and it's not a logical model, it has no type", "(kind!='logical'  and differential.element.first().path.contains('.').not()) implies differential.element.first().type.empty()").with_xpath("f:kind/@value='logical' or not(f:differential/f:element[1][not(contains(f:path/@value, '.'))]/f:type)"),
    rh_foundation::Invariant::new("sdf-16", rh_foundation::Severity::Error, "All element definitions must have unique ids (snapshot)", "snapshot.element.all(id.exists()) and snapshot.element.id.trace('ids').isDistinct()").with_xpath("count(f:snapshot/f:element)=count(f:snapshot/f:element/@id) and (count(f:snapshot/f:element)=count(distinct-values(f:snapshot/f:element/@id)))"),
    rh_foundation::Invariant::new("sdf-17", rh_foundation::Severity::Error, "All element definitions must have unique ids (diff)", "differential.element.all(id.exists()) and differential.element.id.trace('ids').isDistinct()").with_xpath("count(f:differential/f:element)=count(f:differential/f:element/@id) and (count(f:differential/f:element)=count(distinct-values(f:differential/f:element/@id)))"),
    rh_foundation::Invariant::new("sdf-18", rh_foundation::Severity::Error, "Context Invariants can only be used for extensions", "contextInvariant.exists() implies type = 'Extension'").with_xpath("not(exists(f:contextInvariant)) or (f:type/@value = 'Extension')"),
    rh_foundation::Invariant::new("sdf-19", rh_foundation::Severity::Error, "FHIR Specification models only use FHIR defined types", "url.startsWith('http://hl7.org/fhir/StructureDefinition') implies (differential.element.type.code.all(matches('^[a-zA-Z0-9]+$') or matches('^http:\\\\/\\\\/hl7\\\\.org\\\\/fhirpath\\\\/System\\\\.[A-Z][A-Za-z]+$')) and snapshot.element.type.code.all(matches('^[a-zA-Z0-9\\\\.]+$') or matches('^http:\\\\/\\\\/hl7\\\\.org\\\\/fhirpath\\\\/System\\\\.[A-Z][A-Za-z]+$')))").with_xpath("not(starts-with(f:url/@value, 'http://hl7.org/fhir/StructureDefinition')) or count(f:differential/f:element/f:type/f:code[@value and not(matches(string(@value), '^[a-zA-Z0-9\\.]+$'))]|f:snapshot/f:element/f:type/f:code[@value and not(matches(string(@value), '^[a-zA-Z0-9]+$\\.'))]) =0"),
    rh_foundation::Invariant::new("sdf-2", rh_foundation::Severity::Error, "Must have at least a name or a uri (or both)", "name.exists() or uri.exists()").with_xpath("exists(f:uri) or exists(f:name)"),
    rh_foundation::Invariant::new("sdf-20", rh_foundation::Severity::Error, "No slicing on the root element", "element.where(path.contains('.').not()).slicing.empty()").with_xpath("not(f:element[1]/f:slicing)"),
    rh_foundation::Invariant::new("sdf-21", rh_foundation::Severity::Error, "Default values can only be specified on specializations", "differential.element.defaultValue.exists() implies (derivation = 'specialization')").with_xpath("not(exists(f:differential/f:element/*[starts-with(local-name(), 'defaultValue')])) or (f:derivation/@value = 'specialization')"),
    rh_foundation::Invariant::new("sdf-22", rh_foundation::Severity::Error, "FHIR Specification models never have default values", "url.startsWith('http://hl7.org/fhir/StructureDefinition') implies (snapshot.element.defaultValue.empty() and differential.element.defaultValue.empty())").with_xpath("not(starts-with(f:url/@value, 'http://hl7.org/fhir/StructureDefinition')) or (not(exists(f:snapshot/f:element/*[starts-with(local-name(), 'defaultValue')])) and not(exists(f:differential/f:element/*[starts-with(local-name(), 'defaultValue')])))"),
    rh_foundation::Invariant::new("sdf-23", rh_foundation::Severity::Error, "No slice name on root", "(snapshot | differential).element.all(path.contains('.').not() implies sliceName.empty())").with_xpath("count(*[self::snapshot or self::differential]/f:element[not(contains(f:path/@value, '.')) and f:sliceName])=0"),
    rh_foundation::Invariant::new("sdf-3", rh_foundation::Severity::Error, "Each element definition in a snapshot must have a formal definition and cardinalities", "element.all(definition.exists() and min.exists() and max.exists())").with_xpath("count(f:element) = count(f:element[exists(f:definition) and exists(f:min) and exists(f:max)])"),
    rh_foundation::Invariant::new("sdf-4", rh_foundation::Severity::Error, "If the structure is not abstract, then there SHALL be a baseDefinition", "abstract = true or baseDefinition.exists()").with_xpath("(f:abstract/@value=true()) or exists(f:baseDefinition)"),
    rh_foundation::Invariant::new("sdf-5", rh_foundation::Severity::Error, "If the structure defines an extension then the structure must have context information", "type != 'Extension' or derivation = 'specialization' or (context.exists())").with_xpath("not(f:type/@value = 'extension') or (f:derivation/@value = 'specialization') or (exists(f:context))"),
    rh_foundation::Invariant::new("sdf-6", rh_foundation::Severity::Error, "A structure must have either a differential, or a snapshot (or both)", "snapshot.exists() or differential.exists()").with_xpath("exists(f:snapshot) or exists(f:differential)"),
    rh_foundation::Invariant::new("sdf-8", rh_foundation::Severity::Error, "All snapshot elements must start with the StructureDefinition's specified type for non-logical models, or with the same type name for logical models", "(%resource.kind = 'logical' or element.first().path = %resource.type) and element.tail().all(path.startsWith(%resource.snapshot.element.first().path&'.'))").with_xpath("f:element[1]/f:path/@value=parent::f:StructureDefinition/f:type/@value and count(f:element[position()!=1])=count(f:element[position()!=1][starts-with(f:path/@value, concat(ancestor::f:StructureDefinition/f:type/@value, '.'))])"),
    rh_foundation::Invariant::new("sdf-8a", rh_foundation::Severity::Error, "In any differential, all the elements must start with the StructureDefinition's specified type for non-logical models, or with the same type name for logical models", "(%resource.kind = 'logical' or element.first().path.startsWith(%resource.type)) and (element.tail().empty() or element.tail().all(path.startsWith(%resource.differential.element.first().path.replaceMatches('\\\\..*','')&'.')))").with_xpath("count(f:element)=count(f:element[f:path/@value=ancestor::f:StructureDefinition/f:type/@value or starts-with(f:path/@value, concat(ancestor::f:StructureDefinition/f:type/@value, '.'))])"),
    rh_foundation::Invariant::new("sdf-8b", rh_foundation::Severity::Error, "All snapshot elements must have a base definition", "element.all(base.exists())").with_xpath("count(f:element) = count(f:element/f:base)"),
    rh_foundation::Invariant::new("sdf-9", rh_foundation::Severity::Error, "In any snapshot or differential, no label, code or requirements on an element without a \".\" in the path (e.g. the first element)", "children().element.where(path.contains('.').not()).label.empty() and children().element.where(path.contains('.').not()).code.empty() and children().element.where(path.contains('.').not()).requirements.empty()").with_xpath("not(exists(f:snapshot/f:element[not(contains(f:path/@value, '.')) and (f:label or f:code or f:requirements)])) and not(exists(f:differential/f:element[not(contains(f:path/@value, '.')) and (f:label or f:code or f:requirements)]))"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for StructureDefinition {
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

impl crate::traits::resource::ResourceMutators for StructureDefinition {
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

impl crate::traits::resource::ResourceExistence for StructureDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for StructureDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for StructureDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for StructureDefinition {
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

impl crate::traits::structure_definition::StructureDefinitionAccessors for StructureDefinition {
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
    fn keyword(&self) -> &[Coding] {
        self.keyword.as_deref().unwrap_or(&[])
    }
    fn fhir_version(&self) -> Option<FHIRVersion> {
        self.fhir_version.clone()
    }
    fn mapping(&self) -> &[StructureDefinitionMapping] {
        self.mapping.as_deref().unwrap_or(&[])
    }
    fn kind(&self) -> StructureDefinitionKind {
        self.kind.clone()
    }
    fn abstract_(&self) -> BooleanType {
        self.abstract_
    }
    fn context(&self) -> &[StructureDefinitionContext] {
        self.context.as_deref().unwrap_or(&[])
    }
    fn context_invariant(&self) -> &[StringType] {
        self.context_invariant.as_deref().unwrap_or(&[])
    }
    fn type_(&self) -> StringType {
        self.type_.clone()
    }
    fn base_definition(&self) -> Option<StringType> {
        self.base_definition.clone()
    }
    fn derivation(&self) -> Option<TypeDerivationRule> {
        self.derivation.clone()
    }
    fn snapshot(&self) -> Option<StructureDefinitionSnapshot> {
        self.snapshot.clone()
    }
    fn differential(&self) -> Option<StructureDefinitionDifferential> {
        self.differential.clone()
    }
}

impl crate::traits::structure_definition::StructureDefinitionMutators for StructureDefinition {
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
    fn set_keyword(self, value: Vec<Coding>) -> Self {
        let mut resource = self.clone();
        resource.keyword = Some(value);
        resource
    }
    fn add_keyword(self, item: Coding) -> Self {
        let mut resource = self.clone();
        resource.keyword.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_fhir_version(self, value: FHIRVersion) -> Self {
        let mut resource = self.clone();
        resource.fhir_version = Some(value);
        resource
    }
    fn set_mapping(self, value: Vec<StructureDefinitionMapping>) -> Self {
        let mut resource = self.clone();
        resource.mapping = Some(value);
        resource
    }
    fn add_mapping(self, item: StructureDefinitionMapping) -> Self {
        let mut resource = self.clone();
        resource.mapping.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_kind(self, value: StructureDefinitionKind) -> Self {
        let mut resource = self.clone();
        resource.kind = value;
        resource
    }
    fn set_abstract_(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.abstract_ = value;
        resource
    }
    fn set_context(self, value: Vec<StructureDefinitionContext>) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
        resource
    }
    fn add_context(self, item: StructureDefinitionContext) -> Self {
        let mut resource = self.clone();
        resource.context.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_context_invariant(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.context_invariant = Some(value);
        resource
    }
    fn add_context_invariant(self, item: String) -> Self {
        let mut resource = self.clone();
        resource
            .context_invariant
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_type_(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_base_definition(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base_definition = Some(value);
        resource
    }
    fn set_derivation(self, value: TypeDerivationRule) -> Self {
        let mut resource = self.clone();
        resource.derivation = Some(value);
        resource
    }
    fn set_snapshot(self, value: StructureDefinitionSnapshot) -> Self {
        let mut resource = self.clone();
        resource.snapshot = Some(value);
        resource
    }
    fn set_differential(self, value: StructureDefinitionDifferential) -> Self {
        let mut resource = self.clone();
        resource.differential = Some(value);
        resource
    }
}

impl crate::traits::structure_definition::StructureDefinitionExistence for StructureDefinition {
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
    fn has_keyword(&self) -> bool {
        self.keyword.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_fhir_version(&self) -> bool {
        self.fhir_version.is_some()
    }
    fn has_mapping(&self) -> bool {
        self.mapping.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_kind(&self) -> bool {
        true
    }
    fn has_abstract_(&self) -> bool {
        true
    }
    fn has_context(&self) -> bool {
        self.context.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_context_invariant(&self) -> bool {
        self.context_invariant
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_base_definition(&self) -> bool {
        self.base_definition.is_some()
    }
    fn has_derivation(&self) -> bool {
        self.derivation.is_some()
    }
    fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }
    fn has_differential(&self) -> bool {
        self.differential.is_some()
    }
}

impl crate::validation::ValidatableResource for StructureDefinition {
    fn resource_type(&self) -> &'static str {
        "StructureDefinition"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/StructureDefinition")
    }
}
