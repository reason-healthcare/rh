use crate::bindings::namingsystem_identifier_type::NamingsystemIdentifierType;
use crate::bindings::namingsystem_type::NamingsystemType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::period::Period;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// NamingSystem
///
/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a "System" used within the Identifier and Coding data types.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/NamingSystem
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: NamingSystem
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingSystem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Name for this naming system (computer friendly)
    pub name: StringType,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// codesystem | identifier | root
    pub kind: NamingsystemType,
    /// Extension element for the 'kind' primitive field. Contains metadata and extensions.
    pub _kind: Option<Element>,
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
    /// Who maintains system namespace?
    pub responsible: Option<StringType>,
    /// Extension element for the 'responsible' primitive field. Contains metadata and extensions.
    pub _responsible: Option<Element>,
    /// e.g. driver,  provider,  patient, bank etc.
    ///
    /// Binding: extensible (A coded type for an identifier that can be used to determine which identifier to use for a specific purpose.)
    ///
    /// Available values:
    /// - `DL`
    /// - `PPN`
    /// - `BRN`
    /// - `MR`
    /// - `MCN`
    /// - `EN`
    /// - `TAX`
    /// - `NIIP`
    /// - `PRN`
    /// - `MD`
    /// - ... and 8 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Natural language description of the naming system
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for naming system (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// How/where is it used
    pub usage: Option<StringType>,
    /// Extension element for the 'usage' primitive field. Contains metadata and extensions.
    pub _usage: Option<Element>,
    /// Unique identifiers used for system
    #[serde(rename = "uniqueId")]
    pub unique_id: Vec<NamingSystemUniqueid>,
}
/// NamingSystem nested structure for the 'uniqueId' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingSystemUniqueid {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// oid | uuid | uri | other
    #[serde(rename = "type")]
    pub type_: NamingsystemIdentifierType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// The unique identifier
    pub value: StringType,
    /// Extension element for the 'value' primitive field. Contains metadata and extensions.
    pub _value: Option<Element>,
    /// Is this the id that should be used for this type
    pub preferred: Option<BooleanType>,
    /// Extension element for the 'preferred' primitive field. Contains metadata and extensions.
    pub _preferred: Option<Element>,
    /// Notes about identifier usage
    pub comment: Option<StringType>,
    /// Extension element for the 'comment' primitive field. Contains metadata and extensions.
    pub _comment: Option<Element>,
    /// When is identifier valid?
    pub period: Option<Period>,
}

impl Default for NamingSystem {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            name: StringType::default(),
            _name: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            kind: NamingsystemType::default(),
            _kind: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            responsible: Default::default(),
            _responsible: Default::default(),
            type_: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            usage: Default::default(),
            _usage: Default::default(),
            unique_id: Vec::new(),
        }
    }
}

impl Default for NamingSystemUniqueid {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            _type: Default::default(),
            value: Default::default(),
            _value: Default::default(),
            preferred: Default::default(),
            _preferred: Default::default(),
            comment: Default::default(),
            _comment: Default::default(),
            period: Default::default(),
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
    rh_foundation::Invariant::new("nsd-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("nsd-1", rh_foundation::Severity::Error, "Root systems cannot have uuid identifiers", "kind != 'root' or uniqueId.all(type != 'uuid')").with_xpath("not(f:kind/@value='root' and f:uniqueId/f:type/@value='uuid')"),
    rh_foundation::Invariant::new("nsd-2", rh_foundation::Severity::Error, "Can't have more than one preferred identifier for a type", "uniqueId.where(preferred = true).select(type).isDistinct()").with_xpath("not(exists(for $type in distinct-values(f:uniqueId/f:type/@value) return if (count(f:uniqueId[f:type/@value=$type and f:preferred/@value=true()])>1) then $type else ()))"),
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
                "NamingSystem.kind",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-type|4.0.1",
            )
            .with_description("Identifies the purpose of the naming system."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|4.0.1",
            )
            .with_description("The lifecycle status of an artifact."),
            rh_foundation::ElementBinding::new(
                "NamingSystem.uniqueId.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/namingsystem-identifier-type|4.0.1",
            )
            .with_description(
                "Identifies the style of unique identifier used to identify a namespace.",
            ),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("NamingSystem.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contained", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.extension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.name", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.kind", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.date", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.contact", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.responsible", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.useContext", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("NamingSystem.usage", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId", 1, None),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "NamingSystem.uniqueId.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.value", 1, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.preferred", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.comment", 0, Some(1)),
            rh_foundation::ElementCardinality::new("NamingSystem.uniqueId.period", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for NamingSystem {
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

impl crate::traits::resource::ResourceMutators for NamingSystem {
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

impl crate::traits::resource::ResourceExistence for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceAccessors for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceMutators for NamingSystem {
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

impl crate::traits::domain_resource::DomainResourceExistence for NamingSystem {
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

impl crate::traits::naming_system::NamingSystemAccessors for NamingSystem {
    fn name(&self) -> StringType {
        self.name.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn kind(&self) -> NamingsystemType {
        self.kind.clone()
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
    fn responsible(&self) -> Option<StringType> {
        self.responsible.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
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
    fn usage(&self) -> Option<StringType> {
        self.usage.clone()
    }
    fn unique_id(&self) -> &[NamingSystemUniqueid] {
        &self.unique_id
    }
}

impl crate::traits::naming_system::NamingSystemMutators for NamingSystem {
    fn new() -> Self {
        Self::default()
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = value;
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_kind(self, value: NamingsystemType) -> Self {
        let mut resource = self.clone();
        resource.kind = value;
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
    fn set_responsible(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.responsible = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
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
    fn set_usage(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.usage = Some(value);
        resource
    }
    fn set_unique_id(self, value: Vec<NamingSystemUniqueid>) -> Self {
        let mut resource = self.clone();
        resource.unique_id = value;
        resource
    }
    fn add_unique_id(self, item: NamingSystemUniqueid) -> Self {
        let mut resource = self.clone();
        resource.unique_id.push(item);
        resource
    }
}

impl crate::traits::naming_system::NamingSystemExistence for NamingSystem {
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
    fn has_name(&self) -> bool {
        true
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_kind(&self) -> bool {
        true
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
    fn has_responsible(&self) -> bool {
        self.responsible.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
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
    fn has_usage(&self) -> bool {
        self.usage.is_some()
    }
    fn has_unique_id(&self) -> bool {
        !self.unique_id.is_empty()
    }
}

impl crate::validation::ValidatableResource for NamingSystem {
    fn resource_type(&self) -> &'static str {
        "NamingSystem"
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
        Some("http://hl7.org/fhir/StructureDefinition/NamingSystem")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::naming_system::{
    NamingSystemAccessors, NamingSystemExistence, NamingSystemMutators,
};
