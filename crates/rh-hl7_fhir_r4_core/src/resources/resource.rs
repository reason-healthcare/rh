use crate::datatypes::element::Element;
use crate::datatypes::extension::Extension;
use crate::datatypes::meta::Meta;
use crate::primitives::string::StringType;
use serde::{Deserialize, Serialize};
/// Resource
///
/// This is the base resource type for everything.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Resource
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
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
}
/// Last Review Date
///
/// The date on which the asset content was last reviewed. Review happens periodically after that, but doesn't change the original approval date.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-lastReviewDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLastReviewDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// pertains to goal
///
/// Indicates that the resource is related to either the measurement, achievement or progress towards the referenced goal.  For example, a Procedure to exercise pertainsToGoal of losing weight.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-pertainsToGoal
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePertainsToGoal {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}
/// Approval Date
///
/// The date on which the asset content was approved by the publisher. Approval happens once when the content is officially approved for usage.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/resource-approvalDate
/// - Version: 4.0.1
/// - Kind: complex-type
/// - Type: Extension
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceApprovalDate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Extension,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            id: Default::default(),
            meta: Default::default(),
            implicit_rules: Default::default(),
            _implicit_rules: Default::default(),
            language: Default::default(),
            _language: Default::default(),
        }
    }
}

impl Default for ResourceLastReviewDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ResourcePertainsToGoal {
    fn default() -> Self {
        Self {
            base: Extension::default(),
        }
    }
}

impl Default for ResourceApprovalDate {
    fn default() -> Self {
        Self {
            base: Extension::default(),
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
        )
        .with_xpath("@value|f:*|h:div")]
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
}
