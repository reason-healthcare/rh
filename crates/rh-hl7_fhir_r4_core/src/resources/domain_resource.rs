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
