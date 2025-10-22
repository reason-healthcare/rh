use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::bindings::document_relationship_type::DocumentRelationshipType;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DocumentReference
///
/// A reference to a document of any kind for any purpose. Provides metadata about the document so that the document can be discovered and managed. The scope of a document is any seralized object with a mime-type, so includes formal patient centric documents (CDA), cliical notes, scanned paper, and non-patient specific documents like policy text.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Master Version Specific Identifier
    #[serde(rename = "masterIdentifier")]
    pub master_identifier: Option<Identifier>,
    /// Other identifiers for the document
    pub identifier: Option<Vec<Identifier>>,
    /// current | superseded | entered-in-error
    pub status: DocumentReferenceStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// preliminary | final | amended | entered-in-error
    #[serde(rename = "docStatus")]
    pub doc_status: Option<CompositionStatus>,
    /// Extension element for the 'docStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_docStatus")]
    pub _doc_status: Option<Element>,
    /// Kind of document (LOINC if possible)
    ///
    /// Binding: preferred (Precise type of clinical document.)
    ///
    /// Available values:
    /// - `55107-7`: Addendum Document
    /// - `74155-3`: ADHD action plan
    /// - `51851-4`: Administrative note
    /// - `67851-6`: Admission evaluation note
    /// - `34744-3`: Nurse Admission evaluation note
    /// - `34873-0`: Surgery Admission evaluation note
    /// - `68552-9`: Emergency medicine Emergency department Admission evaluation note
    /// - `67852-4`: Hospital Admission evaluation note
    /// - `68471-2`: Cardiology Hospital Admission evaluation note
    /// - `68483-7`: Cardiology Medical student Hospital Admission evaluation note
    /// - ... and 6391 more values
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Categorization of document
    ///
    /// Binding: example (High-level kind of a clinical document at a macro level.)
    ///
    /// Available values:
    /// - `11369-6`: History of Immunization
    /// - `11485-0`: Anesthesia records
    /// - `11486-8`: Chemotherapy records
    /// - `11488-4`: Consult Note
    /// - `11506-3`: Provider-unspecified progress note
    /// - `11543-6`: Nursery records
    /// - `15508-5`: Labor and delivery records
    /// - `18726-0`: Radiology studies (set)
    /// - `18761-7`: Provider-unspecified transfer summary
    /// - `18842-5`: Discharge summary
    /// - ... and 35 more values
    pub category: Option<Vec<CodeableConcept>>,
    /// Who/what is the subject of the document
    pub subject: Option<Reference>,
    /// When this document reference was created
    pub date: Option<InstantType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who and/or what authored the document
    pub author: Option<Vec<Reference>>,
    /// Who/what authenticated the document
    pub authenticator: Option<Reference>,
    /// Organization which maintains the document
    pub custodian: Option<Reference>,
    /// Relationships to other documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<DocumentReferenceRelatesto>>,
    /// Human-readable description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Document security-tags
    ///
    /// Binding: extensible (Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-labels
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<CodeableConcept>>,
    /// Document referenced
    pub content: Vec<DocumentReferenceContent>,
    /// Clinical context of document
    pub context: Option<DocumentReferenceContext>,
}
/// DocumentReference nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// replaces | transforms | signs | appends
    pub code: DocumentRelationshipType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Target of the relationship
    pub target: Reference,
}
/// DocumentReference nested structure for the 'context' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceContext {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Context of the document  content
    pub encounter: Option<Vec<Reference>>,
    /// Main clinical acts documented
    ///
    /// Binding: example (This list of codes represents the main clinical acts being documented.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    pub event: Option<Vec<CodeableConcept>>,
    /// Time of service that is being documented
    pub period: Option<Period>,
    /// Kind of facility where patient was seen
    ///
    /// Binding: example (XDS Facility Type.)
    ///
    /// Available values:
    /// - `82242000`: Hospital-children's
    /// - `225732001`: Hospital-community
    /// - `79993009`: Hospital-government
    /// - `32074000`: Hospital-long term care
    /// - `4322002`: Hospital-military field
    /// - `224687002`: Hospital-prison
    /// - `62480006`: Hospital-psychiatric
    /// - `80522000`: Hospital-rehabilitation
    /// - `36125001`: Hospital-trauma center
    /// - `48311003`: Hospital-Veterans' Administration
    /// - ... and 69 more values
    #[serde(rename = "facilityType")]
    pub facility_type: Option<CodeableConcept>,
    /// Additional details about where the content was created (e.g. clinical specialty)
    ///
    /// Binding: example (Additional details about where the content was created (e.g. clinical specialty).)
    ///
    /// Available values:
    /// - `408467006`: Adult mental illness
    /// - `394577000`: Anesthetics
    /// - `394578005`: Audiological medicine
    /// - `421661004`: Blood banking and transfusion medicine
    /// - `408462000`: Burns care
    /// - `394579002`: Cardiology
    /// - `394804000`: Clinical cytogenetics and molecular genetics
    /// - `394580004`: Clinical genetics
    /// - `394803006`: Clinical hematology
    /// - `408480009`: Clinical immunology
    /// - ... and 107 more values
    #[serde(rename = "practiceSetting")]
    pub practice_setting: Option<CodeableConcept>,
    /// Patient demographics from source
    #[serde(rename = "sourcePatientInfo")]
    pub source_patient_info: Option<Reference>,
    /// Related identifiers or resources
    pub related: Option<Vec<Reference>>,
}
/// DocumentReference nested structure for the 'content' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceContent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Where to access the document
    pub attachment: Attachment,
    /// Format/content rules for the document
    ///
    /// Binding: preferred (Document Format Codes.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/formatcodes
    pub format: Option<Coding>,
}

impl Default for DocumentReference {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            master_identifier: Default::default(),
            identifier: Default::default(),
            status: DocumentReferenceStatus::default(),
            _status: Default::default(),
            doc_status: Default::default(),
            _doc_status: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            author: Default::default(),
            authenticator: Default::default(),
            custodian: Default::default(),
            relates_to: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            security_label: Default::default(),
            content: Vec::new(),
            context: Default::default(),
        }
    }
}

impl Default for DocumentReferenceRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            target: Default::default(),
        }
    }
}

impl Default for DocumentReferenceContext {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            encounter: Default::default(),
            event: Default::default(),
            period: Default::default(),
            facility_type: Default::default(),
            practice_setting: Default::default(),
            source_patient_info: Default::default(),
            related: Default::default(),
        }
    }
}

impl Default for DocumentReferenceContent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            attachment: Attachment::default(),
            format: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for DocumentReference {
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

impl crate::traits::resource::ResourceMutators for DocumentReference {
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

impl crate::traits::resource::ResourceExistence for DocumentReference {
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

impl crate::traits::domain_resource::DomainResourceAccessors for DocumentReference {
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

impl crate::traits::domain_resource::DomainResourceMutators for DocumentReference {
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

impl crate::traits::domain_resource::DomainResourceExistence for DocumentReference {
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

impl crate::traits::document_reference::DocumentReferenceAccessors for DocumentReference {
    fn master_identifier(&self) -> Option<Identifier> {
        self.master_identifier.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> DocumentReferenceStatus {
        self.status.clone()
    }
    fn doc_status(&self) -> Option<CompositionStatus> {
        self.doc_status.clone()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn date(&self) -> Option<InstantType> {
        self.date.clone()
    }
    fn author(&self) -> &[Reference] {
        self.author.as_deref().unwrap_or(&[])
    }
    fn authenticator(&self) -> Option<Reference> {
        self.authenticator.clone()
    }
    fn custodian(&self) -> Option<Reference> {
        self.custodian.clone()
    }
    fn relates_to(&self) -> &[DocumentReferenceRelatesto] {
        self.relates_to.as_deref().unwrap_or(&[])
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn security_label(&self) -> &[CodeableConcept] {
        self.security_label.as_deref().unwrap_or(&[])
    }
    fn content(&self) -> &[DocumentReferenceContent] {
        &self.content
    }
    fn context(&self) -> Option<DocumentReferenceContext> {
        self.context.clone()
    }
}

impl crate::traits::document_reference::DocumentReferenceMutators for DocumentReference {
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
    fn set_doc_status(self, value: CompositionStatus) -> Self {
        let mut resource = self.clone();
        resource.doc_status = Some(value);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = Some(value);
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
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
    fn set_authenticator(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.authenticator = Some(value);
        resource
    }
    fn set_custodian(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.custodian = Some(value);
        resource
    }
    fn set_relates_to(self, value: Vec<DocumentReferenceRelatesto>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = Some(value);
        resource
    }
    fn add_relates_to(self, item: DocumentReferenceRelatesto) -> Self {
        let mut resource = self.clone();
        resource.relates_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_security_label(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.security_label = Some(value);
        resource
    }
    fn add_security_label(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .security_label
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_content(self, value: Vec<DocumentReferenceContent>) -> Self {
        let mut resource = self.clone();
        resource.content = value;
        resource
    }
    fn add_content(self, item: DocumentReferenceContent) -> Self {
        let mut resource = self.clone();
        resource.content.push(item);
        resource
    }
    fn set_context(self, value: DocumentReferenceContext) -> Self {
        let mut resource = self.clone();
        resource.context = Some(value);
        resource
    }
}

impl crate::traits::document_reference::DocumentReferenceExistence for DocumentReference {
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
    fn has_doc_status(&self) -> bool {
        self.doc_status.is_some()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_authenticator(&self) -> bool {
        self.authenticator.is_some()
    }
    fn has_custodian(&self) -> bool {
        self.custodian.is_some()
    }
    fn has_relates_to(&self) -> bool {
        self.relates_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_security_label(&self) -> bool {
        self.security_label.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_content(&self) -> bool {
        !self.content.is_empty()
    }
    fn has_context(&self) -> bool {
        self.context.is_some()
    }
}
