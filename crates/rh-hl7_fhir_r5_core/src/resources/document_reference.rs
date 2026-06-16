use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_reference_status::DocumentReferenceStatus;
use crate::datatypes::attachment::Attachment;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// DocumentReference
///
/// A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: DocumentReference
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifiers for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// An explicitly assigned identifer of a variation of the content in the DocumentReference
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// Procedure that caused this media to be created
    #[serde(rename = "basedOn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<Reference>,
    /// current | superseded | entered-in-error
    pub status: DocumentReferenceStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    #[serde(rename = "docStatus")]
    pub doc_status: Option<CompositionStatus>,
    /// Extension element for the 'docStatus' primitive field. Contains metadata and extensions.
    #[serde(rename = "_docStatus")]
    pub _doc_status: Option<Element>,
    /// Imaging modality used
    ///
    /// Binding: extensible (Type of acquired data in the instance.)
    ///
    /// ValueSet: http://dicom.nema.org/medical/dicom/current/output/chtml/part16/sect_CID_33.html
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality: Vec<CodeableConcept>,
    /// Kind of document (LOINC if possible)
    ///
    /// Binding: preferred (Precise type of clinical document.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/doc-typecodes
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// Categorization of document
    ///
    /// Binding: example (High-level kind of document at a macro level.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/referenced-item-category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// Who/what is the subject of the document
    pub subject: Option<Reference>,
    /// Context of the document content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<Reference>,
    /// Main clinical acts documented
    ///
    /// Binding: example (This list of codes represents the main clinical acts being documented.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CodeableReference>,
    /// Body part included
    ///
    /// Binding: example (SNOMED CT Body site concepts)
    ///
    /// Available values:
    /// - `53075003`: Distal phalanx of hallux
    /// - `76986006`: Distal phalanx of second toe
    /// - `65258003`: Distal phalanx of third toe
    /// - `54333003`: Distal phalanx of fourth toe
    /// - `10770001`: Distal phalanx of fifth toe
    /// - `363670009`: Interphalangeal joint structure of great toe
    /// - `371216008`: Distal interphalangeal joint of second toe
    /// - `371219001`: Distal interphalangeal joint of third toe
    /// - `371205001`: Distal interphalangeal joint of fourth toe
    /// - `371203008`: Distal interphalangeal joint of fifth toe
    /// - ... and 30 more values
    #[serde(rename = "bodySite")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<CodeableReference>,
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
    /// Time of service that is being documented
    pub period: Option<Period>,
    /// When this document reference was created
    pub date: Option<InstantType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who and/or what authored the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<Reference>,
    /// Attests to accuracy of the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<DocumentReferenceAttester>,
    /// Organization which maintains the document
    pub custodian: Option<Reference>,
    /// Relationships to other documents
    #[serde(rename = "relatesTo")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<DocumentReferenceRelatesto>,
    /// Human-readable description
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Document security-tags
    ///
    /// Binding: example (Example Security Labels from the Healthcare Privacy and Security Classification System.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/security-label-examples
    #[serde(rename = "securityLabel")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<CodeableConcept>,
    /// Document referenced
    pub content: Vec<DocumentReferenceContent>,
}
/// DocumentReference nested structure for the 'attester' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceAttester {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// personal | professional | legal | official
    ///
    /// Binding: preferred (The way in which a person authenticated a document.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/composition-attestation-mode
    pub mode: CodeableConcept,
    /// When the document was attested
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// Who attested the document
    pub party: Option<Reference>,
}
/// DocumentReference nested structure for the 'content' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceContent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Content profile rules for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<DocumentReferenceContentProfile>,
    /// Where to access the document
    pub attachment: Attachment,
}
/// DocumentReferenceContent nested structure for the 'profile' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceContentProfile {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code|uri|canonical (Coding)
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,
    /// Code|uri|canonical (uri)
    #[serde(rename = "valueUri")]
    pub value_uri: StringType,
    /// Code|uri|canonical (canonical)
    #[serde(rename = "valueCanonical")]
    pub value_canonical: StringType,
}
/// DocumentReference nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReferenceRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The relationship type with another document
    ///
    /// Binding: extensible (The type of relationship between the documents.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/document-relationship-type
    pub code: CodeableConcept,
    /// Target of the relationship
    pub target: Reference,
}

impl Default for DocumentReference {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            based_on: Default::default(),
            status: DocumentReferenceStatus::default(),
            _status: Default::default(),
            doc_status: Default::default(),
            _doc_status: Default::default(),
            modality: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            context: Default::default(),
            event: Default::default(),
            body_site: Default::default(),
            facility_type: Default::default(),
            practice_setting: Default::default(),
            period: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            author: Default::default(),
            attester: Default::default(),
            custodian: Default::default(),
            relates_to: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            security_label: Default::default(),
            content: Vec::new(),
        }
    }
}

impl Default for DocumentReferenceAttester {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: CodeableConcept::default(),
            time: Default::default(),
            _time: Default::default(),
            party: Default::default(),
        }
    }
}

impl Default for DocumentReferenceContent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            profile: Default::default(),
            attachment: Attachment::default(),
        }
    }
}

impl Default for DocumentReferenceContentProfile {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            value_coding: Default::default(),
            value_uri: Default::default(),
            value_canonical: Default::default(),
        }
    }
}

impl Default for DocumentReferenceRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            target: Default::default(),
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
    rh_foundation::Invariant::new("docRef-1", rh_foundation::Severity::Warning, "facilityType SHALL only be present if context is not an encounter", "facilityType.empty() or context.where(resolve() is Encounter).empty()"),
    rh_foundation::Invariant::new("docRef-2", rh_foundation::Severity::Warning, "practiceSetting SHALL only be present if context is not present", "practiceSetting.empty() or context.where(resolve() is Encounter).empty()"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "DocumentReference.docStatus",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/composition-status|5.0.0",
            )
            .with_description("Status of the underlying document."),
            rh_foundation::ElementBinding::new(
                "DocumentReference.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "DocumentReference.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/document-reference-status|5.0.0",
            )
            .with_description("The status of the document reference."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("DocumentReference.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.contained", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.extension", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.identifier", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.docStatus", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.modality", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.type", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.category", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.context", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.event", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.bodySite", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.facilityType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.practiceSetting", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.author", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.attester", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.attester.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.attester.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.attester.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DocumentReference.attester.mode", 1, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.attester.time", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.attester.party", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.custodian", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.relatesTo", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.relatesTo.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.relatesTo.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.relatesTo.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("DocumentReference.relatesTo.code", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.relatesTo.target",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DocumentReference.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.securityLabel", 0, None),
            rh_foundation::ElementCardinality::new("DocumentReference.content", 1, None),
            rh_foundation::ElementCardinality::new("DocumentReference.content.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("DocumentReference.content.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.attachment",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("DocumentReference.content.profile", 0, None),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.profile.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.profile.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.profile.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "DocumentReference.content.profile.value[x]",
                1,
                Some(1),
            ),
        ]
    });

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
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
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
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for DocumentReference {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::document_reference::DocumentReferenceAccessors for DocumentReference {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_slice()
    }
    fn status(&self) -> DocumentReferenceStatus {
        self.status.clone()
    }
    fn doc_status(&self) -> Option<CompositionStatus> {
        self.doc_status.clone()
    }
    fn modality(&self) -> &[CodeableConcept] {
        self.modality.as_slice()
    }
    fn type_(&self) -> Option<CodeableConcept> {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn context(&self) -> &[Reference] {
        self.context.as_slice()
    }
    fn event(&self) -> &[CodeableReference] {
        self.event.as_slice()
    }
    fn body_site(&self) -> &[CodeableReference] {
        self.body_site.as_slice()
    }
    fn facility_type(&self) -> Option<CodeableConcept> {
        self.facility_type.clone()
    }
    fn practice_setting(&self) -> Option<CodeableConcept> {
        self.practice_setting.clone()
    }
    fn period(&self) -> Option<Period> {
        self.period.clone()
    }
    fn date(&self) -> Option<InstantType> {
        self.date.clone()
    }
    fn author(&self) -> &[Reference] {
        self.author.as_slice()
    }
    fn attester(&self) -> &[DocumentReferenceAttester] {
        self.attester.as_slice()
    }
    fn custodian(&self) -> Option<Reference> {
        self.custodian.clone()
    }
    fn relates_to(&self) -> &[DocumentReferenceRelatesto] {
        self.relates_to.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn security_label(&self) -> &[CodeableConcept] {
        self.security_label.as_slice()
    }
    fn content(&self) -> &[DocumentReferenceContent] {
        &self.content
    }
}

impl crate::traits::document_reference::DocumentReferenceMutators for DocumentReference {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = value;
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.push(item);
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
    fn set_modality(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.modality = value;
        resource
    }
    fn add_modality(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.modality.push(item);
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = Some(value);
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_context(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.context = value;
        resource
    }
    fn add_context(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.context.push(item);
        resource
    }
    fn set_event(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.event = value;
        resource
    }
    fn add_event(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.event.push(item);
        resource
    }
    fn set_body_site(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.body_site = value;
        resource
    }
    fn add_body_site(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.body_site.push(item);
        resource
    }
    fn set_facility_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.facility_type = Some(value);
        resource
    }
    fn set_practice_setting(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.practice_setting = Some(value);
        resource
    }
    fn set_period(self, value: Period) -> Self {
        let mut resource = self.clone();
        resource.period = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_author(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.author = value;
        resource
    }
    fn add_author(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.author.push(item);
        resource
    }
    fn set_attester(self, value: Vec<DocumentReferenceAttester>) -> Self {
        let mut resource = self.clone();
        resource.attester = value;
        resource
    }
    fn add_attester(self, item: DocumentReferenceAttester) -> Self {
        let mut resource = self.clone();
        resource.attester.push(item);
        resource
    }
    fn set_custodian(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.custodian = Some(value);
        resource
    }
    fn set_relates_to(self, value: Vec<DocumentReferenceRelatesto>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = value;
        resource
    }
    fn add_relates_to(self, item: DocumentReferenceRelatesto) -> Self {
        let mut resource = self.clone();
        resource.relates_to.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_security_label(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.security_label = value;
        resource
    }
    fn add_security_label(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.security_label.push(item);
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
}

impl crate::traits::document_reference::DocumentReferenceExistence for DocumentReference {
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_based_on(&self) -> bool {
        !self.based_on.is_empty()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_doc_status(&self) -> bool {
        self.doc_status.is_some()
    }
    fn has_modality(&self) -> bool {
        !self.modality.is_empty()
    }
    fn has_type_(&self) -> bool {
        self.type_.is_some()
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_context(&self) -> bool {
        !self.context.is_empty()
    }
    fn has_event(&self) -> bool {
        !self.event.is_empty()
    }
    fn has_body_site(&self) -> bool {
        !self.body_site.is_empty()
    }
    fn has_facility_type(&self) -> bool {
        self.facility_type.is_some()
    }
    fn has_practice_setting(&self) -> bool {
        self.practice_setting.is_some()
    }
    fn has_period(&self) -> bool {
        self.period.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_attester(&self) -> bool {
        !self.attester.is_empty()
    }
    fn has_custodian(&self) -> bool {
        self.custodian.is_some()
    }
    fn has_relates_to(&self) -> bool {
        !self.relates_to.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_security_label(&self) -> bool {
        !self.security_label.is_empty()
    }
    fn has_content(&self) -> bool {
        !self.content.is_empty()
    }
}

impl crate::validation::ValidatableResource for DocumentReference {
    fn resource_type(&self) -> &'static str {
        "DocumentReference"
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
        Some("http://hl7.org/fhir/StructureDefinition/DocumentReference")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::document_reference::{
    DocumentReferenceAccessors, DocumentReferenceExistence, DocumentReferenceMutators,
};
