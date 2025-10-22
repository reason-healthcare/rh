use crate::bindings::composition_attestation_mode::CompositionAttestationMode;
use crate::bindings::composition_status::CompositionStatus;
use crate::bindings::document_relationship_type::DocumentRelationshipType;
use crate::bindings::list_mode::ListMode;
use crate::bindings::v3confidentiality_classification::V3ConfidentialityClassification;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::narrative::Narrative;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Composition
///
/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Composition
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Composition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Version-independent identifier for the Composition
    pub identifier: Option<Identifier>,
    /// preliminary | final | amended | entered-in-error
    pub status: CompositionStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// Kind of composition (LOINC if possible)
    ///
    /// Binding: preferred (Type of a composition.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/doc-typecodes
    #[serde(rename = "type")]
    pub type_: CodeableConcept,
    /// Categorization of Composition
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
    /// Who and/or what the composition is about
    pub subject: Option<Reference>,
    /// Context of the Composition
    pub encounter: Option<Reference>,
    /// Composition editing time
    pub date: DateTimeType,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Who and/or what authored the composition
    pub author: Vec<Reference>,
    /// Human Readable name/title
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// As defined by affinity domain
    pub confidentiality: Option<V3ConfidentialityClassification>,
    /// Extension element for the 'confidentiality' primitive field. Contains metadata and extensions.
    pub _confidentiality: Option<Element>,
    /// Attests to accuracy of composition
    pub attester: Option<Vec<CompositionAttester>>,
    /// Organization which maintains the composition
    pub custodian: Option<Reference>,
    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<CompositionRelatesto>>,
    /// The clinical service(s) being documented
    pub event: Option<Vec<CompositionEvent>>,
    /// Composition is broken into sections
    pub section: Option<Vec<CompositionSection>>,
}
/// Composition nested structure for the 'event' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionEvent {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Code(s) that apply to the event being documented
    ///
    /// Binding: example (This list of codes represents the main clinical acts being documented.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActCode
    pub code: Option<Vec<CodeableConcept>>,
    /// The period covered by the documentation
    pub period: Option<Period>,
    /// The event(s) being documented
    pub detail: Option<Vec<Reference>>,
}
/// Composition nested structure for the 'attester' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionAttester {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// personal | professional | legal | official
    pub mode: CompositionAttestationMode,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// When the composition was attested
    pub time: Option<DateTimeType>,
    /// Extension element for the 'time' primitive field. Contains metadata and extensions.
    pub _time: Option<Element>,
    /// Who attested the composition
    pub party: Option<Reference>,
}
/// Composition nested structure for the 'section' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionSection {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for section (e.g. for ToC)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Classification of section (recommended)
    ///
    /// Binding: example (Classification of a section of a composition/document.)
    ///
    /// Available values:
    /// - `10154-3`
    /// - `10157-6`
    /// - `10160-0`
    /// - `10164-2`
    /// - `10183-2`
    /// - `10184-0`
    /// - `10187-3`
    /// - `10210-3`
    /// - `10216-0`
    /// - `10218-6`
    /// - ... and 47 more values
    pub code: Option<CodeableConcept>,
    /// Who and/or what authored the section
    pub author: Option<Vec<Reference>>,
    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<Reference>,
    /// Text summary of the section, for human interpretation
    pub text: Option<Narrative>,
    /// working | snapshot | changes
    pub mode: Option<ListMode>,
    /// Extension element for the 'mode' primitive field. Contains metadata and extensions.
    pub _mode: Option<Element>,
    /// Order of section entries
    ///
    /// Binding: preferred (What order applies to the items in the entry.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-order
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,
    /// A reference to data that supports this section
    pub entry: Option<Vec<Reference>>,
    /// Why the section is empty
    ///
    /// Binding: preferred (If a section is empty, why it is empty.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/list-empty-reason
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,
    /// Nested Section
    pub section: Option<Vec<StringType>>,
}
/// Composition nested structure for the 'relatesTo' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRelatesto {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// replaces | transforms | signs | appends
    pub code: DocumentRelationshipType,
    /// Extension element for the 'code' primitive field. Contains metadata and extensions.
    pub _code: Option<Element>,
    /// Target of the relationship (Identifier)
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: Identifier,
    /// Target of the relationship (Reference)
    #[serde(rename = "targetReference")]
    pub target_reference: Reference,
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: CompositionStatus::default(),
            _status: Default::default(),
            type_: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            encounter: Default::default(),
            date: DateTimeType::default(),
            _date: Default::default(),
            author: Vec::new(),
            title: StringType::default(),
            _title: Default::default(),
            confidentiality: Default::default(),
            _confidentiality: Default::default(),
            attester: Default::default(),
            custodian: Default::default(),
            relates_to: Default::default(),
            event: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for CompositionEvent {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            period: Default::default(),
            detail: Default::default(),
        }
    }
}

impl Default for CompositionAttester {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            mode: CompositionAttestationMode::default(),
            _mode: Default::default(),
            time: Default::default(),
            _time: Default::default(),
            party: Default::default(),
        }
    }
}

impl Default for CompositionSection {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            code: Default::default(),
            author: Default::default(),
            focus: Default::default(),
            text: Default::default(),
            mode: Default::default(),
            _mode: Default::default(),
            ordered_by: Default::default(),
            entry: Default::default(),
            empty_reason: Default::default(),
            section: Default::default(),
        }
    }
}

impl Default for CompositionRelatesto {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            code: Default::default(),
            _code: Default::default(),
            target_identifier: Default::default(),
            target_reference: Default::default(),
        }
    }
}

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Composition {
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

impl crate::traits::resource::ResourceMutators for Composition {
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

impl crate::traits::resource::ResourceExistence for Composition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Composition {
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

impl crate::traits::domain_resource::DomainResourceMutators for Composition {
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

impl crate::traits::domain_resource::DomainResourceExistence for Composition {
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

impl crate::traits::composition::CompositionAccessors for Composition {
    fn identifier(&self) -> Option<Identifier> {
        self.identifier.clone()
    }
    fn status(&self) -> CompositionStatus {
        self.status.clone()
    }
    fn type_(&self) -> CodeableConcept {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn encounter(&self) -> Option<Reference> {
        self.encounter.clone()
    }
    fn date(&self) -> DateTimeType {
        self.date.clone()
    }
    fn author(&self) -> &[Reference] {
        &self.author
    }
    fn title(&self) -> StringType {
        self.title.clone()
    }
    fn confidentiality(&self) -> Option<V3ConfidentialityClassification> {
        self.confidentiality.clone()
    }
    fn attester(&self) -> &[CompositionAttester] {
        self.attester.as_deref().unwrap_or(&[])
    }
    fn custodian(&self) -> Option<Reference> {
        self.custodian.clone()
    }
    fn relates_to(&self) -> &[CompositionRelatesto] {
        self.relates_to.as_deref().unwrap_or(&[])
    }
    fn event(&self) -> &[CompositionEvent] {
        self.event.as_deref().unwrap_or(&[])
    }
    fn section(&self) -> &[CompositionSection] {
        self.section.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::composition::CompositionMutators for Composition {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn set_status(self, value: CompositionStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_type_(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
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
    fn set_encounter(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.encounter = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = value;
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
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = value;
        resource
    }
    fn set_confidentiality(self, value: V3ConfidentialityClassification) -> Self {
        let mut resource = self.clone();
        resource.confidentiality = Some(value);
        resource
    }
    fn set_attester(self, value: Vec<CompositionAttester>) -> Self {
        let mut resource = self.clone();
        resource.attester = Some(value);
        resource
    }
    fn add_attester(self, item: CompositionAttester) -> Self {
        let mut resource = self.clone();
        resource.attester.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_custodian(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.custodian = Some(value);
        resource
    }
    fn set_relates_to(self, value: Vec<CompositionRelatesto>) -> Self {
        let mut resource = self.clone();
        resource.relates_to = Some(value);
        resource
    }
    fn add_relates_to(self, item: CompositionRelatesto) -> Self {
        let mut resource = self.clone();
        resource.relates_to.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_event(self, value: Vec<CompositionEvent>) -> Self {
        let mut resource = self.clone();
        resource.event = Some(value);
        resource
    }
    fn add_event(self, item: CompositionEvent) -> Self {
        let mut resource = self.clone();
        resource.event.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_section(self, value: Vec<CompositionSection>) -> Self {
        let mut resource = self.clone();
        resource.section = Some(value);
        resource
    }
    fn add_section(self, item: CompositionSection) -> Self {
        let mut resource = self.clone();
        resource.section.get_or_insert_with(Vec::new).push(item);
        resource
    }
}

impl crate::traits::composition::CompositionExistence for Composition {
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
    fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        self.category.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }
    fn has_date(&self) -> bool {
        true
    }
    fn has_author(&self) -> bool {
        !self.author.is_empty()
    }
    fn has_title(&self) -> bool {
        true
    }
    fn has_confidentiality(&self) -> bool {
        self.confidentiality.is_some()
    }
    fn has_attester(&self) -> bool {
        self.attester.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_custodian(&self) -> bool {
        self.custodian.is_some()
    }
    fn has_relates_to(&self) -> bool {
        self.relates_to.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_event(&self) -> bool {
        self.event.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_section(&self) -> bool {
        self.section.as_ref().is_some_and(|v| !v.is_empty())
    }
}
