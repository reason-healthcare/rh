use crate::resources::plan_definition::PlanDefinition;
use serde::{Deserialize, Serialize};
/// Computable PlanDefinition
///
/// Defines a computable PlanDefinition that specifies a single library and requires all expressions referenced from the PlanDefinition to be definitions in that single library
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/computableplandefinition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: PlanDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/PlanDefinition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Computableplandefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: PlanDefinition,
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Computableplandefinition {
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

impl crate::traits::resource::ResourceMutators for Computableplandefinition {
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

impl crate::traits::resource::ResourceExistence for Computableplandefinition {
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

impl crate::traits::computableplandefinition::ComputableplandefinitionMutators
    for Computableplandefinition
{
    fn new() -> Self {
        Self::default()
    }
}
