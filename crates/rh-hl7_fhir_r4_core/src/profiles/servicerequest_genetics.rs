use crate::datatypes::extension::Extension;
use crate::resources::service_request::ServiceRequest;
use serde::{Deserialize, Serialize};
/// ServiceRequest-Genetics
///
/// Describes how the ServiceRequest resource is used to for genetics
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-genetics
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: ServiceRequest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/ServiceRequest
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicerequestGenetics {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: ServiceRequest,
}
/// Item
///
/// The specific diagnostic investigations that are requested as part of this request. Sometimes, there can only be one item per request, but in most contexts, more than one investigation can be requested.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/servicerequest-geneticsItem
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicerequestGeneticsItem {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ServicerequestGenetics {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for ServicerequestGenetics {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ServicerequestGenetics {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::servicerequest_genetics::ServicerequestGeneticsMutators
    for ServicerequestGenetics
{
    fn new() -> Self {
        Self::default()
    }
}
