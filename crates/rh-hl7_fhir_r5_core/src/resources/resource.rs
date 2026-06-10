use crate::datatypes::base::Base;
use crate::datatypes::element::Element;
use crate::datatypes::meta::Meta;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Resource
///
/// This is the base resource type for everything.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Resource
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Base,
    /// Logical id of this artifact
    pub id: Option<StringType>,
    /// Metadata about the resource
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[serde(rename = "implicitRules")]
    pub implicit_rules: Option<StringType>,
    /// Extension element for the 'implicitRules' primitive field. Contains metadata and extensions.
    #[serde(rename = "_implicitRules")]
    pub _implicit_rules: Option<Element>,
    /// Language of the resource content
    pub language: Option<StringType>,
    /// Extension element for the 'language' primitive field. Contains metadata and extensions.
    pub _language: Option<Element>,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            base: Base::default(),
            id: Default::default(),
            meta: Default::default(),
            implicit_rules: Default::default(),
            _implicit_rules: Default::default(),
            language: Default::default(),
            _language: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::Invariant::new(
            "ele-1",
            rh_foundation::Severity::Error,
            "All FHIR elements must have a @value or children",
            "hasValue() or (children().count() > id.count())",
        )]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Resource.language",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
        )
        .with_description("IETF language tag for a human language")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Resource.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Resource.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Resource.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Resource.language", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Resource {
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for Resource {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for Resource {
    fn has_id(&self) -> bool {
        self.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.language.is_some()
    }
}

impl crate::validation::ValidatableResource for Resource {
    fn resource_type(&self) -> &'static str {
        "Resource"
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
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::resource::{ResourceAccessors, ResourceExistence, ResourceMutators};
