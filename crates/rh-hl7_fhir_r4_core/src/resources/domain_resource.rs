use crate::datatypes::extension::Extension;
use crate::datatypes::narrative::Narrative;
use crate::resources::resource::Resource;
use serde::{Deserialize, Serialize};
/// DomainResource
///
/// A resource that includes narrative, extensions, and contained resources.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DomainResource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Resource,
    /// Text summary of the resource, for human interpretation
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,
}

impl Default for DomainResource {
    fn default() -> Self {
        Self {
            base: Resource::default(),
            text: Default::default(),
            contained: Default::default(),
            extension: Default::default(),
            modifier_extension: Default::default(),
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
]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DomainResource.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DomainResource.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DomainResource.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DomainResource.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DomainResource.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DomainResource.contained", 0, None),
            rh_foundation::ElementCardinality::new("DomainResource.extension", 0, None),
            rh_foundation::ElementCardinality::new("DomainResource.modifierExtension", 0, None),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DomainResource {
    fn id(&self) -> Option<String> {
        self.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for DomainResource {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for DomainResource {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for DomainResource {
    fn text(&self) -> Option<Narrative> {
        self.text.clone()
    }
    fn contained(&self) -> &[Resource] {
        self.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[Extension] {
        self.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[Extension] {
        self.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for DomainResource {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: Narrative) -> Self {
        let mut resource = self.clone();
        resource.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<Resource>) -> Self {
        let mut resource = self.clone();
        resource.contained = Some(value);
        resource
    }
    fn add_contained(self, item: Resource) -> Self {
        let mut resource = self.clone();
        resource.contained.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_extension(self, value: Vec<Extension>) -> Self {
        let mut resource = self.clone();
        resource.extension = Some(value);
        resource
    }
    fn add_extension(self, item: Extension) -> Self {
        let mut resource = self.clone();
        resource.extension.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<Extension>) -> Self {
        let mut resource = self.clone();
        resource.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: Extension) -> Self {
        let mut resource = self.clone();
        resource
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for DomainResource {
    fn has_id(&self) -> bool {
        self.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.language.is_some()
    }
    fn has_text(&self) -> bool {
        self.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.contained.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.extension.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.modifier_extension
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for DomainResource {
    fn resource_type(&self) -> &'static str {
        "DomainResource"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::domain_resource::{
    DomainResourceAccessors, DomainResourceExistence, DomainResourceMutators,
};
