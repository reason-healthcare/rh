use serde::{Deserialize, Serialize};
use crate::resources::domain_resource::DomainResource;
/// Patient Resource
///
/// Demographics and other administrative information about an individual.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Patient
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DomainResource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
}


// Trait implementations
impl crate::traits::resource::ResourceAccessors for Patient {
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


impl crate::traits::domain_resource::DomainResourceAccessors for Patient {
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
