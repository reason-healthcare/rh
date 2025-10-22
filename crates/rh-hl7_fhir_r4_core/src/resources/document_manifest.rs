use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DocumentManifest
///
/// A collection of documents compiled for a purpose together with metadata that applies to the collection.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentManifest
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentManifest
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentManifest {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Unique Identifier for the set of documents
    #[serde(rename = "masterIdentifier")]
    pub master_identifier: Option<Identifier>,
    /// Other identifiers for the manifest
    pub identifier: Option<Vec<Identifier>>,
    /// current | superseded | entered-in-error
    pub status: DocumentReferenceStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Kind of document set
    ///
    /// Binding: example (The activity that caused the DocumentManifest to be created.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The subject of the set of documents
    pub subject: Option<Reference>,
    /// When this document manifest created
    pub created: Option<DateTimeType>,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// Who and/or what authored the DocumentManifest
    pub author: Option<Vec<Reference>>,
    /// Intended to get notified about this set of documents
    pub recipient: Option<Vec<Reference>>,
    /// The source system/application/software
    pub source: Option<StringType>,
    /// Extension element for the 'source' primitive field. Contains metadata and extensions.
    pub _source: Option<Element>,
    /// Human-readable description (title)
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Items in manifest
    pub content: Vec<Reference>,
    /// Related things
    pub related: Option<Vec<DocumentManifestRelated>>,
}
/// DocumentManifest nested structure for the 'related' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentManifestRelated {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Identifiers of things that are related
    pub identifier: Option<Identifier>,
    /// Related Resource
    #[serde(rename = "ref")]
    pub ref_: Option<Reference>,
}

impl Default for DocumentManifest {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            master_identifier: Default::default(),
            identifier: Default::default(),
            status: DocumentReferenceStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            subject: Default::default(),
            created: Default::default(),
            _created: Default::default(),
            author: Default::default(),
            recipient: Default::default(),
            source: Default::default(),
            _source: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            content: Vec::new(),
            related: Default::default(),
        }
    }
}

impl Default for DocumentManifestRelated {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            identifier: Default::default(),
            ref_: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DocumentManifest {
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

impl crate::traits::resource::ResourceMutators for DocumentManifest {
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

impl crate::traits::resource::ResourceExistence for DocumentManifest {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DocumentManifest {
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

impl crate::traits::domain_resource::DomainResourceMutators for DocumentManifest {
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

impl crate::traits::domain_resource::DomainResourceExistence for DocumentManifest {
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

impl crate::traits::document_manifest::DocumentManifestAccessors for DocumentManifest {
    fn master_identifier(&self) -> Option<Identifier> {
        self.master_identifier.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DocumentReferenceStatus {
        self.status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn created(&self) -> Option<DateTimeType> {
        self.created.clone()
    }
    fn author(&self) -> &[Reference] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn recipient(&self) -> &[Reference] {
        self.recipient.as_deref().unwrap_or(&[])
    }
    fn source(&self) -> Option<StringType> {
        self.source.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn content(&self) -> &[Reference] {
        &self.content
    }
    fn related(&self) -> &[DocumentManifestRelated] {
        self.related.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::document_manifest::DocumentManifestMutators for DocumentManifest {
    fn new() -> Self {
        Self::default()
    }
    fn set_master_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.master_identifier = Some(value);
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
    fn set_status(self, value: DocumentReferenceStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = Some(value);
        resource
    }
    fn set_author(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.author = Some(value);
        resource
    }
    fn add_author(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.author.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_recipient(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.recipient = Some(value);
        resource
    }
    fn add_recipient(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.recipient.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_source(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.source = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_content(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.content = value;
        resource
    }
    fn add_content(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.content.push(item);
        resource
    }
    fn set_related(self, value: Vec<DocumentManifestRelated>) -> Self {
        let mut resource = self.clone();
        resource.related = Some(value);
        resource
    }
    fn add_related(self, item: DocumentManifestRelated) -> Self {
        let mut resource = self.clone();
        resource.related.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::document_manifest::DocumentManifestExistence for DocumentManifest {
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
    fn has_master_identifier(&self) -> bool {
        self.master_identifier.is_some()
    }
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_created(&self) -> bool {
        self.created.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_recipient(&self) -> bool {
        self.recipient.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_source(&self) -> bool {
        self.source.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_content(&self) -> bool {
        !self.content.is_empty()
    }
    fn has_related(&self) -> bool {
        self.related.as_ref().is_some_and(|v| !v.is_empty())
    }
}
