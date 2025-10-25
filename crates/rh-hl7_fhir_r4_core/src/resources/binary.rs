use crate::bindings::mimetypes::Mimetypes;
use crate::datatypes::element::Element;
use crate::datatypes::reference::Reference;
use crate::primitives::base64binary::Base64BinaryType;
use crate::resources::resource::Resource;
use serde::{Deserialize, Serialize};
/// Binary
///
/// A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Binary
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Binary
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binary {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Resource,
    /// MimeType of the binary content
    #[serde(rename = "contentType")]
    pub content_type: Mimetypes,
    /// Extension element for the 'contentType' primitive field. Contains metadata and extensions.
    #[serde(rename = "_contentType")]
    pub _content_type: Option<Element>,
    /// Identifies another resource to use as proxy when enforcing access control
    #[serde(rename = "securityContext")]
    pub security_context: Option<Reference>,
    /// The actual content
    pub data: Option<Base64BinaryType>,
    /// Extension element for the 'data' primitive field. Contains metadata and extensions.
    pub _data: Option<Element>,
}

impl Default for Binary {
    fn default() -> Self {
        Self {
            base: Resource::default(),
            content_type: Mimetypes::default(),
            _content_type: Default::default(),
            security_context: Default::default(),
            data: Default::default(),
            _data: Default::default(),
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

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![rh_foundation::ElementBinding::new(
            "Binary.contentType",
            rh_foundation::BindingStrength::Required,
            "http://hl7.org/fhir/ValueSet/mimetypes|4.0.1",
        )
        .with_description("The mime type of an attachment. Any valid mime type is allowed.")]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Binary.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.contentType", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.securityContext", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Binary.data", 0, Some(1)),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Binary {
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

impl crate::traits::resource::ResourceMutators for Binary {
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

impl crate::traits::resource::ResourceExistence for Binary {
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

impl crate::traits::binary::BinaryAccessors for Binary {
    fn content_type(&self) -> Mimetypes {
        self.content_type.clone()
    }
    fn security_context(&self) -> Option<Reference> {
        self.security_context.clone()
    }
    fn data(&self) -> Option<Base64BinaryType> {
        self.data.clone()
    }
}

impl crate::traits::binary::BinaryMutators for Binary {
    fn new() -> Self {
        Self::default()
    }
    fn set_content_type(self, value: Mimetypes) -> Self {
        let mut resource = self.clone();
        resource.content_type = value;
        resource
    }
    fn set_security_context(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.security_context = Some(value);
        resource
    }
    fn set_data(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.data = Some(value);
        resource
    }
}

impl crate::traits::binary::BinaryExistence for Binary {
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
    fn has_content_type(&self) -> bool {
        true
    }
    fn has_security_context(&self) -> bool {
        self.security_context.is_some()
    }
    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

impl crate::validation::ValidatableResource for Binary {
    fn resource_type(&self) -> &'static str {
        "Binary"
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
        Some("http://hl7.org/fhir/StructureDefinition/Binary")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::binary::{BinaryAccessors, BinaryExistence, BinaryMutators};
